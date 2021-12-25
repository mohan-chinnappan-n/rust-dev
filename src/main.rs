fn main() {
    println!("Hello, world!");
    println!("{}", add(1000,20));

    let fruit = "apple";
    let my_fruit = if fruit == "apple" { true } else { false};
    println!("{} is vaule of my_fruit", my_fruit);

    // while 
    liftoff(5);

    // array
    println!("{} is my requested fruit", favfruits(2));

    // string literal - immutable - directly hardcoded in the final exe
    let state = "NH";
    println!("state is {}", state);


    // String mutation - size can change during the runtime of the exe
    //  mutable, growable amount of memory on the heap
    //  size is unknown at the compile time

    // 1. Memory allocator has to be used at runtime
    // 2. We need a way to give back this memory to the memory allocator once we are done with that memory

    // 1. String::from does the request to the memory allocator
    // 2. so we neeed to one request and one free

    // the memory is automatically returned once the variable that owns the memory goes out of scope


    {
        let mut osname = String::from("Unix"); // osname is valid from this point forward
        osname.push_str (", BSD");
        println!("osname is {}", osname);
    } // this scope is now over and osname is no longer valid
      // rust calls a special function called drop, it is where the author of the String can put the code 
      //   to return the memory. rust calls drop automatically at this closing curly bracket


    /* =======
    you will get this error: if used osname outside of this scope
38 |     println!("osname is {}", osname);
   |                              ^^^^^^ not found in this scope
    ============== */
    // println!("osname is {}", osname);
 

  // pointer repointing

  let s1 = String::from("rust");
  let s2 = s1;
  println!("s2 = {}", s2);
  // s1 is no longer valid now
  /* ===
  println!("value of s1: {}", s1); // println!("value of s1: {}", s1);
  |                               ^^ value borrowed here after move

  // Rust prevents us from using the invalidated reference
  // Rust invalidates the first variable, instead of being called a shallow copy, it is known as move
  // so here, s1 is moved into s2

  // so when s2 goes out of scope, it alone will free the memory, we are done!

  // if we want deeply copy the heap data of the String, instead of just stack data (ptr, len, capacity)
  //   we can use a common method caled clone
  // 
  

  === */

  let str1 = String::from("Unix");
  let str2 = str1.clone();

  println!("str1 = {}, str2 = {}", str1, str2);

}

fn add(a: i32, b:i32) -> i32 {
    if a > 100 {
        println!("{} is greater than 100", a);
    }
    return a + b;
}

fn liftoff (c: i32) {
    let mut count = c;
    while count > 0 {
        println!("Lifting {}...", count);
        count -= 1;
    }
    println!("Lifting done!");
}

fn favfruits (i: usize) -> String {
    let fruits: [String; 4] = ["apple".to_string(), "mango".to_string(), "peach".to_string(), "pear".to_string() ];
    
    // let fruits: [String; 4] = ["apple", "mango", "peach", "pear" ];

    return fruits[i].to_string();
}