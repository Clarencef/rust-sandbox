/*
  Vec (pronounced as vector) called buffer.
  vec! is a macro to create a Vec more easily with an array-like syntax.
  An alternative would be to use Vec::new().
  Rust actually has array's, too, and they look a lot like JavaScripts arrays (e.g. [1, 2]).
  However they don't behave like JavaScripts arrays.
  A JavaScript array is much more similar to Rusts Vec.
  Vec and array can be compared to String and &str in this regard.
  A Vec and a String can have a dynamic size and behave similar to JavaScripts arrays and strings while Rusts array and &str have a fixed size.

  from: https://github.com/Mercateo/rust-for-node-developers/tree/master/read-files
*/

use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

  // Re-assign value
  numbers[2] = 32;

  // Add on to Vector
  numbers.push(6);
  numbers.push(7);

  println!("{:?}", numbers);

  // Get single val
  println!("Single value: {}", numbers[0]);

  // Get Vector length
  println!("Vector length: {}", numbers.len());

  // Vector are stack allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop & mutate vector values
  for x in numbers.iter_mut() {
    *x *=2;
  }

  println!("Number Vec: {:?}", numbers);
}
