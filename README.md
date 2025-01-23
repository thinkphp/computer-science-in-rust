# Computer Science in Rust

## Course Overview
This course introduces the Rust programming language, focusing on its unique features and common programming paradigms. By the end of this course, students will have a solid foundation in Rust programming and be able to write safe, concurrent, and efficient code.

## Lesson 1: Rust Basics and Variables

### 1.1 Hello, World!
Let's start with the traditional "Hello, World!" program:

```rust
fn main() {
    println!("Hello, Stanford!");
}
```

Key points:
- `fn main()` is the entry point of every Rust program.
- `println!` is a macro (note the `!`) for printing to the console.

### 1.2 Variables and Mutability
Rust variables are immutable by default:

```rust
let x = 5; // immutable
let mut y = 10; // mutable

y = 15; // OK
x = 6; // Error: cannot assign twice to immutable variable
```

### 1.3 Data Types
Rust is statically typed, but can infer types:

```rust
let integer: i32 = 42;
let float = 3.14; // f64 by default
let boolean = true;
let character = 'A';
```

## Lesson 2: Control Flow

### 2.1 If Expressions
```rust
let number = 7;

if number < 5 {
    println!("Number is less than 5");
} else if number > 5 {
    println!("Number is greater than 5");
} else {
    println!("Number is 5");
}
```

### 2.2 Loops
Rust provides several looping constructs:

```rust
// loop
let mut counter = 0;
loop {
    println!("Count: {}", counter);
    counter += 1;
    if counter == 5 {
        break;
    }
}

// while
let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number -= 1;
}

// for
for i in 1..=5 {
    println!("{}!", i);
}
```

## Lesson 3: Ownership and Borrowing

### 3.1 Ownership Rules
1. Each value has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.

```rust
{
    let s = String::from("hello"); // s is valid from this point forward
    // do stuff with s
}  // s is no longer valid here
```

### 3.2 Borrowing
Rust allows borrowing references to values:

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### 3.3 Mutable References
```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

## Lesson 4: Structs and Enums

### 4.1 Structs
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Area: {}", rect.area());
}
```

### 4.2 Enums and Pattern Matching
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## Lesson 5: Error Handling

### 5.1 Panic!
```rust
fn main() {
    panic!("crash and burn");
}
```

### 5.2 Result Enum
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

## Lesson 6: Generics and Traits

### 6.1 Generics
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

### 6.2 Traits
```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

## Conclusion
This introduction to Rust covers the fundamental concepts of the language. Students are encouraged to practice these concepts and explore more advanced topics such as concurrency, smart pointers, and the module system.


# Essential Rust.

Rust is a programming language that helps you write faster, more reliable software. High-level ergonomics and low-level control are often at odds with each other in programming language design; Rust stands to challenge that. Through balancing powerful technical capacity and a great developer experience, Rust gives you the option to control low-level details (such as memory usage) without all the hassle traditionally associated with such control.

```
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```  

### Featuring
    * pattern matching
    * efficient C bindings
    * type inference
    * minimal runtime  


```
$ rustc hello.rs
$ ./hello
Hello World!

fn main() {
    // Statements here are executed when the compiled binary is called.
    
    //println! is a macro that prints text to the console.
    println!("Hello World!");
}
```  

## Scalar Types

* Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
* Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
* Floating point: f32, f64
* char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
* bool either true or false
* The unit type (), whose only possible value is an empty tuple: ()
* Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.

## While Control Flow

```
fn main() {
    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }
}
```

## if/else Control Flow
```
fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);
}
```

 ## Euclid's Algorithm https://ideone.com/Yv1jcj

```rust
use std::io;
use std::str::FromStr;
use std::cmp::Ordering;

fn get_number(prompt_input: &str) -> u32 {

       println!("{}", prompt_input);
       
       let mut input = String::new();
       
       io::stdin().read_line(&mut input).expect("no input!");     
       
       u32::from_str(input.trim()).unwrap()      
}

fn main() {
     
       let a = get_number("Enter first number a = ");
       
       let b = get_number("Enter first number b = ");

       println!("The Greatest Common Divisor of {} and {} is {}", a, b, euclid( a, b) );       
}

fn euclid(a: u32, b: u32) -> u32 {
 
   assert!(a > 0 && b > 0);
   
   match a.cmp(&b) {
   
      Ordering::Equal => b,
      
      Ordering::Less => euclid(a,b-a),
      
      Ordering::Greater => euclid(a-b,b)
   }   
}
```
### Practice
Hundreds of companies, large and small, use Rust in production for a variety of tasks, including command line tools, web services, DevOps tooling, embedded devices, audio and video analysis and transcoding, cryptocurrencies, bioinformatics, search engines, Internet of Things applications, machine learning, and even major parts of the Firefox web browser.

## References
* https://cs.lmu.edu/~ray/notes/introrust/
* https://stevedonovan.github.io/rust-gentle-intro/
* https://doc.rust-lang.org/stable/rust-by-example
* https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/#the-rust-bookshelf

## Books:

- (...)

<details><summary>Interview Questions</summary>    

- [link1] (https://github.com/imhq/rust-interview-handbook)    
- [link2] (https://www.turing.com/interview-questions/rust)
    
</details>
