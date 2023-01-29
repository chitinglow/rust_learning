#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
mod restaurant;
use crate::restaurant::order_food;

// input output
// fn main() {
//     println!("What is your name?");
//     let mut name = String::new();
//     let greeting = "Nice to meet you";
//     io::stdin()
//         .read_line(&mut name)
//         .expect("Didn't Receive Input");
//     println!("Hello {}! {}", name.trim_end(), greeting);
// }

// type of data
// fn main() {
//     const ONE_MIL: u32 = 1_000_000;
//     const PI: f32 = 3.141592;
//     let age: &str = "47";
//     let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
//     age = age + 1;
//     print!("I'm {} and I want ${}", age, ONE_MIL);
// }

// fn main() {
//     // Unsigned integer : u8, u16, u32, u64, u128, usize
//     // unsigned integer : i8, i16, i32, i64, i128, isize

//     println!("Max u32: {}", u32::MAX);
//     println!("Max u64: {}", u64::MAX);
//     println!("Max usize: {}", usize::MAX);
//     println!("Max u128: {}", u128::MAX);
//     println!("Max f32: {}", f32::MAX);
//     println!("Max f64: {}", f64::MAX);

//     let is_true = true;
//     let is_false = false;

//     let my_grad = 'A';
//     let num_1: f32 = 1.11111111111111;
//     let num_2: f64 = 1.11111111111111;
//     let num_3: u32 = 5;
//     let num_4: u32 = 4;

//     println!("5 + 4 = {}", num_3 + num_4);
//     println!("5 - 4 = {}", num_3 - num_4);
//     println!("5 * 4 = {}", num_3 * num_4);
//     println!("5 / 4 = {}", num_3 / num_4);
//     println!("5 % 4 = {}", num_3 % num_4);

//     println!("f32:{}", num_1 + 0.1111111111111);
//     println!("f64:{}", num_2 + 0.1111111111111);
// }

// generate random numbers

// fn main() {
//     let random_num = rand::thread_rng().gen_range(1..101);
//     println!("Random: {}", random_num);

//     let age = 8;

//     if (random_num >= 1) && (random_num <= 18) {
//         println!("Important birthday!")
//     } else if (random_num == 21) || (random_num == 50) {
//         println!("Important birthday!")
//     } else if random_num >= 65 {
//         println!("Important birthday!")
//     } else {
//         println!("Not and important birthday!")
//     }
// }

// // if condition
// fn main() {
//     let mut my_age = 47;
//     let can_vote = if my_age >= 18 { true } else { false };

//     println!("Can vote : {}", can_vote);
// }

// match
// fn main() {
//     let age2 = 8;

//     match age2 {
//         1..=18 => println!("Important birthday"),
//         21 | 50 => println!("Important birthday"),
//         65..=i32::MAX => println!("Important birthday"),
//         _ => println!("Not an important birthday"),
//     }
// }

// fn main() {
//     let my_age = 18;
//     let voting_age = 18;

//     match my_age.cmp(&voting_age) {
//         Ordering::Less => println!("Can't vote"),
//         Ordering::Greater => println!("Can vote"),
//         Ordering::Equal => println!("Can vote"),
//     };
// }

// Array and loop
// fn main() {
//     let arr_1 = [1, 2, 3, 4];

//     println!("1st : {}", arr_1[0]);
//     println!("Length : {}", arr_1.len());

//     let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];

//     let mut loop_idx = 0;

//     for val in arr_2.iter() {
//         println!("val: {}", val);
//     }

//     // loop {
//     //     if arr_2[loop_idx] % 2 == 0 {
//     //         loop_idx += 1;
//     //         continue;
//     //     }
//     //     if arr_2[loop_idx] == 9 {
//     //         break;
//     //     }

//     //     println!("Val : {}", arr_2[loop_idx]);
//     //     loop_idx += 1;
//     // }

//     // while loop_idx < arr_2.len() {
//     //     println!("Val : {}", arr_2[loop_idx]);
//     //     loop_idx += 1;
//     // }
// }

// tuple
// fn main() {
//     let my_tuples: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
//     println!("Name: {}", my_tuples.1);

//     let (v1, v2, v3) = my_tuples;
//     println!("Age: {}", v1)
// }

