# Introducere în Limbajul de Programare Rust

## Ce este Rust?
Rust este un limbaj de programare sistemic, compilat, care pune accentul pe siguranță, paralelism și performanță. A fost dezvoltat inițial de Mozilla Research și este folosit în prezent de companii precum Mozilla, Dropbox, și Microsoft.

## Caracteristici Principale
- **Siguranța memoriei**: Rust garantează siguranța memoriei fără a folosi un garbage collector
- **Concurență fără date partajate**: "Fearless concurrency" - concurență fără race conditions
- **Zero-cost abstractions**: Abstractizări fără impact asupra performanței
- **Interoperabilitate cu C**: Poate fi folosit împreună cu cod C existent
- **Package manager modern**: Cargo - pentru gestiunea dependințelor și build

## Concepte Fundamentale

### 1. Ownership (Proprietatea)
Rust folosește un sistem unic de ownership pentru a gestiona memoria:
```rust
fn main() {
    let s1 = String::from("hello");    // s1 este proprietarul
    let s2 = s1;                       // proprietatea se mută la s2
    // println!("{}", s1);             // Eroare! s1 nu mai este valid
    println!("{}", s2);                // OK
}
```

### 2. Borrowing (Împrumutul)
În loc să transferăm proprietatea, putem împrumuta referințe:
```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);    // împrumutăm o referință la s1
    println!("Lungimea '{}' este {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### 3. Tipuri de Date de Bază
```rust
fn main() {
    // Întregi
    let x: i32 = 42;        // 32-bit signed
    let y: u64 = 123;       // 64-bit unsigned
    
    // Float
    let pi: f64 = 3.14159;
    
    // Boolean
    let active: bool = true;
    
    // Caractere
    let letter: char = 'A';
    
    // Tuple
    let tup: (i32, f64, char) = (500, 6.4, 'z');
    
    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
}
```

### 4. Structuri și Enumerări
```rust
// Structură
struct Student {
    nume: String,
    varsta: u32,
    activ: bool,
}

// Enumerare
enum Rezultat {
    Succes(i32),
    Eroare(String),
}

fn main() {
    let student = Student {
        nume: String::from("Ion Popescu"),
        varsta: 20,
        activ: true,
    };
    
    let rezultat = Rezultat::Succes(42);
}
```

### 5. Pattern Matching
```rust
fn main() {
    let numar = 42;
    
    match numar {
        0 => println!("Zero"),
        1..=10 => println!("Număr mic"),
        _ => println!("Număr mare"),
    }
}
```

## Primii Pași

### Instalare
1. Vizitează [rustup.rs](https://rustup.rs)
2. Urmează instrucțiunile de instalare pentru sistemul tău de operare
3. Verifică instalarea cu `rustc --version`

### Hello World în Rust
```rust
fn main() {
    println!("Salut, lume!");
}
```

### Compilare și Rulare
```bash
# Compilare
rustc hello.rs

# Rulare
./hello
```

## Resurse de Învățare
- [Cartea Oficială Rust](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Playground Rust](https://play.rust-lang.org/)

## Exerciții Practice Recomandate
1. Implementează un calculator simplu
2. Creează o structură pentru gestionarea unei liste de studenți
3. Scrie un program care manipulează șiruri de caractere
4. Implementează un algoritm de sortare
5. Creează un sistem simplu de gestionare a fișierelor
