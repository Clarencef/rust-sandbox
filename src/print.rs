pub fn run() {
  println!("Hello from the print.rs file.");

  // Basic Formatting
  println!("Number: {}", 1);
  println!("{} is from {}", "Albert", "Taiwan");

  // Positional Arguments
  println!(
    "{0} is from {1} and {0} likes to {2}",
    "Albert",
    "Taiwan",
    "code"
  );

  // Named Argument
  println!(
    "{name} likes to play {activity}",
    name = "Albert",
    activity = "Basketball"
  );

  // Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // Placeholder for debug trait
  println!("{:?}", (12, true, "hello"));
}