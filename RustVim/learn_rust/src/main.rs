/*
 % cargo new my_lib --lib 
   creates Rust crate my_lib and /src folder for library crate
   can add main.rs with main() to make it
     an executable binary crate
   do the lib crate first for the rust analyzer visibility
 % cargo run --bin file 
   compile/run file.rs in /src/bin as seperate binary   i 
   each file needs added to [[bin]]
 % cargo run --bin my_lib 
   compiles and runs the main executable and compiles lib 
   replace run with build to only compile
 % cargo clean
   removes the executables and target dir
 This crate has a lib.rs, used to add files/folders as a library
 lib files/folders need declared by mod (module) in the binary 
     crate in main.rs and in lib.rs
   so that the lib file is visible in bin and lib crates
 types: u8, u16, u32, u64, u128 is unsigned 
        i8, i16, i32, i64, i128 is signed
        f32, f64 for floats
        bool for boolean
        char is 4 bytes
        isize, usize for size of machine word
*/

mod lib_one;   // declared for this main crate

// make lib_one code easier to call
use crate::lib_one::*;    // add on ::func for specific function/classes etc.
use std::convert::TryFrom;

fn main() { 

  // test initial exec and lib crates
  println!("Hello, world!");  //format string literal
  lib_test();                 // easy way with "use"
  crate::lib_one::lib_test(); // long way

  // variables
  // immutable by default, mut makes variable mutable
  let a = 10;
  println!("{}", a);  // with format string
  let a = 12;  // rebind/shadow old x in same/inner scope  
  let mut b = 11;    
  println!("{b}");   // for std types, primitive or obj
  b = 12;
  const NUM:u32 = 32;  // const requires type, immut
  println!("{a}, {b}, {NUM}");
  dbg!(a, b, NUM);  // prints var=val debug 1info 
  type BensType = u32; // type alias for long types 
  let _c:BensType = 41;  // underscore means unused  
  // static/global variable have static lifetime
  // accessable anywhere in the crate
  // unsafe to read or write (if mut)
  static mut Y:bool = true;
  unsafe{Y = false};
  
  // as is primitive cast, may lead to data loss, no error check
  // as will never panic
  let inum:isize = 10;
  let unum:usize = inum as usize;
  println!("{unum}");

  // safer cast with error checking
  // try_from() returns Result type to force error checking
  // unwrap() extracts the value or throws error  
  // expect("error msg") is the same with custom msg
  let ic:isize = 10;
  let uc:usize = usize::try_from(ic).unwrap();
  println!("{uc}");

  // string literals are fast safe, not on heap or stack
  // good for constant fast read-only data, borrow
  // ref on stack, uses slice
  // s is a ref to str, &s is a ref to the ref
  // can't deref &str since the size of the str is unknown
  // may have static lifetime: &`static str
  let s:&str = "Ben";
  println!("{s}");      // fmat string auto deref and prnts value 
  println!("{s:p}");    // prints rf info
  
  // x is val, &x is a ref to x
  let x:i32 = 300;
  let xref:&i32 = &x;
  println!("{x}, {xref}");            // prints vals by auto-deref
  println!("{xref:p}, {:p}", xref);   // prints refs two ways
  println!("{:p}", &x);               // only way to print ref by temp val 
  println!("{}", *xref);              // only way to print val by temp deref
  
  // String is mutable, data allocated to the heap, like Vec<u8>
  // ref is like a struct of pointer, capacity, length on stack
  // :: namespace operator
  // == always derefs to vals, no matter the & or * aritmetic
  let mut s = String::from("Ben");
  let s2 = String::from("Mags");
  s.push_str(" Harki");        // has methods
  println!("{s} has {} chars with capacity {}", 
    s.len(), s.capacity());
  println!("{}", s == s2);     // false, derefs to vals and checks
  let s3:String = format!("{} and {}", s, s2);  
  println!("{s3}");
  println!("{:p}", &s);        // pointer in the ref or fat pointer
  let b:bool = std::ptr::eq(&s, &s2);  
  println!("{}", b);           // false since different pointers
  let s4:&str = &s;            // can coerce to &str, not other way around
  println!("{}", s4);

  // tuples are easy groups of mixed types in memory
  // useful in pattern matching, returns from functions
  let t:(i32, char) = (34, 'a');
  let (a, b) = t;            // unpack to variables quickly
  println!("{a} and {b}");

  // arrays are of fixed size and same type, panic checks index
  // cannot print vals  through var, no Display trait impl, use Dbg
  // array on stack
  // indices and len() need to be usize or cast to it
  let mut arr:[isize; 4] = [0, 1, 2, 3];       // array and var are mut     
  let arr2:[isize; 25] = [0; 25];              // 25 0's
  println!("{}, {}", arr.len(), arr2.len());   // does have len
  println!("{}", arr[0]);                      // indexing
  arr[1] = 100;                                // change by asgn 
  dbg!(arr);                                   // debug 
  println!("{:?}", arr);                       // also debug
  println!("{:p}, {:p}", &arr, &arr[0]);       // ref is 0th  
 
  // mutable variable and mutable value
  // can only have of one mut ref to a place in mem (error doesn't detect until read or write)
  let mut ival:&mut isize = &mut 10;   // temp rval is promoted to stack
  let ival2:&mut isize = &mut 11;
  println!("{ival}, {ival2}");
  *ival = 12;
  *ival2 = 13;
  // ival = ival2;                 // cannot have two mut& refs
  // println!("{ival}, {ival2}");  // triggers the error on prev line

  // now ival points to ival2, as long as ival2 has no read/write again 
  ival = ival2;           
  println!("{ival}");

  // once &mut is declared, any older refs are now invalid 
  //   no matter if they are & or &mut
  // Any & declared after &mut invalidates the &mut    
  let mut mval:isize = 10;
  #[allow(unused_variables)]  
  let rval:&isize = &mval;
  #[allow(unused_variables)]  
  let rmval:&mut isize = &mut mval;
  // println!("{rval}");      // previous is invalid
  #[allow(unused_variables)]  
  let rmval2:&mut isize = &mut mval;
  // println!("{rmval}");     // previous is invalid
  let rval2:&isize = &mval;
  // println!("{rmval2}");    // previous is invalid
  println!("{rval2}");
  

  // can have any number of shared reference, since readonly
  let ival3:&isize = &14;
  let ival4:&isize = ival3;
  let ival5:&isize = ival4;
  println!("all hold {:p} -> [{}]", ival3, ival3);
  println!("all hold {:p} -> [{}]", ival5, ival5);
 

  // read only iterator
  // iterator may Move container if it is a Move type
  // Arrays are Copy or Move, depending on the type they contain
  // isize is a Copy type, so it doesn't Move
  for a in arr {
    println!("{a}");
  }
  println!("{}", a);    // a is still alive

  // borrow by ref/slice, Copy on ref is the same as borrow
  // prints refs and vals, print auto derefs to vals, can use deref *
  // AI says this is the fastest iterator?
  for a in &arr {                         // can drop & for vals only
    if *a == 100 {                        // dref for cond check
      println!("100 is found");
    }
    println!("{} in {:p}", a, a);  
  }
  println!("{a}");   // a is still alive
  
  
  // there is a mutable iterator, but only as borrow with &
  // &mut arr means array vals can change
  for a in &mut arr {
    *a += 10;
  }
  println!("{:?}", arr);

  // &a version, more like a pattern match 
  // only work on Copy type containers, no Move
  // address of the local loop variable 
  // not mut& arr version
  for &a in &arr {
    println!("{:p}: {}", &a, a);    
  }
  
  // iterator on a true Move type [String]
  let sarr:[String; 3] = [
    String::from("Ben"),
    String::from("Mags"),
    String::from("Will")];
  for s in sarr {
    println!("{s}");
  }
  // println!("{:?}", sarr);  // sarr Move out, so panics

  // Move types, when borrowed with &, will not move
  let nm:String = String::from("Will");
  // let mm:String = nm;   // mving does not allow next borrow 
  let nmref:&String = &nm; 
  let nmref2:&String = nmref;
  println!("{nm}, {nmref2}");
  
  // .clone may help with Move types, deep copy
  // technically a &, immutable borrow, refs always Copy
  let mm:String = nm.clone();
  println!("{nm}, {mm}");    // nm is not invalid


  // allocate on the heap with box, initialized (can pick uninit in unsafe{})
  // give new() an array initializer
  // also an index based loop
  // buf is a ref to that holds the pointer to the array on the heap
  // autoderefs the ref to the location on stack, no need for *, impl Deref
  // 0..=10 includes 10, 0.. goes to the end 
  const N:usize = 10;
  let mut buf:Box<[isize; N]> = 
    Box::<[isize; N]>::new([0; N]);       // don't need rhs [isize;N] 
  for i in 0..10 {
    buf[i] = 1;   // no need to use *
  }
  println!("{:?}", buf);

  // read only iterators
  // borrow so Copy on ref
  for v in &*buf {     // * derefs from box
    println!("{:p}", v);
  }
  for v in *buf {     // could move, but isize is a Copy type, so arr is also Copy
    print!("{}", v);
  }
  println!("\n{:p}", buf);

  // mutable iterator on Box 
  // only possible to iterate mutably and borrow
  for v in &mut *buf {
    *v += 1; 
  }
  println!("{:?}", buf);

  // heap pointer to Move type
  let mut sbuf:Box<[String; 5]> = Box::new( 
    [String::new(),
     String::new(),
     String::new(),
     String::new(),
     String::new()]);
  let st:&str = "abcde";
  let mut st:std::str::Chars<> = st.chars();    // char iterator
  for s in &mut *sbuf {          // borrow iterator
    s.push(st.next().unwrap());  // each item unwrapped from Some
  }
  // Moves sbuf array
  for s in *sbuf {
    println!("{s}");
  }
  // println!("{:?}", sbuf);  // ref invalid


  // statements end in ; and do not return a value
  // exprs come from calling macro, funct, if, or anything w/ {}
  let y = {let x = 1; x + 1};     // expr return 2 in y assign stmt
  println!("{y}");    // 2
  
  // pattern matching on arrays, _ is wildcard
  // can match on Some/None (Option) or Err/Ok (Result)
  let mut arr2:[usize;10] = [0;10];
  for i in 1..10 {
    arr2[i] = i + arr2[i - 1];
  }
  println!("{:?}", arr2);
  let tp:(usize, usize, usize) = match arr2 { 
    [_, f, s, .., l] => (f, s, l)
  };
  println!("{:?}", tp);
 
  // call on mutable array ref
  let arr2ref:&mut [usize] = null_out(&mut arr2);     
  println!("{:?}", arr2ref);
  for v in &mut *arr2ref {
    *v = 1;
  }
  println!("{:?}", arr2ref);
  null_out_ind(arr2ref, 0);
  println!("{:?}", arr2ref);

  // function that Moves on Move type
  let mv:String = String::from("move this");
  mymove(mv);
  // println!("{mv}");    // string has been moved

  // placeholder function to implement later
  donothing(10);
 
  // loop block that returns with a break
  // return is assigned to let var statement 
  // continue can also be used
  println!("{:?}", arr);
  let mut i:usize = 0;
  let cnt:usize = loop {
    if i >= arr.len() {
      break i;
    }
    arr[i] += 1;
    i += 1;
  };
  println!("{:?} has count {}", arr, cnt);


  // Option<T> is Some(T) or None, Rust's null replacement
  // handle by using if let or unwrap_or
  // can handle by pattern match and many other methods, some panic on None
  // unwrap_or(0) returns val from Some(val) or 0 on None 
  // works on any Enum include Ok/Err (Result)
  let oarr: [Option<isize>; 5] = [Some(1), None, Some(3), None, Some(5)];
  for o in oarr {
    if let Some(v) = o {
      println!("if let got: {v}");
    }
    else {
      println!("hit None");
    }
  }
  for o in oarr {
    let v: isize = o.unwrap_or(0);
    println!("unwrap_or got: {v}");
  }

  // while let loops as long as pattern matches Some(v)
  // loops until extract None on wval after last iteration
  // if expression can be transformed into an assignment
  let mut wval: Option<isize> = Some(3);
  while let Some(v) = wval {
    println!("while let got: {v}");
    wval = 
      if v > 0 { 
        Some(v - 1) 
      } 
      else { 
        None 
      };
  }

  // move String from st, change it,  back to st
  let mut st = String::from("hello");
  st = change(st);
  println!("{st}");

  // function modifies Move type, by borrow/Copy ref
  // add_last borrow st3, so it is not moved
  let mut st3:String = String::from("Ben");
  add_last(&mut st3);
  println!("{st3}");
  
  // slice can use 0..10 ranges
  // String can implicitly convert (coerce) to &str  
  // adding a & or &mut to a contiguous type makes it a slice
  let myname:String = String::from("Ben Harki");
  println!("{}", &myname[4..]);


  // STOPPED ON REFERENCE/BORROW
  // TODO:  
  //       understand RC pointers
  //       struct with a Move type in a field 
  //         on move, can destroy the object
  //       famous lifetime example: ref in outer {}
  //         assign val in inner{} fails in rust
  //         can fix with a box, or mut var in outer
} 
