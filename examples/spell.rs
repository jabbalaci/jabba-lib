use jabba_lib::jspell;

fn main() {
    for n in [0, 5, 12, 21, 55, 99, 101, 123, 576, 999, 1000] {
        println!("{}: {}", n, jspell::spell_number(n));
    }
}
