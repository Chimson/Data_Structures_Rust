// pub makes it visible by other crates
// by default code is only visible within the same module
pub fn lib_test() {
  println!("Hello from lib one again!");
}

// function with mutable parameters and return
// suppress some warnings by attributes
// returns by expression and can also use return stmnt
// refs Copy which is the same as "borrow"
pub fn null_out(arr:&mut [usize]) -> &mut [usize] {
  for v in &mut *arr {
    *v = 0;  
  }
  arr
}

// parameter can be mutable
// like a void return, but returns () unit type implicitly
pub fn null_out_ind(arr:&mut [usize], mut i:usize) {
  while i < arr.len() {
    arr[i] = 0;
    i += 1;
  }
}

// functions can Move on a Move type
pub fn mymove(msg:String) { 
  println!("{msg}");
}

// can return a Move type to another var, or back to old
pub fn change(s:String) -> String {
  s + " world"
}

// to modify Move type, need to use mutable reference
// not enough to make the variable mut
// do not need to remove, since its not moved
//   borrowed by ref
pub fn add_last(first:&mut String) {
  first.push_str(" Harki");
}

// cannot return any local var ref bc dropped from stk
// fix by accepting a ref and passing it through, using a box, 
//   or pass by value
// pub fn dangle() -> &isize {
  // let val:isize = 10;
  // return &val;
// }

// set up functions to implement later
// attributes suppress warnings 
#[allow(unused_variables)]
#[allow(dead_code)]
pub fn donothing(i:usize) {
  // implement later
}
