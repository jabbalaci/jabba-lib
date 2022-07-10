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

See the folder [examples/](https://github.com/jabbalaci/jabba-lib/tree/main/examples)
for more examples.
