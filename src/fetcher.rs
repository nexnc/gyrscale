use std::env;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::mem::MaybeUninit;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct SystemData {
    pub os_name: String,
    pub os_version: String,
    pub host_name: String,
    pub user: String,
    pub terminal: String,
    pub shell: String,
    pub wm: String,
    pub cursor: String,
    pub cursor_size: String,
    pub kernel_version: String,
    pub cpu_arch: String,
    pub cpu: String,
    pub core_count: usize,
    pub thread_count: usize,
    pub cpu_freq: f64,
    pub motherboard: String,
    pub vendor: String,
    pub uptime: u64,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub disks: Vec<DiskInfo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiskInfo {
    pub mount_point: String,
    pub used_space: u64,
    pub total_space: u64,
    pub file_system: String,
}

#[inline]
fn read_sysfs_trimmed(path: &str) -> Option<String> {
    let mut buf = Vec::with_capacity(128);
    let mut f = File::open(path).ok()?;
    let n = f.read_to_end(&mut buf).ok()?;
    let s = std::str::from_utf8(&buf[..n]).ok()?;
    Some(s.trim().to_string())
}

#[must_use]
pub fn gather_system_info() -> SystemData {
    let (os_name, os_version) = parse_os_release();
    let (host_name, kernel_version, cpu_arch) = parse_uname();
    let (cpu, core_count, thread_count) = parse_cpuinfo();
    let (uptime, total_memory, used_memory, total_swap, used_swap) = parse_meminfo_and_sysinfo();
    let disks = parse_disks();

    let user = env::var("USER").unwrap_or_else(|_| "Unknown".to_string());
    let terminal = env::var("TERM_PROGRAM")
        .or_else(|_| env::var("TERM"))
        .unwrap_or_else(|_| "Unknown Terminal".to_string());

    let raw_shell = env::var("SHELL").unwrap_or_else(|_| "Unknown Shell".to_string());
    let shell = Path::new(&raw_shell)
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or(raw_shell);

    let cursor = env::var("XCURSOR_THEME").unwrap_or_else(|_| "Unknown Cursor".to_string());
    let cursor_size = env::var("XCURSOR_SIZE").unwrap_or_else(|_| "Unknown Cursor Size".to_string());
    let wm = env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .unwrap_or_else(|_| "Unknown Desktop".to_string());

    let cpu_freq = read_sysfs_trimmed("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
        .or_else(|| read_sysfs_trimmed("/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq"))
        .and_then(|s| s.parse::<f64>().ok())
        .map_or(0.0, |khz| khz / 1_000_000.0);

    let vendor = read_sysfs_trimmed("/sys/class/dmi/id/board_vendor").unwrap_or_default();

    let motherboard = read_sysfs_trimmed("/sys/class/dmi/id/product_name")
        .or_else(|| read_sysfs_trimmed("/sys/class/dmi/id/board_name"))
        .unwrap_or_else(|| "Unknown Board".to_string());

    SystemData {
        os_name,
        os_version,
        host_name,
        user,
        terminal,
        shell,
        wm,
        cursor,
        cursor_size,
        kernel_version,
        cpu_arch,
        cpu,
        core_count,
        thread_count,
        cpu_freq,
        motherboard,
        vendor,
        uptime,
        total_memory,
        used_memory,
        total_swap,
        used_swap,
        disks
    }
}

fn parse_uname() -> (String, String, String) {
    let mut uts: libc::utsname = unsafe { MaybeUninit::zeroed().assume_init() };
    if unsafe { libc::uname(&raw mut uts) } == 0 {
        let hostname = unsafe { CStr::from_ptr(uts.nodename.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        let release = unsafe { CStr::from_ptr(uts.release.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        (hostname, release, env::consts::ARCH.to_string())
    } else {
        ("Unknown Host".to_string(), "Unknown Kernel".to_string(), env::consts::ARCH.to_string())
    }
}

fn parse_meminfo_and_sysinfo() -> (u64, u64, u64, u64, u64) {
    let mut uptime = 0u64;
    let mut info: libc::sysinfo = unsafe { MaybeUninit::zeroed().assume_init() };
    if unsafe { libc::sysinfo(&raw mut info) } == 0 {
        uptime = info.uptime.cast_unsigned();
    }

    let mut buf = Vec::with_capacity(2048);
    let n = File::open("/proc/meminfo")
        .and_then(|mut f| f.read_to_end(&mut buf))
        .unwrap_or(0);
    let content = std::str::from_utf8(&buf[..n]).unwrap_or_default();

    let mut total_mem_kb = 0u64;
    let mut avail_mem_kb = 0u64;
    let mut total_swap_kb = 0u64;
    let mut free_swap_kb = 0u64;

    for line in content.lines() {
        let mut parts = line.split_whitespace();
        let key = parts.next().unwrap_or_default();
        let val = parts.next().and_then(|v| v.parse::<u64>().ok()).unwrap_or(0);

        match key {
            "MemTotal:" => total_mem_kb = val,
            "MemAvailable:" => avail_mem_kb = val,
            "SwapTotal:" => total_swap_kb = val,
            "SwapFree:" => free_swap_kb = val,
            _ => {}
        }
    }

    let total_memory = total_mem_kb * 1024;
    let used_memory = total_mem_kb.saturating_sub(avail_mem_kb) * 1024;
    let total_swap = total_swap_kb * 1024;
    let used_swap = total_swap_kb.saturating_sub(free_swap_kb) * 1024;

    (uptime, total_memory, used_memory, total_swap, used_swap)
}

fn parse_os_release() -> (String, String) {
    let mut buf = Vec::with_capacity(1024);
    let n = File::open("/etc/os-release")
        .or_else(|_| File::open("/usr/lib/os-release"))
        .and_then(|mut f| f.read_to_end(&mut buf))
        .unwrap_or(0);

    let content = std::str::from_utf8(&buf[..n]).unwrap_or_default();
    let mut name = String::new();
    let mut version_id = String::new();
    let mut codename = String::new();

    for line in content.lines() {
        if let Some(val) = line.strip_prefix("NAME=") {
            name = val.trim_matches('"').to_string();
        } else if let Some(val) = line.strip_prefix("VERSION_ID=") {
            version_id = val.trim_matches('"').to_string();
        } else if let Some(val) = line.strip_prefix("VERSION_CODENAME=") {
            codename = val.trim_matches('"').to_string();
        }
    }

    if name.is_empty() {
        name = "Linux".to_string();
    }

    let version = if codename.is_empty() {
        version_id
    } else if version_id.is_empty() {
        codename
    } else {
        format!("{version_id} ({codename})")
    };

    (name, version)
}

fn parse_cpuinfo() -> (String, usize, usize) {
    let mut buf = Vec::with_capacity(8192);
    let mut cpu_name = String::new();
    let mut thread_count = 0;
    
    // Stack-allocated array to track unique physical core IDs up to 256 cores 
    // to avoid HashSet heap allocations.
    let mut core_ids = [false; 256];

    if let Ok(mut file) = File::open("/proc/cpuinfo")
        && let Ok(n) = file.read_to_end(&mut buf) {
            let s = std::str::from_utf8(&buf[..n]).unwrap_or_default();
            for line in s.lines() {
                if line.starts_with("processor") {
                    thread_count += 1;
                } else if cpu_name.is_empty()
                    && (line.starts_with("model name")
                        || line.starts_with("Hardware")
                        || line.starts_with("Model"))
                {
                    if let Some((_, val)) = line.split_once(':') {
                        cpu_name = val.trim().to_string();
                    }
                } else if line.starts_with("core id")
                    && let Some((_, val)) = line.split_once(':')
                        && let Ok(id) = val.trim().parse::<usize>()
                            && id < 256 { 
                                core_ids[id] = true;
                            }
            }
        }

    if cpu_name.is_empty() {
        cpu_name = "Unknown CPU".to_string();
    }

    let physical_cores = core_ids.iter().filter(|&&b| b).count();
    let core_count = if physical_cores == 0 { thread_count } else { physical_cores };
    
    // Fallback if /proc/cpuinfo completely fails
    let thread_count = if thread_count == 0 { 
        unsafe { usize::try_from(libc::sysconf(libc::_SC_NPROCESSORS_ONLN)) } 
    } else { 
        Ok(thread_count)
    };

    (cpu_name, core_count, thread_count.expect("NA"))
}

fn parse_disks() -> Vec<DiskInfo> {
    let mut buf = Vec::with_capacity(8192);
    let Ok(mut file) = File::open("/proc/self/mounts") else {
        return Vec::new();
    };

    let n = file.read_to_end(&mut buf).unwrap_or(0);
    let content = std::str::from_utf8(&buf[..n]).unwrap_or_default();
    let mut disk_list = Vec::with_capacity(4);

    for line in content.lines() {
        let mut parts = line.split_whitespace();
        let Some(device) = parts.next() else { continue };
        let Some(mount_point) = parts.next() else { continue };
        let Some(fs_type) = parts.next() else { continue };

        if !device.starts_with('/')
            || fs_type == "tmpfs"
            || fs_type == "overlay"
            || fs_type == "devtmpfs"
            || fs_type == "squashfs"
            || mount_point.starts_with("/boot")
            || mount_point.starts_with("/nix/store")
            || mount_point.starts_with("/var/lib/containers")
            || mount_point.starts_with("/var/lib/docker")
        {
            continue;
        }

        if disk_list.iter().any(|d: &DiskInfo| d.mount_point == mount_point) {
            continue;
        }

        if let Ok(c_mount) = CString::new(mount_point) {
            unsafe {
                let mut stat: libc::statvfs = MaybeUninit::zeroed().assume_init();
                if libc::statvfs(c_mount.as_ptr(), &raw mut stat) == 0 {
                    let total = (stat.f_blocks).saturating_mul(stat.f_frsize);
                    let available = (stat.f_bavail).saturating_mul(stat.f_frsize);

                    if total < 1_073_741_824 {
                        continue;
                    }

                    disk_list.push(DiskInfo {
                        mount_point: mount_point.to_string(),
                        used_space: total.saturating_sub(available),
                        total_space: total,
                        file_system: fs_type.to_string(),
                    });
                }
            }
        }
    }

    disk_list
}

impl DiskInfo {
    #[must_use]
    pub fn format_disk(&self) -> String {
        const GIB: u64 = 1024 * 1024 * 1024;

        let total_whole = self.total_space / GIB;
        let total_frac = ((self.total_space % GIB).saturating_mul(100)) / GIB;

        let used_whole = self.used_space / GIB;
        let used_frac = ((self.used_space % GIB).saturating_mul(100)) / GIB;

        let percent = (self.used_space.saturating_mul(100))
            .checked_div(self.total_space)
            .unwrap_or(0);

        let mount = &self.mount_point;
        let fs = &self.file_system;

        format!(
            "Disk ({mount}): {used_whole}.{used_frac:02} GiB / {total_whole}.{total_frac:02} GiB ({percent}%) - {fs}"
        )
    }
}

impl SystemData {
    #[must_use]
    pub fn format_uptime(&self) -> String {
        let hours = self.uptime / 3600;
        let minutes = (self.uptime % 3600) / 60;
        format!("{hours}h {minutes}m")
    }

    #[must_use]
    pub fn format_memory(&self) -> String {
        const GIB: u64 = 1_073_741_824;
        let total_gib = self.total_memory / GIB;
        let total_mem = (self.total_memory % GIB) * 100 / GIB;

        let used_gib = self.used_memory / GIB;
        let used_mem = (self.used_memory % GIB) * 100 / GIB;
        format!("{used_gib}.{used_mem:02} GiB / {total_gib}.{total_mem:02} GiB")
    }

    #[must_use]
    pub fn format_swap(&self) -> String {
        const GIB: u64 = 1_073_741_824;
        let total_gib = self.total_swap / GIB;
        let total_mem = (self.total_swap % GIB) * 100 / GIB;

        let used_gib = self.used_swap / GIB;
        let used_mem = (self.used_swap % GIB) * 100 / GIB;
        format!("{used_gib}.{used_mem:02} GiB / {total_gib}.{total_mem:02} GiB")
    }
}
