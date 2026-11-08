# Veliki Rust Cheatsheet

Stvari koje je stvarno korisno znati (uključujući async, File I/O i opći I/O).

---

## 1. Osnovne stvari i Cargo

```bash
# Novi projekt
cargo new ime_projekta
cargo new --lib ime_biblioteke

# Pokretanje
cargo run
cargo run --release
cargo check          # brza provjera
cargo build --release
cargo test
cargo doc --open
cargo clippy         # linter
cargo fmt
```

```toml
# Cargo.toml primjer
[package]
name = "moji_projekt"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

---

## 2. Varijable, mutabilnost i tipovi

```rust
let x = 5;                    // immutable
let mut y = 10;               // mutable
y = 20;

let z: i32 = 42;              // eksplicitni tip
let a = 3.14;                 // f64
let b: f32 = 3.14;
let c = true;                 // bool
let d = 'R';                 // char (Unicode)
let e = "hello";              // &str
let f = String::from("hello"); // String

const MAX: u32 = 100_000;     // konstanta (mora biti poznata u compile-time)
static COUNTER: AtomicUsize = AtomicUsize::new(0);
```

**Najčešći tipovi:**
- Integer: `i8/i16/i32/i64/i128/isize`, `u8/u16/u32/u64/u128/usize`
- Float: `f32`, `f64`
- `bool`, `char`, `str`, `String`
- Tuple: `(i32, f64, bool)`
- Array: `[i32; 5]`
- Slice: `&[i32]`

---

## 3. Ownership, Borrowing i Lifetimes (srce Rusta)

```rust
// Ownership
let s1 = String::from("hello");
let s2 = s1;                  // s1 se move-a, više nije validan
// println!("{}", s1);        // greška!

let s3 = s2.clone();          // duboko kopiranje

// Borrowing
fn len(s: &String) -> usize { // immutable borrow
    s.len()
}

fn push(s: &mut String) {     // mutable borrow
    s.push_str(" world");
}

// Rules:
// 1. U bilo kojem trenutku: ili jedan &mut T, ili bilo koliko &T
// 2. Reference moraju uvijek biti validne (lifetime)

// Lifetime anotacije
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Struct s lifetimeom
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

---

## 4. Control Flow

```rust
if condition {
} else if other {
} else {
}

// if je ekspresija
let number = if condition { 5 } else { 6 };

// match (ekstremno moćan)
match value {
    1 => println!("jedan"),
    2 | 3 => println!("dva ili tri"),
    4..=10 => println!("od 4 do 10"),
    _ => println!("ostalo"),
}

// if let / while let
if let Some(x) = option {
    println!("{}", x);
}

while let Some(x) = stack.pop() {
    // ...
}

// loop, while, for
loop {
    break;
    continue;
}

for i in 0..10 { }           // 0 do 9
for i in 0..=10 { }          // 0 do 10
for item in vec.iter() { }
for (i, item) in vec.iter().enumerate() { }
```

---

## 5. Funkcije, Metode i Closurei

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b                     // bez return i bez ;
}

