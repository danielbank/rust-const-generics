# rust-const-generics

Playing around with const generics. This project was presented at the [Desert Rust](https://rust.azdevs.org/) meetup.

## Examples

### Basic Example

Basic example from the [Shipping Const Generics blog post](https://without.boats/blog/shipping-const-generics/)

```
cargo run --example basic
```

### Rectangle Example

Rectangle example from [the Rust RFC Book](https://rust-lang.github.io/rfcs/2000-const-generics.html)

```
cargo run --example rectangle
```

### Rectangle Example without Const Generics

Rectangle example without const generics

```
cargo run --example rectangle_no_const_generic
```

# Learnings

-   Slicing and dicing with [SliceInfo](https://docs.rs/ndarray/0.14.0/ndarray/struct.SliceInfo.html) (although I ultimately used `window_size` / `chunk_size`)
-   [ndarray windows](https://docs.rs/ndarray/0.12.1/ndarray/iter/struct.Windows.html) are not mutable, but you can get around that with [interior mutability](https://stackoverflow.com/questions/43528081/mutating-a-travelling-window-in-a-rust-ndarray) via [RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
-   _Still Trying To Learn:_ impl blocks for structs using const generics?

# Interesting Rust Image Processing Links

-   [Rust Computer Vision](https://github.com/rust-cv): Dedicated to making computer vision easier than OpenCV and faster than C++
-   [nshare](https://crates.io/crates/nshare): Provides traits that allow conversion between n-dimensional types in different Rust crates
-   [ndarray-image](https://crates.io/crates/ndarray-image): Allows conversion between ndarray's types and image's types
-   [ndarray](https://crates.io/crates/ndarray): implements an n-dimensional container for general elements and for numerics
