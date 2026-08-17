use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};


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
    pub uptime: u64,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64
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
    let user = std::env::var("USER").unwrap_or_else(|_| "Unknown".to_string());
    let terminal = std::env::var("TERM").unwrap_or_else(|_| "Unknown Terminal".to_string());
    let raw_shell = std::env::var("SHELL").unwrap_or_else(|_| "Unknown Shell".to_string());
    let shell = std::path::Path::new(&raw_shell)
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or(raw_shell);

    let cursor = std::env::var("XCURSOR_THEME").unwrap_or_else(|_| "Unknown Cursor".to_string());
    let cursor_size = std::env::var("XCURSOR_SIZE").unwrap_or_else(|_| "Unknown Cursor Size".to_string());

    let wm = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "Unknown Desktop".to_string());

    let os_version = System::os_version().unwrap_or_else(|| "Unknown Version".to_string());
    let core_count = System::physical_core_count().unwrap_or(0);
    let thread_count = cpus.len();
    let cpu = cpus.first().map_or_else(|| "Unknown CPU".to_string(), |c| c.brand().to_string());

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
        kernel_version,
        uptime: System::uptime(),
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap()
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
