use crate::fetcher::SystemData;

pub fn print_fetch(data: &SystemData) {
    let separator = "-------------------------";

    println!();
    println!("[S Y S T E M   I N F O]");
    println!("{}", separator);
    println!("OS:     {}", data.os_name);
    println!("OS Version: {}", data.os_version);
    println!("Hostname: {}", data.host_name);
    println!("Kernel: {}", data.kernel_version);
    println!("Uptime: {}", data.format_uptime());
    println!("Memory: {}", data.format_memory());
    println!("{}\n", separator);
}
