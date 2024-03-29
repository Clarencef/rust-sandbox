pub fn run () {
  greeting("Hello", "Tom");

  // Bind function values to variables
  let get_sum = add(5, 5);
  println!("Sum: {}", get_sum);

  // Closure
  let n3: i32 = 10;
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
  println!("Closure sum: {}", add_nums(6, 6));
}

fn greeting(greet: &str, name: &str) {
  println!("{} {}", greet, name);
}

fn add (n1: i32, n2: i32) -> i32 {
  n1 + n2
}