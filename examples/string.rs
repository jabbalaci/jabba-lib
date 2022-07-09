use jabba_lib::jstring;

fn main() {
    let name = "Laci";
    println!("{} <-> {}", name, jstring::str_rev(name));

    let name = "anna";
    println!("{} is palindrome: {}", name, jstring::is_palindrome(name));

    println!("{:?}", jstring::center("-", 3));
}
