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

// set up functions to implement later
// attributes suppress warnings 
#[allow(unused_variables)]
#[allow(dead_code)]
pub fn donothing(i:usize) {
  // implement later
}
