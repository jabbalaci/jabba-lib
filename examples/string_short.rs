use jabba_lib::jstring;

fn main() {
    let name = "Dave";
    let reversed = jstring::str_rev(name); // evaD
    println!("{} <-> {}", name, reversed);

    let name = "anna";
    println!("{} is palindrome: {}", name, jstring::is_palindrome(name));
}