// string

// fn main() {
//     let mut st1 = String::new();

//     st1.push('A');
//     st1.push_str("word");

//     for word in st1.split_whitespace() {
//         println!("{}", word);
//     }

//     let st2 = st1.replace("A", "Another");
//     println!("{}", st2);

//     let st3 = String::from("x r t h b k k a m c");

//     let mut v1: Vec<char> = st3.chars().collect();
//     v1.sort();
//     v1.dedup();
//     for char in v1 {
//         println!("{}", char);
//     }

//     let st4 = "Random string";
//     let mut st5 = st4.to_string();
//     println!("{}", st5);

//     let byte_arr1 = st5.as_bytes();
//     let st6 = &st5[0..6];
//     println!("{}", st6.len());

//     st5.clear();

//     let st6 = String::from("Just Some");
//     let st7 = String::from(" words");
//     let st8 = st6 + &st7;

//     for char in st8.bytes() {
//         println!("{}", char);
//     }
// }

// Casting
// fn main() {
//     let int_u8: u8 = 5;
//     let int2_u8: u8 = 4;
//     let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

//     enum Day {
//         Monday,
//         Tuesday,
//         Wednesday,
//         Thursday,
//         Friday,
//         Saturday,
//         Sunday,
//     }

//     impl Day {
//         fn is_weekend(&self) -> bool {
//             match self {
//                 Day::Saturday | Day::Sunday => true,
//                 _ => false,
//             }
//         }
//     }

//     let today: Day = Day::Monday;
//     match today {
//         Day::Monday => println!("Everyone hates Monday"),
//         Day::Tuesday => println!("Everyone hates Tues"),
//         Day::Wednesday => println!("Everyone hates Wed"),
//         Day::Thursday => println!("Everyone hates Thur"),
//         Day::Friday => println!("Everyone hates Fri"),
//         Day::Saturday => println!("Weekend"),
//         Day::Sunday => println!("Weekend"),
//     }

//     println!("Is today the weekend {}", today.is_weekend());
// }

// // Vector

// fn main() {
//     let vec1: Vec<i32> = Vec::new();

//     let mut vec2 = vec![1, 2, 3, 4];
//     vec2.push(5);
//     println!("1st : {}", vec2[0]);
//     let second = &vec2[1];

//     match vec2.get(1) {
//         Some(second) => println!("2nd : {}", second),
//         None => println!("No 2nd value"),
//     }

//     for i in &mut vec2 {
//         *i *= 2;
//     }
//     for i in &vec2 {
//         println!("{}", i);
//     }

//     println!("Vec Length {}", vec2.len());
//     println!("Pop : {:?}", vec2.pop());
// }

// Function
// fn say_hello() {
//     println!("Hello");
// }

// fn get_sum(x: i32, y: i32) {
//     println!("{} + {} = {}", x, y, x + y);
// }

// fn get_sum2(x: i32, y: i32) -> i32 {
//     x + y
// }

// fn get_sum3(x: i32, y: i32) -> i32 {
//     return x + y;
// }

// fn get_2(x: i32) -> (i32, i32) {
//     return (x + 1, x + 2);
// }

// fn sum_list(list: &[i32]) -> i32 {
//     let mut sum = 0;
//     for &val in list.iter() {
//         sum += val;
//     }

//     sum
// }

// fn main() {
//     say_hello();
//     get_sum(5, 4);
//     println!("{}", get_sum2(5, 4));
//     println!("{}", get_sum3(5, 4));

//     let (val1, va2) = get_2(3);
//     println!("Nums : {} {}", val1, va2);

//     let num_list = vec![1, 2, 3, 4, 5];

//     println!("Sum of list = {}", sum_list(&num_list));
// }

// generic

// use std::ops::Add;

// fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
//     return x + y;
// }

// fn main() {
//     println!("5 + 4 = {}", get_sum_gen(5, 4));
//     println!("5.2 + 4.6 = {}", get_sum_gen(5.2, 4.6));
// }

// Ownership

// fn print_str(x: String) {
//     println!("{}", x);
// }

// fn print_return_str(x: String) -> String {
//     println!("{}", x);
//     x
// }

// fn change_string(name: &mut String) {
//     name.push_str(" is happy");
//     println!("{}", name);
// }