// Metode
impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Point) -> f64 {
        // ...
    }

    fn move_by(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

// Closurei
let add = |a, b| a + b;
let add = |a: i32, b: i32| -> i32 { a + b };

// Capture
let x = 5;
let print_x = || println!("{}", x);           // immutable borrow
let mut y = 5;
let mut add_y = || y += 1;                    // mutable borrow
let consume = move || println!("{}", x);      // move
```

---

## 6. Structs, Enums i Pattern Matching

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

let user = User {
    username: String::from("pero"),
    email: String::from("pero@example.com"),
    active: true,
};

// Tuple struct
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);

// Unit struct
struct AlwaysEqual;

// Enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Option i Result (najvažniji enumi)
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Pattern matching s destructuringom
match msg {
    Message::Quit => {},
    Message::Move { x, y } => {},
    Message::Write(text) => {},
    Message::ChangeColor(r, g, b) => {},
}
```

---

## 7. Generics, Traits i Trait Objects

```rust
// Generics
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    // ...
}

struct Point<T> {
    x: T,
    y: T,
}

// Traits
trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

// Trait bound
fn notify(item: &impl Summary) { }
fn notify<T: Summary>(item: &T) { }
fn notify<T>(item: &T) where T: Summary + Display { }

// Trait objects (dyn)
fn print_summary(item: &dyn Summary) { }

// Derive
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Point { x: i32, y: i32 }
```

---

## 8. Error Handling

```rust
// panic! (neopravljivo)
panic!("nešto je pošlo po zlu");

// Result
fn read_file() -> Result<String, std::io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// ? operator automatski radi From conversion
// unwrap / expect
let v = some_option.unwrap();
let v = some_result.expect("poruka greške");

// Custom error (preporučeno s thiserror ili anyhow)
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}
```

---

## 9. Collections

```rust
// Vec
let mut v = Vec::new();
v.push(1);
let v = vec![1, 2, 3];
let third = &v[2];
let third = v.get(2);          // Option

// String
let mut s = String::from("hello");
s.push_str(" world");
s.push('!');
let s2 = s + " more";          // s se move-a
let s3 = format!("{}-{}", s1, s2);

// HashMap
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.entry(String::from("Yellow")).or_insert(50);
```

---

## 10. Iterators (jako važno)

```rust
let v = vec![1, 2, 3];

// Lazy
v.iter()                       // &T
v.iter_mut()                   // &mut T
v.into_iter()                  // T (troši v)

v.iter()
 .map(|x| x * 2)
 .filter(|&x| x > 2)
 .collect::<Vec<_>>();

// Česti adaptori
.map(), .filter(), .filter_map()
.take(), .skip(), .step_by()
.enumerate(), .zip()
.chain(), .flatten()
.inspect()                     // debugging

// Consuming adaptori
.collect(), .sum(), .product()
.fold(), .reduce()
.any(), .all(), .find()
.count(), .max(), .min()
```

---

## 11. Smart Pointeri

```rust
// Box (heap alokacija)
let b = Box::new(5);

// Rc (reference counting, single-threaded)
use std::rc::Rc;
let a = Rc::new(5);
let b = Rc::clone(&a);

// RefCell (interior mutability, runtime borrow checking)
use std::cell::RefCell;
let x = RefCell::new(5);
*x.borrow_mut() += 1;

// Arc + Mutex (multi-threaded)
use std::sync::{Arc, Mutex};
let counter = Arc::new(Mutex::new(0));
```

---

## 12. Concurrency (std)

```rust
use std::thread;
use std::sync::mpsc;

// Thread
let handle = thread::spawn(|| {
    // ...
});
handle.join().unwrap();

// Channel
let (tx, rx) = mpsc::channel();
tx.send(42).unwrap();
let val = rx.recv().unwrap();

// Mutex
let m = Mutex::new(5);
{
    let mut num = m.lock().unwrap();
    *num = 6;
}
```

---

## 13. Moduli i Visibility

```rust
// lib.rs ili main.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;
use self::front_of_house::hosting;   // relativno
use super::hosting;                  // parent

pub use crate::front_of_house::hosting; // re-export

// Visibility:
// - sve je privatno po defaultu
// - pub čini javnim
// - pub(crate) vidljivo unutar crate-a
// - pub(super) vidljivo u parent modulu
```

---

## 14. Macros (osnovno)

```rust
// Declarative macro
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

// Vec macro primjer
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Derive macros i procedural macros su napredniji (syn + quote)
```

---

## 15. Korisni idiomi i savjeti

```rust
// Newtype pattern
struct Meters(f64);

// Builder pattern
let config = Config::builder()
    .timeout(30)
    .retries(3)
    .build()?;

// Type alias
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Drop trait
impl Drop for MyType {
    fn drop(&mut self) {
        println!("čišćenje...");
    }
}

// From / Into / TryFrom / TryInto
impl From<i32> for MyType { ... }

// AsRef / AsMut
fn process(s: impl AsRef<str>) { }

// Default
#[derive(Default)]
struct Config { ... }

// ? u main (s anyhow ili Box<dyn Error>)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ...
    Ok(())
}
```

---

## 16. Najkorisniji crateovi

| Crate          | Za što                          |
|----------------|---------------------------------|
| `serde`        | (de)serijalizacija              |
| `tokio`        | async runtime                   |
| `anyhow`       | jednostavni error handling      |
| `thiserror`    | custom error tipovi             |
| `clap`         | CLI argumenti                   |
| `reqwest`      | HTTP client                     |
| `rayon`        | data parallelism                |
| `tracing`      | structured logging              |
| `chrono` / `time` | datumi i vrijeme             |
| `uuid`         | UUID-ovi                        |
| `regex`        | regularni izrazi                |
| `parking_lot`  | brži Mutex/RwLock               |

---

## 17. Brzi podsjetnici

- `cargo add serde --features derive`
- `rustup update` + `rustup component add clippy rustfmt`
- `RUST_BACKTRACE=1` za bolje panic poruke
- `#[must_use]` na funkcijama koje vraćaju Result/Option
- Preferiraj `&str` umjesto `String` u parametrima funkcija
- Preferiraj `impl Trait` u argumentima kad je moguće
- `clippy::pedantic` je odličan (ali malo agresivan)

---

## 18. Async / Await (Tokio)

### Osnovni setup

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Hello async!");
    sleep(Duration::from_secs(1)).await;
    println!("Gotovo");
}
```

### Async funkcije i `.await`

```rust
async fn fetch_data() -> String {
    // simulacija I/O
    tokio::time::sleep(Duration::from_millis(500)).await;
    "podaci".to_string()
}

