// Conditionals - Used to check the condition of something and act on the result.

pub fn run() {
  let age: u8 = 18;
  let check_id: bool = false;
  let knew_person_is_adult = true;

  // If/Else
  if age >= 21 && check_id || knew_person_is_adult {
    println!("Bartender: What would you like to drink?");
  } else if age < 21 && check_id {
    println!("Bartender: Sorry, you have to leave.");
  } else {
    println!("Bartender: I'll need to check your id.");
  }

  // Shorthand If
  let is_of_age = if age >= 21 { true } else { false };
  println!("Is Of Age: {}", is_of_age)
}