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

// enums restrict to given constant possibilities
// each choice can wrap a tuple, have fields, or neither, and mixed
// useful for pattern matching
// Move type by default
// can implement traits, methods, etc.
pub enum PlayerMode {
  Enhanced(usize),
  Default(usize),
  Dead
}

pub fn get_val_from_status(mode: &PlayerMode) -> usize {
  match *mode {
    PlayerMode::Enhanced(v) => v,
    PlayerMode::Default(v) => v,
    PlayerMode::Dead => 0
  }  
}

// static dispatch
// can force a generic type to implement multiple traits with +
// pub struct GenericPlayer<H: Healable + Attackable>
// where clauses can be easier to read for adding more trait bounds
// generics help polymorphism at the type level
pub struct GenericPlayer<H : Healable> {
  health: H  // by generic param
}

// need <H: Healable> to flag that H is generic and not concrete
impl<H: Healable> GenericPlayer<H> {
  pub fn new(health: H) -> Self {
    Self {health}
  }
  
  // &impl uses generics but reduces the syntax
  // probably wouldn't normally mix the H and Healable normally, but just wanted a demo
  // can similarily add more bounds on the trait
  // pub fn trait_heal(&mut self, health: &impl Healable + Attackable) {i
  pub fn trait_heal(&mut self, health: &impl Healable) {
    self.heal(health.get());
  }


  pub fn heal(&mut self, amnt: usize) {
    self.health.heal(amnt);
  }
}

// can borrow T: &H
pub fn borrow_generic<H: Healable>(health: &H) -> usize {
  health.get()
}

impl<H: Healable> Display for GenericPlayer<H> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, 
      "GenericPlayer {{health: {}}}", self.health.get())
  }
}

pub trait Healable {
  fn heal(&mut self, amt: usize);
  fn get(&self) -> usize;
}

pub struct Health {
  health: usize
}

impl Health {
  pub fn new(health: usize) -> Self {
    Self {health}
  }
}

impl Healable for Health {
  fn heal(&mut self, amt: usize) {
    self.health += amt;
  }

  fn get(&self) -> usize {
    self.health
  }
}

// by trait type, using dynamic dispatch
// dyn means that the type is known at run time, so the size is not
// Sized trait is not implemented
// need to put it behind a reference &dyn to borrow, or in a Box that owns it
//   Box is allocated on the heap so it resolves the size conflict
// trait references help with polymorphism at the object level
pub struct TraitPlayer {
  health: Box<dyn Healable>    // like a reference to an abstract type
}

impl TraitPlayer {
  pub fn new(health: Box<dyn Healable>) -> Self {
    Self {health}
  }

  pub fn get(&self) -> &dyn Healable {     // also &mut dyn Healable is possible
    &*self.health
  }
}

pub trait Attackable {
  fn take_damage(&mut self, amnt: usize);
}

pub struct AttackedPlayer {
  health: usize
} 

impl Attackable for AttackedPlayer {
  fn take_damage(&mut self, amnt: usize) {
    self.health -= amnt;
  }
}

// simulates inheritance
trait PlayerBehaviors: Attackable + Healable {}

struct FullPlayer {
  health: usize
}

impl Attackable for FullPlayer {
  fn take_damage(&mut self, amnt: usize) {
    // fill in the implementation here
  }
}

impl Healable for FullPlayer {
  fn heal(&mut self, amt: usize) {
    // fill in the implementation here
  }
  fn get(&self) -> usize {
    // fill in the implementation here
    0
  }
}

impl PlayerBehaviors for FullPlayer {}

// references require a lifetime generic parameter
  // enums, traits, and functions can require them
// struct declares that the Healable ref must live at least as long as the struct
// can't have the borrow dropped by the owner while struct object still lives
// struct RefPlayer<'a, T> for another generic parameter
// cannot use '_ lifetime (any lifetime) here since they need to match
pub struct RefPlayer<'a> {
  health: &'a mut dyn Healable
}

impl<'a> RefPlayer<'a> { 
  pub fn new(health: &'a mut dyn Healable) -> Self {
    Self {health}
  } 

  // no need to add lifetime here bc of lifetime elison rules
  // basically it is assumed that the output should live as long as self
  pub fn get_health(&'a mut self) -> &'a mut dyn Healable {
    self.health
  }
}

// possible (likely) that the two references could have seperate lifetimes
// could be the same too
// functions with two lifetime params tend to follow elison rules where they
//   are unnecessary
pub struct TwoLifetimes<'a, 'b> {
  r1: &'a str,
  r2: &'b str
}

impl<'a, 'b> TwoLifetimes<'a, 'b> {
  pub fn new(r1: &'a str, r2:&'b str) -> Self {
    Self {r1, r2}
  }

  pub fn print_strs(&self) {
    println!("{}, {}", self.r1, self.r2);
  }

}


