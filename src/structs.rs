// Structs - Used to create custom data types

// Traditional Struct
// struct Color {
//   red: u8,
//   green: u8,
//   blue: u8
// }

// Tuple Struct
// struct Color (u8,u8,u8);

struct Person {
  first_name: String,
  last_name: String
}

impl Person {
  // Construct Person
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string()
    }
  }

  // Get full name
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  // Set last name
  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }

  // Name to tuple
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run () {
  // For Traditional Struct
  // let mut c = Color {
  //   red: 255,
  //   green: 0,
  //   blue: 0
  // };

  // c.red = 200;

  // println!("Color: {} {} {}", c.red, c.green, c.blue);


  // For Tuple Struct
  // let mut c = Color(255, 0, 0);
  // c.0 = 200;

  // println!("Color: {} {} {}", c.0, c.1, c.2);

  // For Person struct
  let mut p = Person::new("Tom", "Fang");
  p.set_last_name("Albert");
  println!("Preson {} {}", p.first_name, p.last_name);
  println!("Preson {}", p.full_name());
  println!("Preson Tuple {:?}", p.to_tuple());
}