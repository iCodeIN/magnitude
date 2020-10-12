
![](assets/Magnitude.png)
<a href="https://www.freepik.com/vectors/logo">Logo by www.freepik.com</a>

Magnitude - To infinity and beyond!
==========================================================
[![Crate](https://img.shields.io/crates/v/magnitude.svg)](https://crates.io/crates/magnitude)
[![API](https://docs.rs/magnitude/badge.svg)](https://docs.rs/magnitude) \
This crate is useful when you need to work with algorithms like
[Dijkstra's Shortest Path](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Pseudocode) or
[Floyd–Warshall algorithm](https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm#Algorithm)
that require infinite values in order to be written elegantly.

 One simple example can be finding the max value in a vector:
 ```rust
 use magnitude::Magnitude;

 fn find_max(vec: &Vec<Magnitude<i32>>) -> Magnitude<i32> {
     let mut max = Magnitude::NegInfinite;
     for val in vec {
         if *val > max {
             max = *val;
         }
     }

     max
 }

 let vec: Vec<Magnitude<i32>> = vec![2.into(), 3.into(), 6.into(), (-10).into()];
 assert_eq!(find_max(&vec), 6.into());
 ````
 You can do all **valid** comparison(==, !=, >, <, >=, <=) and arithmetic(+,-, *, /, +=, -=, *=, /=) operations on magnitudes. \
 Invalid operations are listed below which means any other operation is valid.

 # Invalid operations
 * Comparison:
    - two `PosInfinite`
    - two `NegInfinite`
 * Arithmetic:
     - Add:
         - `PosInfinite` + `NegInfinite`
     - Sub:
         - `PosInfinit` - `PosInfinit`
         - `NegInfinit` - `NegInfinit`
     - Mul:
         - zero * `PosInfinite`
         - zero * `NegInfinite`
     - Div:
         - non-zero / `PosInfinite`
         - non-zero / `NegInfinite`
         - `PosInfinite` / zero
         - `NegInfinite` / zero
         - `PosInfinite` / `PosInfinite`
         - `PosInfinite` / `NegInfinite`
         - `NegInfinite` / `PosInfinite`
         - `NegInfinite` / `NegInfinite`