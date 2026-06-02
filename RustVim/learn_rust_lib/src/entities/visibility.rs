// pub is visible to any outside crate, like the executable crate
// default (private) is visible only to the module (file here)

pub struct Enemy {
  health: u32
}

impl Enemy {
  pub fn new(health: u32) -> Self {
    Self {health}
  }
  
  pub fn get(&self) -> u32 {
    self.health
  }

  // like a static function
  pub fn demo() {
    demo();
  }
  
}

// only visible in this crate, hidden from exec crate
// alternatives: pub(super), pub(in crate::entities) 
//   makes visible up to various different module levels
//   also can contol larger modules in module.rs files
pub(crate) fn demo() {
  let e1: Enemy = Enemy::new(50);
  println!("demo: {}", e1.health);     // still visible even outside the struct
}


