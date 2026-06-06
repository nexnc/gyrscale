// Enforce strict code quality and safety rules
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

use sysinfo::System;


fn main() {

    const GIB_CONV: f64 = 1024.0 * 1024.0 * 1024.0;


    let mut sys = System::new_all();
    sys.refresh_all();
   
    // Get OS Name
    if let Some(os_name) = System::name() {
        println!("OS Name: {os_name}");
    }

    // Get System Name
    if let Some(sys_name) = System::host_name() {
        println!("Hostname: {sys_name}");
    }

    // Get Kernel Version
    if let Some(kernel_ver) = System::kernel_version() {
        println!("Kernel: {kernel_ver}");
    }

    // System Uptime
    let uptime =  System::uptime();
    let hours = uptime / 3600;
    let minutes = (uptime % 3600) / 60;
    println!("Uptime: {hours} hours {minutes} minutes");

    // Get CPU Name
    if let Some(cpu) = sys.cpus().first() {
        println!("CPU: {brand}", brand = cpu.brand());
    }

    // Get shell name
    let shell_env = std::env::var("SHELL");
    let shell = shell_env.as_deref().unwrap_or("Unknown");
    println!("Shell: {}", shell.split('/').next_back().unwrap_or("Unknown"));

    // Get terminal name
    let term_env = std::env::var("TERM");
    let terminal = term_env.as_deref().unwrap_or("Unknown");
    println!("Terminal: {terminal}");
    
    let total_bytes = sys.total_memory();
    let available_bytes = sys.available_memory();

    if total_bytes == 0 {

        println!("RAM: 0.00 GiB / 0.00 GiB (0%)");
    } else {

        #[allow(clippy::cast_precision_loss)]
        let used_ram = (total_bytes - available_bytes) as f64/ GIB_CONV;

        #[allow(clippy::cast_precision_loss)]
        let total_ram = total_bytes as f64 / GIB_CONV;

        let ram_percentage = (used_ram / total_ram) * 100.0;

        println!("RAM: {used_ram:.2} GiB / {total_ram:.2} GiB ({ram_percentage:.0}%)");
    }
}
