use crate::fetcher::SystemData;

pub fn print_fetch(data: &SystemData) {
    let separator = "-------------------------";

    println!();
    println!("[S Y S T E M   I N F O]");
    println!("{separator}");
    println!("OS:     {0}", data.os_name);
    println!("OS Version: {0}", data.os_version);
    println!("Hostname: {0}", data.host_name);
    println!("Kernel: {0}", data.kernel_version);
    println!("Uptime: {0}", data.format_uptime());
    println!("Memory: {0}", data.format_memory());
    println!("{separator}\n");
}
