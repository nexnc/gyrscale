use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    if let Some(os_name) = System::name() {
        println!("OS Name: {}", os_name);
    }

    // Get CPU Name
    if let Some(cpu) = sys.cpus().first() {
        println!("CPU Name {}", cpu.brand());
    }

    let used_ram = sys.used_memory() / 1024 / 1024 / 1024;
    let total_ram = sys.total_memory() / 1024 / 1024 / 1024;

    println!("RAM:    {} GB / {} GB", used_ram, total_ram);


}
