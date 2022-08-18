use jabba_lib::jos;

fn main() {
    let op_sys = jos::get_operating_system_name();
    println!("Your operating system is {:?}", op_sys);

    println!(
        "Is it Linux?   {}",
        if jos::is_linux() { "yes" } else { "no" }
    );
    println!(
        "Is it Windows? {}",
        if jos::is_windows() { "yes" } else { "no" }
    );

    let value = jabba_lib::jos::get_available_parallelism().unwrap_or(1);
    println!("Available parallelism: {}", value);
}
