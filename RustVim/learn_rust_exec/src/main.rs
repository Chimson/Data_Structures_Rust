/*
 Each crate is seperately compilable: 
   so make an executable crate and a library crate
 % cargo new learn_rust_lib --lib 
   creates Rust library crate 
   creates src/ and src/lib.rs and more
   lib.rs contains direct module declarations 
   do the lib crate first for the rust analyzer visibility
 % cargo new learn_rust_exec 
   creates an executable crate with main.rs in /src
   can add a /bin folder for multiple executable files
     add [[bin]] line to cargo.toml to link them
   link a local library crate:
     % cargo add learn_rust_lib --path ../learn_rust_lib
     adds dependencies to cargo.toml
 % cargo build
   builds either crate
 % cargo run --bin learn_rust_exec   // main.rs executable
 % cargo run --bin otherexec  // other executable in /bin
 % cargo clean
   removes the executable binary files and target dir
 modules are any collection of code, files, folders of files, etc.
 library structure with multiple modules:
   src/
     lib.rs - mod declarations of direct child modules in src
     lib_one.rs - standalone module (file)
     entities.rs - mod declarations of modules in entities
     entities/
       player.rs - file module
         pub struct Player

 types: u8, u16, u32, u64, u128 is unsigned 
        i8, i16, i32, i64, i128 is signed
        f32, f64 for floats
        bool for boolean
        char is 4 bytes
        isize, usize for size of machine word
*/

/*
Rust Philosophy:
Move types like Box<>, structs by default, String can move ownership to another variable
They drop when out of scope if not moved
Copy types include References (& &mut) and primitive values (including &str)
References borrow a value and do not own them
Copy types cannot drop since they do not own
Arrays take on the Copy or Move of the type they are an array of
*/


use std::convert::TryFrom;
use std::rc::Rc;


// "use" makes members from modules easier to call
// pathed as:
//   use crate::module_as_folder::...::module_as_file::public_member;
//   last name on the right of use statement is directly usable
//   can stop at any module for use statement; adds namespace prefixes when used
// can either add members individually or take off lib_one and add module prefixed
use learn_rust_lib::lib_one::*;   
use learn_rust_lib::entities::visibility;   // contains Enemy
use learn_rust_lib::entities::player::*;
// use learn_rust_lib::entities::player::Player;
// use learn_rust_lib::entities::player::GenericPlayer;
// use learn_rust_lib::entities::player::Health;
// use learn_rust_lib::entities::player::TraitPlayer;