async fn main_logic() {
    let data = fetch_data().await;
    println!("{}", data);
}
```

### Spawn taskova

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let handle = task::spawn(async {
        // radi u pozadini
        42
    });

    let result = handle.await.unwrap();
    println!("Rezultat: {}", result);
}
```

### Concurrent izvršavanje (`join!`, `select!`, `try_join!`)

```rust
use tokio::join;
use tokio::select;

async fn work1() -> i32 { 1 }
async fn work2() -> i32 { 2 }

#[tokio::main]
async fn main() {
    // paralelno čekaj oba
    let (a, b) = join!(work1(), work2());
    println!("{} {}", a, b);

    // prvi koji završi
    select! {
        res = work1() => println!("work1: {}", res),
        res = work2() => println!("work2: {}", res),
    }
}
```

### Async Mutex, RwLock, Semaphore

```rust
use tokio::sync::{Mutex, RwLock, Semaphore};
use std::sync::Arc;

let counter = Arc::new(Mutex::new(0));

let counter2 = counter.clone();
tokio::spawn(async move {
    let mut num = counter2.lock().await;
    *num += 1;
});
```

### Channels (async)

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        tx.send("poruka").await.unwrap();
    });

    while let Some(msg) = rx.recv().await {
        println!("Primljeno: {}", msg);
    }
}

// oneshot (jednokratni)
use tokio::sync::oneshot;
let (tx, rx) = oneshot::channel();
tx.send(42).unwrap();
let val = rx.await.unwrap();
```

### Streams

```rust
use tokio_stream::{self as stream, StreamExt};

let mut stream = stream::iter(vec![1, 2, 3]);

while let Some(value) = stream.next().await {
    println!("{}", value);
}
```

### Async I/O (osnovno s Tokio)

```rust
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // čitanje cijelog filea
    let content = fs::read_to_string("input.txt").await?;
    
    // pisanje
    fs::write("output.txt", "hello async").await?;

    // buffered
    let mut file = fs::File::open("input.txt").await?;
    let mut buf = vec![0u8; 1024];
    let n = file.read(&mut buf).await?;
    
    Ok(())
}
```

### Timeout i cancellation

```rust
use tokio::time::{timeout, Duration};

