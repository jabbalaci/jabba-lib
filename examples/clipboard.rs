use jabba_lib::jclipboard;

fn main() {
    jclipboard::check();

    for &text in &["", "a", "aa", "hello", "world", "rust", "Éva"] {
        jclipboard::set_text(text).unwrap();
        println!("The text {:?} was pasted on the clipboard", text);

        let read = jclipboard::get_text().unwrap();
        println!("Contents of the clipboard: {:?}", read);
        assert_eq!(read, text);
        println!("---");
    }
}
