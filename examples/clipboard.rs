use jabba_lib::jclipboard;

fn main() {
    let text = "world";

    jclipboard::check();

    jclipboard::set_text(text).unwrap();
    println!("The text {:?} was pasted on the clipboard", text);

    let read = jclipboard::get_text().unwrap();
    println!("Contents of the clipboard: {:?}", read);

    assert_eq!(read, text);
}