// fn main() {
//     let mut str1 = String::from("Derek");
//     //let str2 = str1;
//     // let str2 = str1.clone();
//     // println!("Hello {}", str1);
//     // print_str(str1);
//     // let str3 = print_return_str(str1);
//     // println!("Hello {}", str3);

//     change_string(&mut str1);
// }

// Hashmap
// fn main() {
//     let mut heroes = HashMap::new();

//     heroes.insert("Superman", "Clark Kent");
//     heroes.insert("Batman", "Bruce Wayne");
//     heroes.insert("The Flash", "Barry Allen");

//     for (k, v) in heroes.iter() {
//         println!("{} = {}", k, v);
//     }

//     if heroes.contains_key(&"Batman") {
//         let the_batman = heroes.get(&"Batman");

//         match the_batman {
//             Some(x) => println!("Batman is a hero"),
//             None => println!("Batman is not a hero"),
//         }
//     }
// }

// Struct

// fn main() {
//     const PI: f32 = 3.141592;
//     // struct Customer {
//     //     name: String,
//     //     address: String,
//     //     balance: f32,
//     // }

//     // let mut bob = Customer {
//     //     name: String::from("Bob Smith"),
//     //     address: String::from("123 Main St"),
//     //     balance: 234.50,
//     // };

//     // bob.address = String::from("456 Main St");
//     // println!("{}", bob.address);
//     trait Shape {
//         // This is a constructor which returns a Shape
//         fn new(length: f32, width: f32) -> Self;

//         // An area function that belongs to this trait
//         fn area(&self) -> f32;
//     }

//     struct Rectangle {
//         length: f32,
//         width: f32,
//     }
//     struct Circle {
//         length: f32,
//         width: f32,
//     }

//     // Implement the trait for rectangle
//     impl Shape for Rectangle {
//         // Constructor
//         fn new(length: f32, width: f32) -> Rectangle {
//             return Rectangle { length, width };
//         }

//         // self allows us to refer to parameters for this struct
//         fn area(&self) -> f32 {
//             return self.length * self.width;
//         }
//     }

//     // Implement the trait for circle
//     impl Shape for Circle {
//         // Constructor
//         fn new(length: f32, width: f32) -> Circle {
//             return Circle { length, width };
//         }

//         fn area(&self) -> f32 {
//             return (self.length / 2.0).powf(2.0) * PI;
//         }
//     }

//     // Create circle and rectangle with Shape
//     let rec: Rectangle = Shape::new(10.0, 10.0);
//     let circ: Circle = Shape::new(10.0, 10.0);

//     println!("Rec Area : {}", rec.area());
//     println!("Circ Area : {}", circ.area());
// }

// Packages

// fn main() {
//     order_food();
// }
// fn main() {
//     // error handling and file reading
//     // ----- READING & WRITING TO FILES & ERROR HANDLING -----
//     // Rust doesn't have exceptions like other languages. It handles
//     // recoverable errors with Result and the panic! macro for
//     // unrecoverable errors

//     // When the panic! macro executes your program prints an error
//     // memory is cleaned up and the program quits
//     // panic!("Terrible Error");

//     // Accessing an index that doesn't exist calls panic
//     // let lil_arr = [1,2];
//     // println!("{}", lil_arr[10]);

//     // File to create
//     let path = "lines.txt";

//     // Result has 2 varients Ok and Err
//     // enum Result<T, E> {
//     // Ok(T),
//     // Err(E), }
//     // Where T represents the data typeof the value returns and E
//     // the type of error

//     // Create file and handle errors with match
//     let output = File::create(path);
//     let mut output = match output {
//         Ok(file) => file,
//         Err(error) => {
//             panic!("Problem creating file : {:?}", error);
//         }
//     };

//     // Write to file and define the panic! error message with expect
//     write!(output, "Just some\nRandom Words").expect("Failed to write to file");

//     // Open the file and if everything is ok unwrap returns the file
//     // and if not panic! triggers an error (You could replace unwrap with ?)
//     // Read file using buffering
//     let input = File::open(path).unwrap();
//     let buffered = BufReader::new(input);

//     // Cycle through and print the lines
//     for line in buffered.lines() {
//         println!("{}", line.unwrap());
//     }

