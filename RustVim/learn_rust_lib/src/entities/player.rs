// module declaration is in ../entities.rs
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::rc::Rc;

// pub makes visible to other crates
// default is only visible to the current module (file)
// by default they are Move type (move ownership or dropped)
// Can be Copy type if impl Copy trait: #[derive(Copy, Clone)]  
#[derive(Debug)] 
pub struct Player {
  name: String,   // Move type
  health: u32     // Copy type
}


// Self is the current struct type
// can have different named constructors, new is commonly used 
impl Player {
  pub fn new(_name: String, health: u32) -> Self {
    Self {
      name: _name, 
      health           // shorthand when the field and param have same name
    }
  }
}

// Display Trait (Interface) so fn is automatically public
// generic parameter of '_ which is lifetime wildcard (any lifetime)
// Formatter is the buffer you write to from which is prints
// write! macro takes a formatter, format string, and objects to add 
//   returns a fmt::Result
// double {{ }} escape them, 
impl Display for Player {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, 
      "player {{name: {} , health: {}}}", self.name, self.health)
  }
}

// &self is a shorthand for self: &Self to indicate methods 
//   remember that & borrows so they are a copy type
// can technically deref self for members: (*self).member()
//   auto deref allows you to call self.member() 
// &mut self for set methods and &self for get methods

impl Player {
  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }
 
  // String is a move type, so need to borrow
  pub fn get_name(&self) -> &String {
    &self.name
  }
}

// Box and Rc drop/destroy heap allocated obj when out of scope since they own them
// Rc allows a shallow copy (clone of the reference) for multiple owners
impl Player {
  pub fn demo_drop_box(pl: Box<Player>) {
    println!("{}", pl);
  }

  pub fn demo_rc(pl: Rc<Player>) {
    println!("{}", pl);
  }
  
  // fields of Move type, when moved, will invalidate the whole object
  pub fn move_demo() {
    let p1: Player = Player::new(String::from("move field"), 100);
    let _p1str: String = p1.name;
    //  p1.get_name();  // dropped 
  }
}




