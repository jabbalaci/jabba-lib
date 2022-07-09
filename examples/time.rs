use jabba_lib::jtime;

fn main() {
    let wait = 2.0;

    println!("Waiting for {:.2} seconds...", wait);
    jtime::sleep(wait);
    println!("Done.");
}
