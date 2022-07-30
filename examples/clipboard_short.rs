use jabba_lib::jclipboard;

fn main() {
    let text = "hello";

    jabba_lib::jclipboard::check(); // verify if your platform is supported

    jabba_lib::jclipboard::set_text(text).unwrap();
    println!("The text {:?} was pasted on the clipboard", text);

    let read = jabba_lib::jclipboard::get_text().unwrap();
    println!("Contents of the clipboard: {:?}", read);

    assert_eq!(read, text);
}
