# jabba-lib

A simple utility library for Rust. Its API was mainly inspired by the
Python programming language.

## Some examples

### console

```rust
use jabba_lib::jconsole;

fn main() {
    let name = jconsole::input("Name: ");
    let name = name.trim();
    println!("Hello {}!", name);
}
```

### random

```rust
use jabba_lib::jrandom;

fn main() {
    let number = jrandom::randrange(1, 10);  // 10 is excluded
    println!("random number from [1, 10): {}", number);

    let number = jrandom::randint(1, 100);  // 100 is included
    println!("random number from [1, 100]: {}", number);

    let mut numbers = vec![1, 2, 3, 4, 5];
    jrandom::shuffle(&mut numbers);
    println!("shuffled: {:?}", numbers);  // could be [3, 5, 1, 4, 2]
}
```

### string

```rust
use jabba_lib::jstring;

fn main() {
    let name = "Dave";
    let reversed = jstring::str_rev(name);  // evaD

    let name = "anna";
    println!("{} is palindrome: {}", name, jstring::is_palindrome(name));
}
```

### clipboard

Supported platforms: Linux (with X server), Windows.

Under Linux you must have the program `xsel` installed.
You can install it with your package manager.

Under Linux, the text is pasted on both clipboards (to "primary" and "clipboard").

```rust
let text = "hello";

jabba_lib::jclipboard::check();  // verify if your platform is supported

jabba_lib::jclipboard::set_text(text).unwrap();
println!("The text {:?} was pasted on the clipboard", text);

let read = jabba_lib::jclipboard::get_text().unwrap();
println!("Contents of the clipboard: {:?}", read);

assert_eq!(read, text);
```

See the folder [examples/](https://github.com/jabbalaci/jabba-lib/tree/main/examples)
for more examples.
