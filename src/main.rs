use gyrscale::display::print_fetch;
use gyrscale::fetcher::gather_system_info;

fn main() {
    let sys_data = gather_system_info();
    
    print_fetch(&sys_data);
}
