#![allow(unused)]

use rand::Rng;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    pub struct Bank {
        balance: f32,
    }

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();

        if bank_ref.balance < 5.00 {
            println!(
                "Current balance : {} withdrawal a smalled amount",
                bank_ref.balance
            );
        } else {
            bank_ref.balance -= amt;
            println!(
                "Customer withdrew {} current balance {}",
                amt, bank_ref.balance
            );
        }
    }

    fn customer(the_bank: &Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 20.00 }));
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(move || {
            customer(&bank_ref);
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total {}", bank.lock().unwrap().balance);
}

// fn main() {
//     pub struct Bank {
//         balance: f32,
//     }

//     fn withdraw(the_bank: &mut Bank, amt: f32) {
//         the_bank.balance -= amt;
//     }

//     let mut bank = Bank { balance: 100.0 };
//     withdraw(&mut bank, 5.00);

//     println!("Balance : {}", bank.balance);

//     fn customer(the_bank: &mut Bank) {
//         withdraw(the_bank, 5.00);
//     }

//     thread::spawn(|| {
//         customer(&mut bank);
//     })
//     .join()
//     .unwrap();
// }

// fn main() {
// let thread1 = thread::spawn(|| {
//     for i in 1..25 {
//         println!("Spawn thread : {}", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// });

// for i in 1..20 {
//     println!("Main thread : {}", i);
//     thread::sleep(Duration::from_millis(1));
// }

// thread1.join().unwrap();
// }

// fn main() {
//     #[derive(Debug)]
//     struct TreeNode<T> {
//         pub left: Option<Box<TreeNode<T>>>,
//         pub right: Option<Box<TreeNode<T>>>,
//         pub key: T,
//     }

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

//     let node1 = TreeNode::new(1)
//         .left(TreeNode::new(2))
//         .right(TreeNode::new(3));

//     println!("{:?}", node1);
// }

// fn main() {
//     // let can_vote = |age: i32| age >= 18;
//     // println!("Can vote: {}", can_vote(8));
//     // let mut samp1 = 5;
//     // let print_var = || println!("samp1 = {}", samp1);
//     // print_var();
//     // samp1 = 10;
//     // let mut change_var = || samp1 += 1;
//     // change_var();
//     // println!("samp1 = {}", samp1);
//     // samp1 = 10;
//     // println!("samp1 = {}", samp1);

//     fn use_func<T>(a: i32, b: i32, func: T) -> i32
//     where
//         T: Fn(i32, i32) -> i32,
//     {
//         func(a, b)
//     }

//     let sum = |a, b| a + b;
//     let prod = |a, b| a * b;
//     println!("5 + 4 = {}", use_func(5, 4, sum));
//     println!("5 * 4 = {}", use_func(5, 4, prod));
// }

// fn main() {
//     let mut arr_it = [1, 2, 3, 4];
//     for val in arr_it.iter() {
//         println!("{}", val);
//     }
//     let mut iter1 = arr_it.iter();
//     println!("1st: {:?}", iter1.next());
// }

// fn main() {
//     let path = "lines.txt";
//     let output = File::create(path);
//     let mut output = match output {
//         Ok(file) => file,
//         Err(error) => panic!("Problem creating file : {:?}", error),
//     };

//     write!(output, "Just some\nRandom words").expect("Failed to write to file");

//     let input = File::open(path).unwrap();
//     let buffered = BufReader::new(input);

//     for line in buffered.lines() {
//         println!("{}", line.unwrap());
//     }

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

// mod restaurant;
// use restaurant::order_food;

// fn main() {
//     order_food();
// }

// fn main() {
//     const PI: f32 = 3.141592;
//     trait Shape {
//         fn new(length: f32, width: f32) -> Self;
//         fn area(&self) -> f32;
//     }

//     struct Rectangle {
//         length: f32,
//         width: f32,
//     };

//     struct Circle {
//         length: f32,
//         width: f32,
//     };

//     impl Shape for Rectangle {
//         fn new(length: f32, width: f32) -> Rectangle {
//             return Rectangle { length, width };
//         }

//         fn area(&self) -> f32 {
//             return self.length * self.width;
//         }
//     }

//     impl Shape for Circle {
//         fn new(length: f32, width: f32) -> Self {
//             return Circle { length, width };
//         }

//         fn area(&self) -> f32 {
//             return (self.length / 2.0).powf(2.0) * PI;
//         }
//     }

//     let rec: Rectangle = Shape::new(10.0, 10.0);
//     let circ: Circle = Shape::new(10.0, 10.0);

//     println!("Rec Are: {}", rec.area());
//     println!("Circ Area: {}", circ.area());
// }

// fn main() {
//     println!("What is your name?");
//     let mut name = String::new();
//     let greeting = "Nice to meet you";
//     io::stdin()
//         .read_line(&mut name)
//         .expect("Didn't receive input");
//     println!("Hello {}! {}", name.trim_end(), greeting);
// }

// fn main() {
//     const ONE_MIL: u32 = 1_000_000;
//     const PI: f32 = 3.141592;
//     let age = "47";
//     let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
//     age = age + 1;

//     println!("I'm {} and want ${}", age, ONE_MIL);
// }
