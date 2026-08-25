use crate::fetcher::SystemData;
use std::io::{self, BufWriter, Write};

pub fn print_fetch(data: &SystemData) {
    let separator = "-------------------------";

    let stdout = io::stdout();
    let mut out = BufWriter::with_capacity(2048, stdout.lock());

    let _ = writeln!(out);
    let _ = writeln!(out, "{}@{}", data.user, data.host_name);
    let _ = writeln!(out, "{separator}");
    let _ = writeln!(out, "OS: {} {} {}", data.os_name, data.os_version, data.cpu_arch);
    let _ = writeln!(out, "Kernel: {}", data.kernel_version);
    let _ = writeln!(out, "Uptime: {}", data.format_uptime());
    let _ = writeln!(out, "Shell: {}", data.shell);
    let _ = writeln!(out, "Terminal: {}", data.terminal);
    let _ = writeln!(out, "WM: {}", data.wm);
    let _ = writeln!(out, "Cursor: {} ({}px)", data.cursor, data.cursor_size);
    let _ = writeln!(out, "CPU: {} ({}C/{}T) @ {:.2} GHz", data.cpu, data.core_count, data.thread_count, data.cpu_freq);
    let _ = writeln!(out, "Motherboard: {} {}", data.vendor, data.motherboard);
    let _ = writeln!(out, "Memory: {}", data.format_memory());
    let _ = writeln!(out, "Swap: {}", data.format_swap());
    
    for disk in &data.disks {
        let _ = writeln!(out, "{}", disk.format_disk());
    }
    
    let _ = writeln!(out, "{separator}");

    let _ = out.flush();
}
