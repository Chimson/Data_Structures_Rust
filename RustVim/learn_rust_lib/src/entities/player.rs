// module declaration is in ../entities.rs
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;

pub struct Player {
  name: String,
  health: u32
}


// Self is the current struct type
// &self is a borrow of the Self object of type &Self, ref to Self
//   remember that & borrows so they are a copy type
// can technically deref self for members: (*self).member()
//   auto deref allows you to call self.member() 

// TODO: what the fuck kind of constructor returns &Self


impl Player {
  pub fn new(_name: String, _health: u32) -> Self {
    Self {
      name: _name, 
      health: _health
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
