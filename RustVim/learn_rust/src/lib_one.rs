// pub makes it visible by other crates
// by default code is only visible within the same module
pub fn lib_test() {
  println!("Hello from lib one again!");
}

// function with mutable parameters and return
// suppress some warnings by attributes
// returns by expression and can also use return stmnt
pub fn null_out(arr:&mut [usize]) -> &mut [usize] {
  for v in &mut *arr {
    *v = 0;  
  }
  arr
}

// parameter can be mutable
// 0 is not of usize so needed isize cast
pub fn null_out_ind(arr:&mut [usize], mut i:isize) {
  while i < (arr.len() as isize) && i >= 0 {
    arr[i as usize] = 0;
    i += 1;
  }
}