let result = timeout(Duration::from_secs(2), async_operation()).await;
match result {
    Ok(val) => println!("Uspjeh: {:?}", val),
    Err(_) => println!("Timeout!"),
}
```

### Korisni Tokio featurei

```toml
tokio = { version = "1", features = [
    "rt-multi-thread",  # multi-threaded runtime
    "macros",           # #[tokio::main], #[tokio::test]
    "fs",               # async file I/O
    "io-util",          # AsyncReadExt, AsyncWriteExt
    "sync",             # Mutex, channels, etc.
    "time",             # sleep, timeout, interval
    "net",              # TCP/UDP
    "process",          # async procesi
    "signal",           # signal handling
] }
```

---

## 19. Rad s datotekama (File I/O)

### Osnovni importi

```rust
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, BufReader, BufWriter, BufRead, Seek, SeekFrom};
use std::path::{Path, PathBuf};
```

### Path i PathBuf

```rust
let path = Path::new("folder/file.txt");
let mut pathbuf = PathBuf::from("folder");
pathbuf.push("file.txt");

path.exists();
path.is_file();
path.is_dir();
path.extension();          // Some("txt")
path.file_name();
path.parent();

// join
let full = Path::new("dir").join("file.txt");
```

### Tekstualni fileovi

```rust
// Čitanje cijelog filea u String
let content = fs::read_to_string("input.txt")?;

// Pisanje Stringa u file (overwrite)
fs::write("output.txt", "Hello, world!\n")?;

// Čitanje liniju po liniju (BufReader)
let file = File::open("input.txt")?;
let reader = BufReader::new(file);

for line in reader.lines() {
    let line = line?;
    println!("{}", line);
}

// Pisanje linija (BufWriter)
let file = File::create("output.txt")?;
let mut writer = BufWriter::new(file);

writeln!(writer, "Prva linija")?;
writeln!(writer, "Druga linija")?;
writer.flush()?;  // važno!
```

### Binarni fileovi

```rust
// Čitanje cijelog filea u Vec<u8>
let bytes = fs::read("image.png")?;

// Pisanje bajtova
fs::write("output.bin", &bytes)?;

// Ručno čitanje u buffer
let mut file = File::open("data.bin")?;
let mut buffer = [0u8; 1024];
let n = file.read(&mut buffer)?;  // pročitano n bajtova

// Pisanje buffera
let mut file = File::create("data.bin")?;
file.write_all(&[0x01, 0x02, 0x03, 0xFF])?;

// Seek (pomicanje po fileu)
file.seek(SeekFrom::Start(0))?;
file.seek(SeekFrom::End(-10))?;
file.seek(SeekFrom::Current(5))?;
```

### OpenOptions (fine-grained kontrola)

```rust
let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .append(true)       // dodaj na kraj
    .truncate(true)     // obriši sadržaj
    .open("file.txt")?;
```

### Directory operacije

```rust
// Kreiranje foldera
fs::create_dir("novi_folder")?;
fs::create_dir_all("a/b/c")?;  // rekurzivno

// Brisanje
fs::remove_file("file.txt")?;
fs::remove_dir("prazan_folder")?;
fs::remove_dir_all("folder_sa_sadrzajem")?;

// Čitanje sadržaja foldera
for entry in fs::read_dir("folder")? {
    let entry = entry?;
    let path = entry.path();
    if path.is_file() {
        println!("File: {:?}", path);
    }
}

// Kopiranje / premještanje
fs::copy("source.txt", "dest.txt")?;
fs::rename("old.txt", "new.txt")?;
```

### Metadata

```rust
let metadata = fs::metadata("file.txt")?;
metadata.len();            // veličina u bajtovima
metadata.is_file();
metadata.is_dir();
metadata.modified()?;
metadata.permissions();
```

### Async verzija (Tokio)

```rust
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // tekst
    let content = fs::read_to_string("input.txt").await?;
    fs::write("output.txt", content).await?;

    // binarni
    let bytes = fs::read("data.bin").await?;
    fs::write("copy.bin", &bytes).await?;

    // linije
    let file = fs::File::open("input.txt").await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        println!("{}", line);
    }

    Ok(())
}
```

---

## 20. Opći I/O (stdin / stdout / stderr + traitovi)

### Standardni ulaz / izlaz

```rust
use std::io::{self, Write, BufRead};

