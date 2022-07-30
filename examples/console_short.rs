use jabba_lib::jconsole;

fn main() {
    let name = jconsole::input("Name: ");
    let name = name.trim();
    println!("Hello {}!", name);
}
