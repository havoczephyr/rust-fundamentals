<link rel="stylesheet" type="text/css" href="assets/style.css">

# Rust Fundamentals
### By Edward Curren on Pluralsight
**Released on Jan 25th 2022**
### Notes by Giovanni G. D'Amico

## Index
<img id="decor" src="assets/decor.png">

- [0-0](#personal-forward) **Personal Forward**
- [1-0](#introduction--setup) **Introduction & Setup**
  - [1-1](#the-project) The Project
  - [1-2](#development-environment-setup) Development Environment Setup
  - [1-3](#anatomy-of-a-rust-program) Anatomy of a Rust Program
  - [1-4](#static-vs-dynamic-and-compiled-vs-interpreted) Static vs Dynamic and Compiled vs Interpreted
  - [1-5](#stack-vs-heap) Stack vs Heap
- [2-0](#data-types) **Data Types**
  - [2-1](#number-types) Number Types
  - [2-2](#characters-and-booleans) Characters and Booleans
  - [2-3](#arrays-and-tuples) Arrays and Tuples
  - [2-4](#strings-and-string-slices) Strings and String Slices
  - [2-5](#string-concatination) String Concatination
- [3-0](#variables) **Variables**
  - [3-1](#vars) Vars
  - [3-2](#casting-data-types) Casting Data Types
  - [3-3](#variable-mutability) Variable Mutability
  - [3-4](#scope-and-shadowing) Scope and Shadowing
- [4-0](#operators) **Operators**
  - [4-1](#math-operators) Math Operators
  - [4-2](#logic-operators) Logic Operators
  - [4-3](#bitwise-operators) Bitwise Operators
  - [4-4](#project-part-1) Project Part 1
- [5-0](#control-flow) **Control Flow**
  - [5-1](#if/else) If/Else
  - [5-2](#enum) Enum
  - [5-3](#option) Option
  - [5-4](#match-statements) Match Statements
  - [5-5](#match-with-enumerations) Match with Enumerations
  - [5-6](#if-let) If Let
  - [5-7](#rust-loops) Rust Loops
  - [5-8](#while-loops) While Loops
  - [5-9](#for-loops) For Loops
  - [5-10](#project-part-2) Project Part 2
- [6-0](#ownership-and-borrowing) **Ownership and Borrowing**
  - [6-1](#memory-management) Memory Management
  - [6-2](#ownership) Ownership
  - [6-3](#borrowing) Borrowing
  - [6-4](#lifetimes) Lifetimes
- [7-0](#functions-and-error-handling) **Functions and Error Handling**
  - [7-1](#functions) Functions
  - [7-2](#ownership-&-borrowing-with-functions) Ownership & Borrowing with Functions
  - [7-3](#closures) Closures
  - [7-4](#error-handling) Error Handling
  - [7-5](#result-enum) Result Enum
  - [7-6](#error-propagation) Error Propagation
- [8-0](#data-structures-&-traits) **Data Structures & Traits**
  - [8-1](#data-structures) Data Structures
  - [8-2](#associated-methods) Associated Methods
  - [8-3](#traits) Traits
- [9-0](#collections) **Collections**
  - [9-1](#sequences) Sequences
  - [9-2](#vectors) Vectors
  - [9-3](#vector-double-ended-queue) Vector Double Ended Queue
  - [9-4](#maps) Maps
  - [9-5](#sets) Sets
- [10-0](#generics) **Generics**
  - [10-1](#generic-types) Generic Types
  - [10-2](#constraints) Constraints
- [11-0](#concurrency) **Concurrency**
  - [11-1](#concurrency-hazards) Concurrency Hazards
  - [11-2](#creating-threads) Creating Threads
  - [11-3](#thread-communication) Thread Communication
  - [11-4](#where-to-go-from-here) Where to go from here
- [12-0](#crates-&-modules) **Crates & Modules**
  - [12-1](#modules) Modules
  - [12-2](#cargo.toml) Cargo.toml
  - [12-3](#cargo-fundamentals) Cargo Fundamentals
  - [12-4](#publishing-crates) Publishing Crates
- [13-0](#conclusion) Conclusion

## **Personal Forward**
[<img id="undo" src="assets/undo_flat.png">](#index)
<img id="decor" src="assets/decor.png">

Hey just wanted to jut in, Im very happy that Pluralsight has finally decided to host a video on rust. I was surprised to see the upload and got very excited to dip my toes once again into rust. When I initially tried to learn, I was almost exclusively using the documentation. To be honest, the documentation wasn't bad but I felt kinda alone trying to figure things out.

If I have my own 2 cents on anything i'll preface it with an "**MHZ**:" obviously most of this is based on the video by Edward Curren so I want to preserve his intake throughout this document.

## **Introduction & Setup** 
[<img id="undo" src="assets/undo_flat.png">](#index)
<img id="decor" src="assets/decor.png">

The Two Questions we ask ourselves when learning something new:
1. What is this?
2. Why should we care?

To answer the first on behalf of Rust, Rust is a language that is built on Safety & Speed.
- Speed
  - Its programs can typically run as fast or faster than C++ programs.
  - writing Concurrent(making use of multiple CPU Cores) is trivial.
- Safety
  - Rust Memory Management is handled by Rust without the need for a garbage collector.
    - Essentially Rust will figure out how to manage memory at compile time.
    - Ergo, if Rust compiles, it will run without error.
- Rust is natively cross-platform
- Rust enforces consistancy which supports governance and makes onboarding easier
  - by being consistant, its easier to maintain.
- Allows mentoring of developers to focus on areas other then defensive coding.
  - Your compiler is a great teacher
  - **MHZ**: He aint kidding, Ive never seen a more verbose and descriptive compiler.

**So Whats the Catch?**
  - Rust has a steep learning curve
  - You must approach learning code in a different way

**That Being Said:**
  - Rust is one of the most loved languages in the last several years
  - It's a good time to learn rust because big companies are investing in Rust's Future.


### **The Project** 
[<img id="undo" src="assets/undo_flat.png">](#index)

Throughout the Video we will be working on a number of projects to help better understand the material

The Premise behind the Projects:
  We are hired by **Duck Airlines** which is proud to say they have a 100% Landing Rate. So Much so, that their mottos is:

  **"Duck Airlines, we hit the ground every time"**

  They have hired us to create an application that will caluclate the great circle route distance between two airports.

### **Development Environment Setup** 
[<img id="undo" src="assets/undo_flat.png">](#index)

**MHZ**: I already have rust installed so im mostly gonna just glance through this.

You have your 3 flavors of Rust Compilers, Rust Stable, Rust Beta and Rust Nightly. Rust Stable and Beta are released once every 6 weeks with Nightly being released every night.

**MHZ**: There is actually a bit more to this. Nightly will always contain features that the Rust Development team doesn't believe meets the standards of the Beta and Stable. There are a handful of frameworks that only work with Nightly.<br>
On the other hand, Beta, unless the feature is deemed otherwise, will usually be a reflection of what features Stable will eventually have.

You'll need to download rustup which is the toolchain management utility for Rust from [here](https://rustup.rs). Follow the instructions for your OS and if it worked you should be able to type out `rustc --version` and get the version on your machine.

For Windows, you will need to install the C++ build tools for Visual Studio 2013 or later. you can either install Visual Studio 2019/17/15/13 and install the C++ tools in the installer or get the tools directly [here](https://visualstudio.microsoft.com/visual-cpp-build-tools)

if Rust generated this error, once you have the needed tools installed, you should be able to run through the install clean by running it again.

Once you have that installed you'll need an IDE of some sort. Jetbrains IntelliJ IDEA is the IDE that will be used for this program.
link to installation is [here](https://jetbrains.com/idea/download)
**MHZ**: Maybe im a bit vanilla but im just using VS Code. its good software and i've found myself wanting to use it with as many languages as I can. you can find VSC [here](https://code.visualstudio.com)

Finally you can also use the rust playground if you want something closer to a Repl [here](https://play.rust-lang.org)

### **Anatomy of a Rust Program** 
[<img id="undo" src="assets/undo_flat.png">](#index)

Every program will have an entry point and for rust that will be the `main()` function.
  - `fn` is used to define a function.
  - you can only ever have ONE `main()` function. if there were multiple `main()` functions in your application, the compiler will not know what is your entry point and rust will just fail to compile.
  - in C fashion, curlys `{}` are used to wrap the definition of a function.
```rust
fn main() {
  println!("Hello World!");
}
```
as seen in other c languages, `println` will print the line provided into your console. the exclamation point at the end designates println as a `macro` and not a `function`. we will cover macros later but it is essentially a piece of code within a function.

to continue with the example, we can add a line at the top that will skip any errors involving unused variables, for the purposes of development, this can be handy to quickly compile and test without being bombarded with warnings. `#![allow(unused_variables)]`
if we do compile the code we get this warning but it will compile:

```bash
Compiling rust_fund_one v0.1.0 (C:\Users\HavocZephyr\Documents\Projects\rust_projects\2022\rust_notes\rust_fund_one)
warning: unused variable: `unused_var`
 --> src\main.rs:4:9
  |
4 |     let unused_var: u32 = 0;
  |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_unused_var`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `rust_fund_one` (bin "rust_fund_one") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.87s     
     Running `target\debug\rust_fund_one.exe`
Hello, world!
```

### **Static vs Dynamic and Compiled vs Interpreted** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Stack vs Heap** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

## **Data Types** 
[<img id="undo" src="assets/undo_flat.png">](#index)
<img id="decor" src="assets/decor.png">

wex quas exort

### **Number Types** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Characters and Booleans** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Arrays and Tuples** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Strings and String Slices** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **String Concatination** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

## **Variables** 
[<img id="undo" src="assets/undo_flat.png">](#index)
<img id="decor" src="assets/decor.png">

wex quas exort

### **Vars** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Casting Data Types** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Variable Mutability** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Scope and Shadowing** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

## **Operators** 
[<img id="undo" src="assets/undo_flat.png">](#index)
<img id="decor" src="assets/decor.png">

wex quas exort

### **Math Operators** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Logic Operators** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Bitwise Operators** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Project Part 1** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

## **Control Flow** 
[<img id="undo" src="assets/undo_flat.png">](#index)
<img id="decor" src="assets/decor.png">

wex quas exort

### **If/Else** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Enum** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Option** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Match Statements** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Match with Enumerations** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **If Let** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Rust Loops** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **While Loops** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **For Loops** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Project Part 2** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

## **Ownership And Borrowing**
[<img id="undo" src="assets/undo_flat.png">](#index)
<img id="decor" src="assets/decor.png">

wex quas exort

### **Memory Management** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Ownership** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Borrowing** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Lifetimes** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Project Part 1** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

## **Functions and Error Handling** 
[<img id="undo" src="assets/undo_flat.png">](#index)
<img id="decor" src="assets/decor.png">
wex quas exort

### **Functions** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Ownership and Borrowing With Functions** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Closures** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Error Handling** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Result Enum** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Error Propagation** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

## **Data Structures & Traits** 
[<img id="undo" src="assets/undo_flat.png">](#index) 
<img id="decor" src="assets/decor.png">

wex quas exort

### **Data Structures** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Associated Methods** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Traits** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

## **Collections** 
[<img id="undo" src="assets/undo_flat.png">](#index)
<img id="decor" src="assets/decor.png">

wex quas exort

### **Sequences** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Vectors** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Vector Double Ended Queue** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Maps** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Sets** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

## **Generics** 
[<img id="undo" src="assets/undo_flat.png">](#index)
<img id="decor" src="assets/decor.png">

wex quas exort

### **Generic Types** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Constraints** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

## **Concurrency** 
[<img id="undo" src="assets/undo_flat.png">](#index)
<img id="decor" src="assets/decor.png">

wex quas exort

### **Concurrency Hazards** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Creating Threads** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Thread Communication** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Where To Go From Here** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

## **Crates & Modules** 
[<img id="undo" src="assets/undo_flat.png">](#index)
<img id="decor" src="assets/decor.png">

wex quas exort

### **The Project** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Modules** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Cargo.toml** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Cargo Fundamentals** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

### **Publishing Crates** 
[<img id="undo" src="assets/undo_flat.png">](#index)

wex quas exort

## **Conclusion** 
[<img id="undo" src="assets/undo_flat.png">](#index)
<img id="decor" src="assets/decor.png">

wex quas exort
