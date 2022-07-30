# jabba-lib

A general-purpose utility library for Rust. Its API was mainly inspired by the
Python programming language.

## Some examples

This is just a teaser. The library has many more modules
and --in most cases-- the modules contain more functions than shown below.

### console

```rust
use jabba_lib::jconsole;

fn main() {
    let name = jconsole::input("Name: ");
    let name = name.trim();
    println!("Hello {}!", name);
}
```

### math

```rust
use jabba_lib::jmath;

fn main() {
    assert_eq!(jmath::is_palindrome(101), true);
    assert_eq!(jmath::is_prime(97), true);
    assert_eq!(jmath::get_divisors(28), [1, 2, 4, 7, 14, 28]);
    assert_eq!(jmath::factorial(5), 120);
    assert_eq!(jmath::factorial_bigint(33).to_string(), "8683317618811886495518194401280000000");
}
```

### string

```rust
use jabba_lib::jstring;

fn main() {
    let name = "Dave";
    let reversed = jstring::str_rev(name); // evaD
    println!("{} <-> {}", name, reversed);

    let name = "anna";
    println!("{} is palindrome: {}", name, jstring::is_palindrome(name));
}
```

### random

```rust
use jabba_lib::jrandom;

fn main() {
    let number = jrandom::randrange(1, 10); // 10 is excluded
    println!("random number from [1, 10): {}", number);

    let number = jrandom::randint(1, 100); // 100 is included
    println!("random number from [1, 100]: {}", number);

    let mut numbers = vec![1, 2, 3, 4, 5];
    jrandom::shuffle(&mut numbers);
    println!("shuffled: {:?}", numbers); // could be [3, 5, 1, 4, 2]
}
```

### clipboard

Supported platforms: Linux (with X server), Windows.

Under Linux you must have the program `xsel` installed.
You can install it with your package manager.

Under Linux, the text is pasted on both clipboards (to "primary" and "clipboard").

```rust
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
```

### spell

Spell a number (write out in words).

```rust
use jabba_lib::jspell;

fn main() {
    assert_eq!(jspell::spell_number(5), "five");
    assert_eq!(jspell::spell_number(11), "eleven");
    assert_eq!(jspell::spell_number(101), "one hundred and one");
    assert_eq!(jspell::spell_number(999), "nine hundred and ninety-nine");
}
```

### time

```rust
use jabba_lib::jtime;

fn main() {
    let wait = 1.5;

    println!("Waiting for {:.2} seconds...", wait);
    jtime::sleep(wait);
    println!("Done.");
}
```

See the folder [examples/](https://github.com/jabbalaci/jabba-lib/tree/main/examples)
for more examples.
