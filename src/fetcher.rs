use sysinfo::System;

#[derive(Debug)]
pub struct SystemData {
    pub os_name: String,
    pub os_version: String,
    pub host_name: String,
    pub kernel_version: String,
    pub uptime: u64,
    pub total_memory: u64,
    pub used_memory: u64,
}

pub fn gather_system_info() -> SystemData {
    let mut sys = System::new_all();
    
    sys.refresh_all();

    let os_name = System::name().unwrap_or_else(|| "Unknown OS".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown Kernel".to_string());
    let host_name = System::host_name().unwrap_or_else(|| "Unknown OS".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown Version".to_string());

    SystemData {
        os_name,
        os_version,
        host_name,
        kernel_version,
        uptime: System::uptime(),
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
    }
}

impl SystemData {
    #[must_use]
    pub fn format_uptime(&self) -> String {
        let hours = self.uptime / 3600;
        let minutes = (self.uptime % 3600) / 60;
        format!("{}h {}m", hours, minutes)
    }

    #[must_use]
    pub fn format_memory(&self) -> String {

        let gibibyte = 1_073_741_824.0;

        let total_mem = self.total_memory as f64 / gibibyte;
        let used_mem = self.used_memory as f64 / gibibyte;
        format!("{:.2} GiB / {:.2} GiB", used_mem, total_mem)
    }
}
