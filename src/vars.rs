// Variables hold primitive data or reference to data
// Variable are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Albert";
  let age = 32;
  let mut country = "Taiwan"; // use "mut" to make variable become mutable.

  country = "Japan";

  println!("My name is {} and I am {} from {}", name, age, country);

  // Define constant
  const ID: i32 = 010;
  println!("ID: {}", ID);

  // Assign multiple vars
  let (my_name, my_age) = ("Albert", 32);
  println!("{} is {}", my_name, my_age);

}