//     // You can also catch specific errors
//     // Here I'll try to open a file and trigger an error if the file
//     // couldn't be created, or use a default
//     let output2 = File::create("rand.txt");
//     let output2 = match output2 {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("rand.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Can't create file: {:?}", e),
//             },
//             _other_error => panic!("Problem opening file : {:?}", error),
//         },
//     };
// }
// ----- ITERATORS -----
// We covered iterators before. They help us cycle through values in
// arrays, vectors, maps, etc.
// An iterator cycles through values by borrowing, so the collection
// // is not moved (You can't change values)

// fn main() {
//     let mut arr_it = [1, 2, 3, 4];
//     for val in arr_it.iter() {
//         println!("{}", val);
//     }

//     // You can create an iterator
//     let mut iter1 = arr_it.iter();
//     // And call for each value with next
//     println!("1st : {:?}", iter1.next());

//     // You could consume the collection with
//     // arr_it.into_iter() but you'll no longer be able to use the collection
// }

// fn main() {
//     // ----- CLOSURES -----
//     // A closure is a function without a name and they are sometimes
//     // stored in a variable (They can be used to pass a function into
//     // another function)
//     // let var_name = |parameters| -> return_type {BODY}

//     // Create a closure that defines if someone can vote
//     let can_vote = |age: i32| age >= 18;
//     println!("Can vote : {}", can_vote(8));

//     // Closures can access variables outside of its body with borrowing
//     let mut samp1 = 5;
//     let print_var = || println!("samp1 = {}", samp1);
//     print_var();
//     samp1 = 10;

//     // You can change values if you mark the closure mutable
//     let mut change_var = || samp1 += 1;
//     change_var();
//     println!("samp1 = {}", samp1);
//     samp1 = 10;
//     println!("samp1 = {}", samp1);

//      // You can pass closures to functions
//      fn use_func<T>(a: i32, b: i32, func: T) -> i32
//      where
//          T: Fn(i32, i32) -> i32,
//      {
//          func(a, b)
//      }

//      let sum = |a, b| a + b;
//      let prod = |a, b| a * b;

//      println!("5 + 4 = {}", use_func(5, 4, sum));
//      println!("5 * 4 = {}", use_func(5, 4, prod));
//  }

// fn main() {
//     // ----- SMART POINTERS -----
//     // A pointer is an address to a location in memory. We have been
//     // using them when we used the reference operator(&) to borrow
//     // a value.

//     // Strings and vectors are smart pointers. They own
//     // data and also have functions for manipulating that data.

//     // Smart pointers provide functionality beyond referencing locations
//     // in memory. They can be used to track who has ownership of data.
//     // Ownership is very important with Rust.

//     // ----- BOX -----

//     // The Box smart pointer stores data on the heap instead of the stack.
//     // All values are stored on the stack by default

//     // Stack : Stores values in a last in first out format
//     // Data on the stack must have a defined fixed size

//     // Heap : When putting data on the heap you request a certain
//     // amount of space. The OS finds space available and returns
//     // an address for that space called a pointer.

//     // A Box is normally used when you have a large amount of data stored
//     // on the heap and then you pass pointers to it on the stack.

//     // Create a Box with value 10
//     let b_int1 = Box::new(10);

//     // Get the value
//     println!("b_int1 = {}", b_int1);

//     // If we try to create a Binary tree we get the error
//     // the size for values of type `str` cannot be known at
//     // compilation time within `TreeNode<T>`

//     // This is saying we can't include nodes in a node because
//     // the size of node depends on the size of multiple nodes
//     // which confuses the compiler
//     // struct TreeNode<T> {
//     //     pub left: TreeNode<T>,
//     //     pub right: TreeNode<T>,
//     //     pub key: T,
//     // }

//     // We have other problems in that Binary Trees eventually end
//     // and Rust doesn't like Null values so we have to use Option

//     // We can use a Box here because it has a pointer to data and
//     // a fixed size

//     struct TreeNode<T> {
//         pub left: Option<Box<TreeNode<T>>>,
//         pub right: Option<Box<TreeNode<T>>>,
//         pub key: T,
//     }

