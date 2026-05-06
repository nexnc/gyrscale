use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
   
    // Get OS Name
    if let Some(os_name) = System::name() {
        println!("OS Name: {}", os_name);
    }

    // Get System Name
    if let Some(sys_name) = System::host_name() {
        println!("Hostname: {}", sys_name);
    }

    // Get Kernel Version
    if let Some(kernel_ver) = System::kernel_version() {
        println!("Kernel: {}", kernel_ver);
    }

    // System Uptime
    let uptime =  System::uptime();
    let hours = uptime / 3600;
    let minutes = (uptime % 3600) / 60;
    println!("Uptime: {} hours {} minutes", hours, minutes);

    // Get CPU Name
    if let Some(cpu) = sys.cpus().first() {
        println!("CPU: {}", cpu.brand());
    }

    // Get shell name
    let shell = std::env::var("SHELL").unwrap_or("Unknown".to_string());
    println!("Shell: {}", shell.split('/').last().unwrap_or("Unknown"));

    // Get terminal name
    let terminal = std::env::var("TERM").unwrap_or("Unknown".to_string());
    println!("Terminal: {}", terminal);


    const GIB_CONV: f64 = 1024.0 * 1024.0 * 1024.0;
    let used_ram = (sys.total_memory() - sys.available_memory()) as f64/ GIB_CONV;
    let total_ram = sys.total_memory() as f64 / GIB_CONV;

    println!("RAM: {:.2} GiB / {:.2} GiB ({:.0}%)", used_ram, total_ram, (used_ram/total_ram * 100.0));
}
