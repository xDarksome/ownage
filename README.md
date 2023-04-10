# ownage
Perform a massive ownage of your variables!

For those who tired of manually cloning all the fancy arcs and boxes before passing them into a closure/thread/future, but who's also against introducing another general purpose macro into the codebase.

The `own` function tries to provide the golden mean between code ergonomics and readability.

## Usage
```rust
use ownage::own;

let string = String::new();
let str_ref = string.as_str();
let vec = Vec::<bool>::new();
let slice = vec.as_slice();
let arc = std::sync::Arc::new(42u8);
let u = 42u8;

let answer = own((&string, str_ref, &vec, slice, &arc, &u), |s, s_ref, v, sl, arc, u| {
   std::thread::spawn(move || {
       // Do your dirty stuff here
       u
   })
   .join()
   .unwrap()
});

assert_eq!(answer, 42);
```
