use crate::fetcher::SystemData;

pub fn print_fetch(data: &SystemData) {
    let separator = "-------------------------";

    println!();
    println!("{0}@{1}", data.user,data.host_name);
    println!("{separator}");
    println!("OS: {0} {1} {2}",data.os_name, data.os_version, data.cpu_arch);
    println!("Kernel: {0}", data.kernel_version);
    println!("Uptime: {0}", data.format_uptime());
    println!("Shell: {0}", data.shell);
    println!("Terminal: {0}", data.terminal);
    println!("WM: {0}", data.wm);
    println!("Cursor: {0} ({1}px)", data.cursor,data.cursor_size);
    println!("CPU: {0} ({1}C/{2}T)",data.cpu, data.core_count, data.thread_count);
    println!("Memory: {0}", data.format_memory());
    println!("Swap: {0}", data.format_swap());
    println!("{separator}\n");
}