//     // Create functions for creating nodes and adding left & right
//     impl<T> TreeNode<T> {
//         pub fn new(key: T) -> Self {
//             TreeNode {
//                 left: None,
//                 right: None,
//                 key,
//             }
//         }

//         pub fn left(mut self, node: TreeNode<T>) -> Self {
//             self.left = Some(Box::new(node));
//             self
//         }

//         pub fn right(mut self, node: TreeNode<T>) -> Self {
//             self.right = Some(Box::new(node));
//             self
//         }
//     }

//     // Create the root node with left and right
//     let node1 = TreeNode::new(1)
//         .left(TreeNode::new(2))
//         .right(TreeNode::new(3));

//     // Used to test original
//     // let mut boss = TreeNode {
//     //     left: None,
//     //     right: None,
//     //     key: 50,
//     // };
// }

fn main() {
    // ----- CONCURRENCY -----
    // Concurrent programming envolves executing different blocks of code
    // independently, while parallel programming is when different
    // code executes at the same time. A thread handles scheduling
    // and execution of these blocks of code.

    // Common problems with parallel programming involve :
    // 1. Thread are accessing data in the wrong order
    // 2. Threads are blocked from executing because of confusion
    // over requirements to proceed with execution

    use std::thread;
    use std::time::Duration;

    // // Create a thread with spawn
    // thread::spawn(|| {
    //     for i in 1..25 {
    //         println!("Spawned thread : {}", i);
    //         // Forces thread to sleep and allow another thread to execute
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // // There are no guarantees on when the threads will execute and
    // // that they will complete execution
    // for i in 1..20 {
    //     println!("Main thread : {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // If we assign the return value for this thread to a variable
    // and then call join on it our program will wait for it to stop
    // executing
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread : {}", i);
            // Forces thread to sleep and allow another thread to execute
            thread::sleep(Duration::from_millis(1));
        }
    });

    // There are no guarantees on when the threads will execute and
    // that they will complete execution
    for i in 1..20 {
        println!("Main thread : {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // We call join here so that the main thread executes with thread1
    // unwrap handles the option Result which is Ok or Err
    thread1.join().unwrap();

    // ----- BANK ACCOUNT EXAMPLE -----
    // We will create a bank account that multiple customers will try
    // to withdraw money from

    // Bank struct just contains current balance
    pub struct Bank {
        balance: f32,
    }

    // Allows for withdrawing money
    // Pass a mutable reference so bank can be used elsewhere
    // fn withdraw(the_bank: &mut Bank, amt: f32) {
    //         the_bank.balance -= amt;
    //     }

    // Create bank struct
    // let mut bank = Bank{balance: 100.00};
    // withdraw(&mut bank, 5.00);
    // println!("Balance : {}", bank.balance);

    // Create a customer thread that withdraws money
    // THIS WON'T WORK
    // fn customer(the_bank: &mut Bank){
    //     withdraw(the_bank, 5.00)
    // }

    // Can't do this closure may outlive the current function,
    // but it borrows `bank`, which is owned by the current function
    // If a thread can outlive the main function and the main function
    // has the bank which causes problems
    // thread::spawn(|| {
    //     customer(&mut bank)
    // }).join().unwrap();

    // The fix that allows multiple owners and blocks access when needed
    // A smart pointer Rc<RefCell<T>> allows multiple owners with mutable
    // access to the same data
    use std::cell::RefCell;
    use std::rc::Rc;
    // Arc<T> provides shared ownership of a value
    // Mutex blocks threads waiting for lock to be available
    use std::sync::{Arc, Mutex};

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();

        if bank_ref.balance < 5.00 {
            println!(
                "Current Balance : {} Withdrawal a smaller amount",
                bank_ref.balance
            );
        } else {
            bank_ref.balance -= amt;
            println!(
                "Customer withdrew {} Current Balance {}",
                amt, bank_ref.balance
            );
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 20.00 }));

    // Creates 10 customer threads
    let handles = (0..10).map(|_| {
        // Clone duplicates an the bank object
        let bank_ref = bank.clone();
        thread::spawn(|| customer(bank_ref))
    });

    // Wait for all customers to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total: {}", bank.lock().unwrap().balance);

    // ----- INSTALLATION ------
    // Install rustup on Mac or Linux
    // curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
}
