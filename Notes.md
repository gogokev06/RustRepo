# Notes
> Hoi! So Rust... Yeah completely new to it, but I will try to write what I learn.

Rust is a programming language with natively immutable data types and the variable's data type need to be stated. All the code should be inside functions, all functions start with `fn`, the function you start off with is the `fn [fileName]() {}` function. Since Rust is immutable, we need to add `mut` to it so that it can change (it can be different value, but needs to be same type of data) along the code; making it mutable basically. In Rust, the concept of variable ownership is defined as 

A breakdown of Rust's fundamental data types. This table explains what they are and why you'd use them.

| Data Type | What It Is & How It Works | Code Example |
| :--- | :--- | :--- |
| **Scalar Types** | *Types that hold a single value.* | |
| &nbsp;&nbsp;&nbsp;Integers | Your standard whole numbers. `i` types are signed (can be negative), `u` types are unsigned (positive only). The number is the bit size. Use `usize` for indexing collections, as it matches your system's architecture. | `let count: u32 = 1024;` |
| &nbsp;&nbsp;&nbsp;Floating-Point | Numbers with decimal points. `f64` is the default and has higher precision, which is what you want for most calculations. Use `f32` if you need to optimize for memory or performance and can tolerate less precision. | `let pi_approx: f64 = 3.1415926535;` |
| &nbsp;&nbsp;&nbsp;Boolean | A fundamental logic type, can only be `true` or `false`. Essential for control flow like `if` expressions. | `let is_processing: bool = false;` |
| &nbsp;&nbsp;&nbsp;Character | A single Unicode character. Note that a `char` is 4 bytes, so it can represent anything from 'a' to '😻'. Use single quotes. | `let symbol: char = 'Ω';` |
| **Compound Types** | *Types that group multiple values.* | |
| &nbsp;&nbsp;&nbsp;Tuple | A fixed-size, ordered list of values with potentially different types. Great for returning multiple values from a function without needing to define a whole new `struct`. | `let point: (i32, i32) = (10, 20);` |
| &nbsp;&nbsp;&nbsp;Array | A fixed-size list where every element must have the same type. Arrays are allocated on the stack, which makes them very fast and efficient. Their size is known at compile time. | `let buffer: [u8; 256] = [0; 256];` |
| **Common Collections** | *Growable data structures on the heap.* | |
| &nbsp;&nbsp;&nbsp;String Slice (`&str`) | An immutable *view* into a string. It's a borrowed type, meaning it doesn't own the data it points to. String literals are `&str`. Extremely efficient because you're just passing a pointer and a length. | `let name: &str = "Rustacean";` |
| &nbsp;&nbsp;&nbsp;String | The owned, growable, mutable string type. If you need to build or modify a string at runtime, you need a `String`. It's allocated on the heap. | `let mut greeting = String::from("Hello");` |
| &nbsp;&nbsp;&nbsp;Vector (`Vec<T>`) | The workhorse collection. A growable list of items of the same type, allocated on the heap. If you need a list that can change in size, you want a `Vec`. | `let mut scores: Vec<i32> = vec![10, 20];` |
| &nbsp;&nbsp;&nbsp;Hash Map | A collection of key-value pairs. Super efficient for looking up a value based on its key. You must import it from `std::collections`. | `use std::collections::HashMap;` |
| **Custom Data Types** | *The building blocks of your programs.* | |
| &nbsp;&nbsp;&nbsp;Struct | A custom data structure you define. You bundle related data together and give each piece a name. This is how you model the concepts in your application. | `struct Player { score: u32, is_alive: bool };` |
| &nbsp;&nbsp;&nbsp;Enum | A type that can be one of several possible variants. Enums are incredibly powerful in Rust for modeling state and creating type-safe code, especially when combined with the `match` keyword. | `enum State { Loading, Success, Failed };` |
| **Utility Types** | *Core types for robust programming.* | |
| &nbsp;&nbsp;&nbsp;Option | Encodes the possibility of a value being absent. It's either `Some(value)` or `None`. This forces you to handle null cases at compile time, which eliminates a whole class of bugs. | `let last_item: Option<i32> = Some(5);` |
| &nbsp;&nbsp;&nbsp;Result | The standard for functions that can fail. It's either `Ok(value)` for success or `Err(error)` for failure. This makes error handling explicit and robust. | `// In a function: Ok("Success!")` |

The print command is denoted as `println!()` the "!" at the end means it's a Rust macro.