fn main() { 

  // test initial exec and lib crates
  println!("Hello, world!");  //format string literal
  lib_test();                 // easy way with "use"
  learn_rust_lib::lib_one::lib_test(); // long way

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
  // expect("error msg") isthe same with custom msg
  let ic:isize = 10;
  let uc:usize = usize::try_from(ic).unwrap();
  println!("{uc}");

  // string literals are fast safe, not on heap or stack
  // good for constant fast read-only data, borrow
  // ref on stack, uses slice
  // s is a ref to str, &s is a ref to the ref
  // can't deref &str since the size of the str is unknown
  // &str is the default read only string literal type
  //   encoded in UTF-8 uses 1-4 u8 values per character
  //   &mut str is unsafe but sometimes possible, not used often
  // may have static lifetime: &`static str for full duration of program
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
  // generally only deref on move types when moving out containing value
  // like Box<>

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
  // once a &mut ref is introduced its more complicated 
  //   (shown in later examples but not here)                  
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

  // function accepting a Move type
  // Not moved to a new owner so it drope
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
  // ?, in a function, propgates the None, or Err back up form calling code (not shown)
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
  
  // slice can use 0..10 ranges, always through a & borrow
  // String can implicitly convert (coerce) to &str  
  // adding a & or &mut to a contiguous type makes it a slice
  let myname:String = String::from("Ben Harki");
  println!("{}", &myname[4..]);

  // from structs, through object value
  // can call methods statically (not shown)
  let mut p1: Player = Player::new(String::from("Ben"), 100);   // on stack
  println!("{p1}");
  // println!("{}", p1.name);   // name is private field
  p1.set_name(String::from("Mags"));
  p1.get_name();
  let p1ref: &mut Player = &mut p1;  // ref can't change, player object can
  p1ref.set_name(String::from("Adam"));
  let _mp1 = p1;
  // p1.get_name();   // custom structs by default are Move type (unless they impl Copy trait)
  
  // various prints of objects generated by #[derive(debug)]
  println!("{_mp1:?}");
  println!("{_mp1:#?}");
  dbg!(&_mp1);    // prints to stderr with line file/line number


  // visibility demos
  let e1: visibility::Enemy = visibility::Enemy::new(100);
  println!("{}", e1.get());
  visibility::Enemy::demo();    // calls a crate private function 

  // allocate an object on the heap
  // with the generic type in variable type declaration
  let mut p2: Box<Player> =
    Box::new(Player::new(String::from("Ben") , 100));
  p2.set_name(String::from("XXX"));
  
  // with the generic type but not type declaration
  // doing both would be extremely obnoxious
  let p3 = Box::<Player>::new(Player::new(String::from("Ben"), 100));
  p3.get_name ();
  
  // Box is a Move type and owns the pointer
  let p4:Box<Player> = p2;   // Box moved to p4
  // p2.get_name();  
  p4.get_name();
  
  // rust suggests to borrow T directly from Box<T>


  let p4ref: &Player = &p4;    // auto deref
  p4ref.get_name();

  // Ownership is about dropping objects when out of scope
  // Can Move to other owners but only one exact owner
  // Certain data structures may require multiple owners of same Box (like a graph)
  //   may be able to get around this with a larger Box allocation
  //   like a Box array of nodes, with a mem manager, instead of individual boxes per node
  // use a clone like Rc? no
    // Box clones only use deep copy: a shallow copy allows a double free, which is forbidden
  let p5: Box<Player> = Box::new(Player::new(String::from("drop"), 100));
  Player::demo_drop_box(p5);    
  // p5.get_name();

  // Rc allows multiple owners through a shallow copy clone
  // Rc keeps track of a reference count 
  // each variable still owns the pointer, but now can have multiple owners
  // multiple owners of an object may be helpful in certain data structure situations
  //   graph with two edges that contain the same node
  //   just adjusting lifetimes wouldn't work since could have cycles and then infinite lifetimes
  // Rc is immutable
  // could cause memory leaks: cycles cause values to never be dropped, ref count is never 0
  let p6: Rc<Player> = Rc::new(Player::new(String::from("does not drop"), 100));
  Player::demo_rc(p6.clone());
  p6.get_name();
  // p6.set_name(String::from("X"));    

  // demo generic trait bound
  let mut g: GenericPlayer<Health> = 
    GenericPlayer::new(Health::new(100));
  g.heal(10);
  
  // demo on the &impl version of generics
  g.trait_heal(&Health::new(100));
  println!("{g}");

  // dynamic dispatch demo coded to the Trait
  // 
  let tp: TraitPlayer = 
    TraitPlayer::new(
      Box::new(
        Health::new(100)));
  
  println!("{}", tp.get().get());   // not great code, but demos the dyn type
  
  let mut h: Health = Health::new(100);
  let mut r: RefPlayer = RefPlayer::new(&mut h);
  let mut h2: Health = h;
  // r.get_health(); violates the lifetime contract since h is dropped
  
  // two lifetimes example, elison rules does not cover
  let tl: TwoLifetimes = TwoLifetimes::new("Ben", "Harki");
  tl.print_strs();

  // small unsafe operation, without raw pointers
  // &str is a slice to UTF-8 slice, where each character is represented by
  //   1-4 u8 vals
  // you can have any number of immutable borrows except ...
  //   that once a mutable borrow is alive you can't have ..
  //   any more new mutable or new immutable borrows     
  // one fix is to hide/drop the old borrows and create new ones each time, 
  //   which you could write in a function? 
  // another fix is to clone name if you have the resources
  let mut name: String = String::from("Ben");
  let char_ptr: &mut u8 = unsafe {&mut (name.as_bytes_mut()[0])};
  *char_ptr = 0x43;
  println!("{:?}", name);
  let char_ptr: &mut u8 = unsafe {&mut (name.as_bytes_mut()[0])}; 
  *char_ptr = 0x43;
  println!("{}", name);

  // unsafe with raw pointers
  // uses raw pointers
  let mut name: String = String::from("Ben");
  let mut name_ptr: *mut u8 = name.as_mut_ptr();
  unsafe {
    *name_ptr += 1;
    name_ptr = name_ptr.add(1);
    *name_ptr += 1;
  } 
  println!("{}", name);

  // TODO:     
  //      macros
} 