// Čitanje jedne linije sa stdin
let mut input = String::new();
io::stdin().read_line(&mut input)?;
let input = input.trim();

// Čitanje svih linija
let stdin = io::stdin();
for line in stdin.lock().lines() {
    let line = line?;
    println!("Primljeno: {}", line);
}

// Ispis
print!("Bez novog reda");
println!("Sa novim redom");
eprint!("Na stderr");
eprintln!("Stderr s novim redom");

// Flush stdout
io::stdout().flush()?;
```

### Važni traitovi iz `std::io`

```rust
use std::io::{Read, Write, BufRead, Seek};

// Read
fn read_something<R: Read>(mut reader: R) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

// Write
fn write_something<W: Write>(mut writer: W, data: &[u8]) -> io::Result<()> {
    writer.write_all(data)?;
    writer.flush()?;
    Ok(())
}

// BufRead (ima lines(), read_line(), fill_buf()...)
fn process_lines<R: BufRead>(reader: R) -> io::Result<()> {
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
```

### Cursor (in-memory Read + Write + Seek)

```rust
use std::io::Cursor;

let mut cursor = Cursor::new(vec![1, 2, 3, 4, 5]);
let mut buf = [0u8; 2];
cursor.read_exact(&mut buf)?;
cursor.seek(SeekFrom::Start(0))?;
```

### Error handling s I/O

```rust
use std::io::{self, ErrorKind};

match File::open("nepostoji.txt") {
    Ok(file) => { /* ... */ },
    Err(e) if e.kind() == ErrorKind::NotFound => {
        println!("File ne postoji");
    },
    Err(e) => return Err(e),
}

// ili s ?
let file = File::open("file.txt").map_err(|e| {
    eprintln!("Greška pri otvaranju: {}", e);
    e
})?;
```

### Korisni helperi

```rust
// Brzo čitanje cijelog stdin u String
let mut input = String::new();
io::stdin().read_to_string(&mut input)?;

// Copy između readera i writera
io::copy(&mut reader, &mut writer)?;

// take / chain
let limited = reader.take(100);  // max 100 bajtova
```

---

## 21. Brzi IO + Async podsjetnici

| Zadatak                        | Sync                          | Async (Tokio)                     |
|--------------------------------|-------------------------------|-----------------------------------|
| Pročitaj cijeli tekst file     | `fs::read_to_string`          | `fs::read_to_string().await`       |
| Pročitaj cijeli binarni file   | `fs::read`                    | `fs::read().await`                 |
| Piši String / bajtove          | `fs::write`                   | `fs::write().await`                |
| Linije po liniju               | `BufReader::lines()`          | `BufReader::lines().next_line()`   |
| Buffered pisanje               | `BufWriter`                   | `BufWriter` + `AsyncWriteExt`      |
| Directory listing              | `fs::read_dir`                | `fs::read_dir` + stream            |
| Stdin linija                   | `stdin().read_line`           | `tokio::io::stdin()`               |

**Najvažnija pravila:**
- Za tekst → `String` / `BufReader` + `lines()`
- Za binarni → `Vec<u8>` / `read` / `write_all`
- Uvijek razmisli o `BufReader` / `BufWriter` kad radiš puno malih operacija
- U async kodu **nikad** ne zovi blocking `std::fs` operacije direktno na runtime threadu (koristi `tokio::fs` ili `spawn_blocking`)
- `?` + `io::Result` je tvoj prijatelj

---

*Kraj cheatsheeta. Sretno s Rustom!*
