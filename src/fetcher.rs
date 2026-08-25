use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct DiskInfo {
    pub mount_point: String,
    pub used_space: u64,
    pub total_space: u64,
    pub file_system: String,
}

#[must_use]
pub fn gather_system_info() -> SystemData {
    let refresh_kind = RefreshKind::nothing()
        .with_cpu(CpuRefreshKind::everything())
        .with_memory(MemoryRefreshKind::everything());
    let sys = System::new_with_specifics(refresh_kind);
    let cpus = sys.cpus();

    let os_name = System::name().unwrap_or_else(|| "Unknown OS".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown Kernel".to_string());
    let host_name = System::host_name().unwrap_or_else(|| "Unknown Host".to_string());
    let user = env::var("USER").unwrap_or_else(|_| "Unknown".to_string());
    let terminal = env::var("TERM").unwrap_or_else(|_| "Unknown Terminal".to_string());
    
    let raw_shell = env::var("SHELL").unwrap_or_else(|_| "Unknown Shell".to_string());
    let shell = Path::new(&raw_shell)
        .file_name()
        .map_or_else(|| raw_shell.clone(), |name| name.to_string_lossy().to_string());

    let cursor = env::var("XCURSOR_THEME").unwrap_or_else(|_| "Unknown Cursor".to_string());
    let cursor_size = env::var("XCURSOR_SIZE").unwrap_or_else(|_| "Unknown Cursor Size".to_string());
    let wm = env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "Unknown Desktop".to_string());

    let os_version = System::os_version().unwrap_or_else(|| "Unknown Version".to_string());
    let core_count = System::physical_core_count().unwrap_or(0);
    let thread_count = cpus.len();
    let cpu = cpus
        .first()
        .map_or_else(|| "Unknown CPU".to_string(), |c| c.brand().to_string());

    let cpu_freq = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
        .ok()
        .and_then(|s| s.trim().parse::<f64>().ok())
        .map_or(0.0, |khz| khz / 1_000_000.0);

    let vendor = fs::read_to_string("/sys/class/dmi/id/board_vendor")
        .map_or_else(|_| String::new(), |s| s.trim().to_string());
        
    let motherboard = fs::read_to_string("/sys/class/dmi/id/product_name")
        .map_or_else(|_| "Unknown Board".to_string(), |s| s.trim().to_string());

    let disks = Disks::new_with_refreshed_list();
    let mut disk_list = Vec::new();
    let mut seen_devices = HashSet::new();

    for disk in disks.list() {
        let fs_type = disk.file_system().to_string_lossy();
        let mount = disk.mount_point().to_string_lossy();
        let dev_name = disk.name().to_string_lossy().to_string();

        // Skip virtual filesystems, boot partitions, or tiny partitions (< 1 GiB)
        if fs_type == "overlay"
            || fs_type == "tmpfs"
            || mount.starts_with("/boot")
            || disk.total_space() < 1_073_741_824
        {
            continue;
        }

        // Record each physical device partition only once
        if seen_devices.insert(dev_name) {
            let total = disk.total_space();
            let available = disk.available_space();
            let occupied = total.saturating_sub(available); // Replaced 'Total' with occupied to
                                                            // pass clippy linting
            disk_list.push(DiskInfo {
                mount_point: mount.into_owned(),
                used_space: occupied,
                total_space: total,
                file_system: fs_type.into_owned(),
            });
        }
    }

    SystemData {
        os_name,
        os_version,
        host_name,
        user,
        terminal,
        shell,
        cursor,
        cursor_size,
        wm,
        cpu_arch: System::cpu_arch(),
        core_count,
        thread_count,
        cpu,
        cpu_freq,
        motherboard,
        vendor,
        kernel_version,
        uptime: System::uptime(),
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
        disks: disk_list,
    }
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
