#![allow(unused)]

use rand::Rng;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod restaurant;
use restaurant::order_food;

fn main() {
    // // Interactions basiques input
    // println!("What is your name ?");
    // let mut name: String = String::new();
    // let greeting: &str = "Nice to meet you";
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Didn't receive input");
    // println!("Hello {}! {}", name.trim_end(), greeting);

    // // Types, Shadowing, Conversion
    // const ONE_MIL: u32 = 1_000_000;
    // const PI: f32 = 3.141592;
    // let age: &str = "47";
    // let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    // age = age + 1;
    // println!("I'm {} and I want ${}", age, ONE_MIL);

    // // Data Types
    // println!("Max u32, {}", u32::MAX);
    // println!("Max u64, {}", u64::MAX);
    // println!("Max u128, {}", u128::MAX);
    // println!("Max usize, {}", usize::MAX);
    // println!("Max f32, {}", f32::MAX);
    // println!("Max f64, {}", f64::MAX);

    // // Math
    // let num_1: u32 = 5;
    // let num_2: u32 = 4;
    // println!("5 + 4 = {}", num_1 + num_2);
    // println!("5 - 4 = {}", num_1 - num_2);
    // println!("5 * 4 = {}", num_1 * num_2);
    // println!("5 / 4 = {}", num_1 / num_2);
    // println!("5 % 4 = {}", num_1 % num_2);
    // _num_1 += 1;

    // // Random
    // let random_num = rand::thread_rng().gen_range(1..101);
    // println!("Random: {}", random_num)

    // // IF
    // let age: u32 = 55;
    // if (age >= 1) && (age <= 18) {
    //   println!("Important Birthday");
    // } else if (age == 21) || (age == 50) {
    //   println!("Important Birthday");
    // } else if (age >= 65) {
    //   println!("Important Birthday");
    // } else {
    //   println!("Not an Important Birthday");
    // }

    // // Ternary operator
    // let mut my_age = 47;
    // let can_vote = if my_age >= 18 { true } else { false };
    // println!("Can vote: {}", can_vote);

    // // Match
    // let age2 = 51;
    // match age2 {
    //     1..=18 => println!("Important Birthday"),
    //     21 | 50 => println!("Important Birthday"),
    //     65..=i32::MAX => println!("Important Birthday"),
    //     _ => println!("Not an Important Birthday")
    // };

    // let my_age = 18;
    // let voting_age = 18;
    // match my_age.cmp(&voting_age) {
    //   Ordering::Less => println!("Can't vote"),
    //   Ordering::Equal => println!("You gained the right to vote"),
    //   Ordering::Greater => println!("Can vote")
    // };

    // // Array
    // let arr_1 = [1,2,3,4];
    // println!("1st {}", arr_1[0]);
    // println!("Length {}", arr_1.len());

    // // Loop, While and For
    // let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    // let mut loop_idx = 0;
    // loop {
    //     if (arr_2[loop_idx] % 2 == 0) {
    //         loop_idx += 1;
    //         continue;
    //     }
    //     if arr_2[loop_idx] == 9 {
    //         println!("Index {}", arr_2[loop_idx]);
    //         break;
    //     }
    //     println!("Index {}", arr_2[loop_idx]);
    //     loop_idx += 1;
    // }
    // while loop_idx < arr_2.len() {
    //     println!("Index {}", arr_2[loop_idx]);
    //     loop_idx += 1;
    // }
    // for val in arr_2.iter() {
    //   println!("Index {}", val);
    // }

    // // Tuples
    // let my_tuple: (u8, String, f64) = (60, "Carlito".to_string(), 50_000.00);
    // println!("Name {}, Age {}, Sold {}", my_tuple.1, my_tuple.0, my_tuple.2);
    // let (v1,v2,v3) = my_tuple;
    // println!("Name {}, Age {}, Sold {}", v2, v1, v3);

    // // String
    // let mut st1 = String::new();
    // st1.push('A');
    // st1.push_str(" word");
    // for word in st1.split_whitespace() {
    //   println!("{}", word);
    // };
    // println!("{}", st1);
    // let st2 = st1.replace("A", "Another");
    // println!("{}", st2);

    // let st3 = String::from("a l m f f j u t s");
    // let mut v1: Vec<char> = st3.chars().collect();
    // v1.sort();
    // v1.dedup();
    // for char in v1 {
    //     println!("{}", char);
    // }
    // let st4 = "Random string";
    // let mut st5 = st4.to_string();
    // println!("{}", st5);
    // let byte_arr1 = st5.as_bytes();
    // let st6 = &st5[0..6];
    // println!("String length : {}", st6.len());
    // st5.clear();
    // let st6 = String::from("Just some");
    // let st7 = String::from(" words");
    // let st8 = st6 + &st7; // st6 n'existe plus il est déplacé dans st8
    // for char in st8.bytes() {
    //     println!("{}", char);
    // }

    // // Casting
    // let int_u8: u8 = 5;
    // let int2_u8: u8 = 4;
    // let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
    // println!("{}", int3_u32);

    // // Enums
    // enum Day {
    //   Lundi,
    //   Mardi,
    //   Mercredi,
    //   Jeudi,
    //   Vendredi,
    //   Samedi,
    //   Dimanche
    // }

    // impl Day {
    //   fn is_weekend(&self) -> bool {
    //     match self {
    //       Day::Samedi | Day::Dimanche => true,
    //       _ => false
    //     }
    //   }
    // }

    // let today: Day = Day::Lundi;
    // match today {
    //   Day::Lundi => println!("Tout le monde déteste le lundi"),
    //   Day::Mardi => println!("mardi"),
    //   Day::Mercredi => println!("mercredi"),
    //   Day::Jeudi => println!("jeudi"),
    //   Day::Vendredi => println!("vendredi"),
    //   Day::Samedi => println!("week end"),
    //   Day::Dimanche => println!("week end")
    // }
    // println!("Is today the weekend {}", today.is_weekend());

    // // Vectors
    // let vec1: Vec<i32> = Vec::new();
    // let mut vec2 = vec![1, 2, 3, 4];
    // vec2.push(5);
    // println!("1st : {}", vec2[0]);
    // let second: &i32 = &vec2[1];
    // match vec2.get(1) {
    //     Some(second) => println!("2nd : {}", second),
    //     None => println!("No 2nd value"),
    // }
    // for i in &mut vec2 {
    //     *i *= 2;
    // }
    // for i in &vec2 {
    //     println!("{}", i);
    // }
    // println!("Pop : {:?}", vec2.pop());
    // println!("Vec Length {}", vec2.len());

    // // Functions
    // fn get_sum(x: i32, y: i32) -> i32 {
    //     x + y
    //     // return x + y;
    // }
    // println!("{}", get_sum(1, 3));
    // fn get_2(x: i32) -> (i32, i32) {
    //   return (x+1, x+2);
    // }
    // let (val1, val2) = get_2(3);
    // println!("{} et {}", val1, val2);

    // let num_list = vec![1,2,3,4,5];
    // fn sum_list(vec: &[i32]) -> i32 { // vec: Vec<i32>
    //   let mut count: i32 = 0;
    //   // for i in vec {
    //   //   count += i;
    //   // }
    //   for &val in vec.iter() {
    //     count += &val;
    //   }
    //   count
    // }
    // println!("Sum of list : {}", sum_list(&num_list));

    // // Generic
    // fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    //     return x + y;
    // }
    // println!("5 + 4 = {}", get_sum_gen(5, 4));
    // println!("5.2 + 4.7 = {}", get_sum_gen(5.2, 4.7));

    // // Ownership
    // let str1 = String::from("World");
    // let str3 = str1.clone();
    // let mut str2 = str1;
    // // println!("Hello {}", str1); // ERROR, str1 n'existe plus
    // println!("Hello {}", str3);

    // fn print_str(x: String) {
    //     println!("A string {}", x);
    // }
    // fn print_return_str(x: String) -> String {
    //     println!("A string {}", x);
    //     x
    // }
    // fn change_string(name: &mut String) {
    //     name.push_str(" is happy");
    //     println!("Message : {}", name)
    // }
    // change_string(&mut str2);

    // // HashMaps
    // let mut heroes = HashMap::new();
    // heroes.insert("Superman", "Clark Kent");
    // heroes.insert("Batman", "Bruce Wayne");
    // heroes.insert("Flash", "Barry Allen");
    // for (k, v) in heroes.iter() {
    //     println!("{} = {}", k, v)
    // }
    // if heroes.contains_key(&"Batman"){
    //     let the_batman = heroes.get(&"Batman");
    //     match the_batman {
    //         Some(x) => println!("Batman is a hero"),
    //         None => println!("Batman is not a hero")
    //     }
    // };

    // // Struct
    // struct Customer {
    //     name: String,
    //     address: String,
    //     balance: f32,
    // }
    // let mut client: Customer = Customer {
    //     name: String::from("Carlito"),
    //     address: String::from("51 toto street"),
    //     balance: 3024.56,
    // };
    // client.address = String::from("62 rue titi");
    // println!("{}", client.address)
    // struct Rectangle<T, U> {
    //     length: T,
    //     height: U,
    // }
    // let rec = Rectangle {
    //     length: 4,
    //     height: 10.5,
    // };

    // // Trait
    // trait Shape {
    //     fn new(length: f32, width: f32) -> Self;
    //     fn area(&self) -> f32;
    // }
    // struct Rectangle {
    //     length: f32,
    //     width: f32,
    // }
    // struct Circle {
    //     length: f32,
    //     width: f32,
    // }
    // impl Shape for Rectangle {
    //     fn new(length: f32, width: f32) -> Rectangle {
    //         return Rectangle{length, width};
    //     }
    //     fn area(&self) -> f32 {
    //         return self.length * self.width;
    //     }
    // }
    // const PI:f32 = 3.141592;
    // impl Shape for Circle {
    //     fn new(length: f32, width: f32) -> Circle {
    //         return Circle { length, width };
    //     }
    //     fn area(&self) -> f32 {
    //         return (self.length / 2.0).powf(2.0) * PI;
    //     }
    // }
    // let rec: Rectangle = Shape::new(10.0,10.0);
    // let cir: Circle = Shape::new(10.0, 10.0);
    // println!("Rectangle Area: {}", rec.area());
    // println!("Circle Area: {}", cir.area());

    // // // Modules
    // // Crates : Modules that produce a library or executable
    // // Modules : Organize and handle privacy
    // // Packages : Build, test and share crates
    // // Paths : A way of naming an item such as a struct, function
    // order_food();

    // // Error Handling
    // let path = "lines.txt";
    // let output = File::create(path);
    // // Result has 2 variables Ok and Err
    // let mut output = match output {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem creating file : {:?}", error);
    //     }
    // };
    // write!(output, "Just some\nRandom words").expect("Failed to write to file");
    // let input = File::open(path).unwrap();
    // let buffered = BufReader::new(input);
    // for line in buffered.lines() {
    //     println!("{}", line.unwrap());
    // }

    // let output2 = File::create("rand.txt");
    // let output2 = match output2 {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("rand.txt"){
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Can't create file : {:?}", error)
    //         },
    //         _other_error => panic!("Problem opening file : {:?}", error)
    //     }
    // };

    // // Closures
    // let can_vote = |age: i32| age >= 18;
    // println!("Can vote : {}", can_vote(18));

    // let mut samp1 = 5;
    // let print_var = || println!("samp1 = {}", samp1);
    // print_var();
    // samp1 = 10;
    // let mut change_var = || samp1 += 1;
    // change_var();
    // println!("samp1 = {}", samp1);
    // samp1 = 10;
    // println!("samp1 = {}", samp1);

    // fn use_func<T>(a: i32, b: i32, func: T) -> i32
    // where
    //     T: Fn(i32, i32) -> i32,
    // {
    //     func(a, b)
    // }
    // let sum = |a, b| a + b;
    // let prod = |a, b| a * b;
    // println!("5 + 4 = {}", use_func(5, 4, sum));
    // println!("5 * 4 = {}", use_func(5, 4, prod));

    // // Box
    // let b_int1 = Box::new(10);
    // println!("b_int1 = {}", b_int1);

    // struct TreeNode<T> {
    //     pub left: Option<Box<TreeNode<T>>>,
    //     pub right: Option<Box<TreeNode<T>>>,
    //     pub key: T
    // }
    // impl<T> TreeNode<T> {
    //     pub fn new(key: T) -> Self {
    //         TreeNode { left: None, right: None, key }
    //     }
    //     pub fn left(mut self, node: TreeNode<T>) -> Self {
    //         self.left = Some(Box::new(node));
    //         self
    //     }
    //     pub fn right(mut self, node: TreeNode<T>) -> Self {
    //         self.right = Some(Box::new(node));
    //         self
    //     }
    // }
    // let node1 = TreeNode::new(1)
    // .left(TreeNode::new(2))
    // .right(TreeNode::new(3));

    // // Thread
    // let thread1 = thread::spawn(|| {
    //     for i in 1..25 {
    //         println!("Spawned thread : {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..20 {
    //     println!("Main thread : {}", i);;
    //     thread::sleep(Duration::from_millis(1));
    // }
    // thread1.join().unwrap(); // Pour etre sur que les deux threads s'executent jusqu'à la fin

    // pub struct Bank {
    //     balance: f32
    // }
    // fn withdraw(the_bank: &mut Bank, amount: f32) {
    //     the_bank.balance -= amount;
    // }
    // let mut bank = Bank{balance: 100.0};
    // withdraw(&mut bank, 5.0);
    // println!("Balance : {}", bank.balance);
    // fn customer(the_bank: &mut Bank) {
    //     withdraw(the_bank, 5.0);
    // }
    // // thread::spawn(|| {
    // //     customer(&mut bank)
    // // }).join().unwrap();

    // Rc T
    pub struct Bank {
        balance: f32,
    }
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.0 {
            println!(
                "Current Balance : {} Withdrawal a smaller amount",
                bank_ref.balance
            )
        } else {
            bank_ref.balance -= amt;
            println!(
                "Customer withdrew : {} Current Balance : {}",
                amt, bank_ref.balance
            )
        }
    }
    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.0)
    }
    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 20.0 }));
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| customer(bank_ref))
    });
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total {}", bank.lock().unwrap().balance);
}
