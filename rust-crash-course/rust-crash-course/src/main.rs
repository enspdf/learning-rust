#![deny(clippy::all)]

use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // println!("{:?}", scores);

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];

    // let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    // println!("{:?}", scores);
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
}

// use std::fmt::format;

// use futures::executor::block_on;
// use futures::Future;
// use tokio::time::{sleep, Duration};

// async fn get_name() -> String {
//     "John".to_string()
// }

// fn call_api_one() -> impl Future<Output = String> {
//     async {
//         sleep(Duration::from_secs(1)).await;
//         "One".to_string()
//     }
// }

// fn call_api_two() -> impl Future<Output = String> {
//     async {
//         sleep(Duration::from_secs(1)).await;
//         "Two".to_string()
//     }
// }

// fn get_async_name() -> impl Future<Output = String> {
//     let name = "John".to_string();
//     async move { format!("Hello {} Doe", name) }
// }

// #[tokio::main]
// async fn main() {
//     // let name = block_on(get_name());
//     // println!("Hello, {}!", name);
//     let one = call_api_one().await;
//     println!("{}", one);
//     let two = call_api_two().await;
//     println!("{}", two);
// }

// use std::ops::AddAssign;

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn move_offset(&mut self, x: T, y: T)
//     where
//         T: AddAssign,
//     {
//         self.x += x;
//         self.y += y;
//     }
// }

// impl<T: AddAssign> AddAssign for Point<T> {
//     fn add_assign(&mut self, other: Self) {
//         self.x += other.x;
//         self.y += other.y;
//     }
// }

// impl<T: PartialEq> PartialEq for Point<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }

// use std::str;

// trait CanRun {
//     fn run(&self);
// }

// trait CanWalk {
//     fn walk(&self);
// }

// impl<T: CanRun> CanRun for Vec<T> {
//     fn run(&self) {
//         for item in self {
//             item.run();
//         }
//     }
// }

// impl<T: CanWalk> CanWalk for Vec<T> {
//     fn walk(&self) {
//         for item in self {
//             item.walk();
//         }
//     }
// }

// struct Person {
//     name: String,
// }

// impl CanWalk for Person {
//     fn walk(&self) {
//         println!("{} is walking", self.name);
//     }
// }

// impl CanRun for Person {
//     fn run(&self) {
//         println!("{} is running", self.name);
//     }
// }

// struct Elephant {
//     name: String,
// }

// impl CanWalk for Elephant {
//     fn walk(&self) {
//         println!("{} is walking", self.name);
//     }
// }

// impl CanRun for Elephant {
//     fn run(&self) {
//         println!("{} is running", self.name);
//     }
// }

// fn main() {
// let p1 = Point { x: 0, y: 8 };
// let p2 = Point { x: 0.0, y: 0.8 };
// let p3 = Point { x: "Goo", y: "abc" };
// let mut p = Point { x: 1.0, y: 2.0 };
// p.move_offset(3.0, 4.0);
// println!("{:?}", p);
// let p1 = Point { x: 1.0, y: 2.0 };
// let p2 = Point { x: 3.0, y: 4.0 };
// // p1 += p2;
// println!("{:?}", p1);
// if p1 == p2 {
//     println!("p1 and p2 are equal");
// } else {
//     println!("p1 and not p2 are equal");
// }
//     let people = vec![
//         Person {
//             name: "John".to_string(),
//         },
//         Person {
//             name: "Jane".to_string(),
//         },
//         Person {
//             name: "Joe".to_string(),
//         },
//     ];
//     people.walk();
//     people.run();

//     let elephants = vec![
//         Elephant {
//             name: "Elephant1".to_string(),
//         },
//         Elephant {
//             name: "Elephant2".to_string(),
//         },
//         Elephant {
//             name: "Elephant3".to_string(),
//         },
//     ];
//     elephants.walk();
//     elephants.run();
// }

// use std::ops::Deref;
// use std::rc::Rc;
// use std::cell::Cell;

// struct BoxedValue<T> {
//     value: T,
// }

// impl<T> BoxedValue<T> {
//     fn new(value: T) -> Self {
//         BoxedValue { value }
//     }
// }

// impl<T> Deref for BoxedValue<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.value
//     }
// }

// fn print_integer(value: &i32) {
//     println!("{}", value);
// }

// struct Person {
//     name: String,
//     age: Cell<u8>,
// }

// impl Person {
//     fn increment_age(&self) -> u8 {
//         self.age.set(self.age.get() + 1);
//         self.age.get()
//     }
// }

// use std::cell::RefCell;

// fn main() {
// let age = Box::new(22);
// let twice = *age * 2;
// println!("{}", twice);
// let age = BoxedValue::new(22);
// println!("Value is {}", *age);
// let actual_age = age.deref();
// println!("Value is {}", actual_age);
// let ref_to_value = age.deref();
// let other = *(age.deref());
// let value = BoxedValue::new(10);
// print_integer(&value);
// let array = vec!["John".to_string(), "Jane".to_string()];
// let rc = Rc::new(array);
// let weak = Rc::downgrade(&rc);
// drop(rc);
// // let value = weak.upgrade().unwrap();
// // println!("{:?}", value);
// match weak.upgrade() {
//     Some(rc) => println!("{:?}", rc),
//     None => println!("None"),
// }
// let rc2 = rc.clone();
// let rc2 = Rc::clone(&rc);
// drop(rc);
// println!("{:?}", rc2);
// let person = Person {
//     name: "John".to_string(),
//     age: Cell::new(20),
// };
// let new_age = person.increment_age();
// println!("{}", new_age);
// let ref_cell = RefCell::new(vec![1, 2, 3]);
// let mut mut_ref = ref_cell.borrow_mut();
// let len = ref_cell.borrow().len();
// mut_ref.push(100);
// println!("Length = {}", len);
// }

// use std::fmt;

// #[derive(Debug)]
// struct Person {
//     first_name: String,
//     last_name: String,
//     age: u8,
// }

// impl fmt::Display for Person {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{} {} is {} years old",
//             self.first_name, self.last_name, self.age
//         )
//     }
// }

// // trait HasFullName {
// //     fn full_name(&self) -> String;
// // }

// trait HasName {
//     fn first_name(&self) -> &str;
//     fn last_name(&self) -> &str;
// }

// trait HasFullName
// where
//     Self: HasName,
// {
//     // fn full_name(&self) -> String {
//     //     format!("{} {}", self.first_name(), self.last_name())
//     // }

//     fn full_name(&self) -> String;
// }

// impl<T> HasFullName for T
// where
//     T: HasName,
// {
//     fn full_name(&self) -> String {
//         format!("{} {}", self.first_name(), self.last_name());
//     }
// }

// fn print_full_name_and_age(value: &impl HasFullName) {
//     println!("{}", value.full_name())
// }

// fn print_details<T>(value: &T)
// where
//     T: HasFullName + CanRun,
// {
//     println!("{}", value.full_name());
//     value.run();
// }

// trait CanRun {
//     fn run(&self);
// }

// impl CanRun for Person {
//     fn run(&self) {}
// }

// impl HasFullName for Person {
//     fn full_name(&self) -> String {
//         format!("{} {}", self.first_name, self.last_name)
//     }
// }

// trait CanInitializeWithFullName {
//     fn new(full_name: &str) -> Self;
// }

// impl CanInitializeWithFullName for Person {
//     fn new(full_name: &str) -> Self {
//         let parts: Vec<&str> = full_name.split(' ').collect();
//         Person {
//             first_name: parts[0].to_string(),
//             last_name: parts[1].to_string(),
//             age: 30,
//         }
//     }
// }

// fn main() {
// let person = Person {
//     first_name: "John".to_string(),
//     last_name: "Doe".to_string(),
//     age: 30,
// };
// println!("{:?}", person);
// let person = Person::new("John Doe");
// println!(
//     "First name = {}, last name = {}, age = {}",
//     person.first_name, person.last_name, person.age
// );
// println!("{}", person);
// print_full_name_and_age(&person);
// }

// fn get_full_name() -> &'static str {
//     "John Doe"
// }

// fn get_random_name<'l>(a: &'l str, b: &'l str) -> &'l str {
//     a
// }

// struct Person<'a> {
//     name: &'a str,
// }

// fn get_first_name(full_name: &str) -> &str {
//     full_name
// }

// struct Person<'a> {
//     first_name: &'a str,
//     last_name: &'a str,
// }

// impl<'a> Person<'a> {
//     fn first_char_of_first_name(&self) -> &str {
//         &self.first_name[0..1]
//     }

//     fn get_full_name(&self) -> String {
//         format!("{} {}", self.first_name, self.last_name)
//     }
// }

// enum Animal<'a> {
//     Dog { name: &'a str },
// }

// fn main() {
// let full_name = get_full_name();
// println!("Hello, {}", full_name);
// let name = get_random_name("John", "Doe");
// println!("{}", name);
// }

// fn get_user_name() -> Result<String, ()> {
//     Ok("John".to_string())
//     // Err(())
// }

// fn get_first_name() -> Result<String, ()> {
//     Ok("John".to_string())
//     // Err(())
// }

// fn get_last_name() -> Result<String, ()> {
//     Ok("Doe".to_string())
// }

// fn get_full_name() -> Result<String, ()> {
//     let first_name = get_first_name()?;
//     let last_name = get_last_name()?;

//     Ok(format!("{} {}", first_name, last_name))
// }

// fn main() {
// let value: Result<&str, Box<dyn std::error::Error>> = Ok("Hello, world");
// match value {
//     Ok(value) => println!("{}", value),
//     Err(error) => println!("{}", error),
// }
// let value: Result<&str, ()> = Err(());
// match value {
//     Ok(value) => println!("{}", value),
//     Err(_) => println!("Some error ocurred"),
// }
// let unwrapped = value.expect("I was expecting a user name");
// let user_name = get_user_name().expect_err("I didn't expect a user name");
// println!("Hello, {}", user_name);
// get_user_name().expect_err("I didn't expect a user name");
// let is_ok = get_user_name().is_ok();
// let is_err = get_user_name().is_err();
// println!("{} {}", is_ok, is_err);

// let full_name = get_full_name();
// match full_name {
//     Ok(name) => print!("Hello, {}", name),
//     Err(_) => println!("Error"),
// }
//     let length = full_name.map(|s| s.len()).unwrap_or_default();
//     println!("{}", length);
// }

// fn main() {
// let age: Option<i32> = None;
// let age = age.unwrap_or_default();
// let age_multiplied_by_two = age.map(|age| age * 2);
// println!("{}", age_multiplied_by_two.unwrap_or_default());
// let name: Option<&str> = None;
// let unwrapped = name.unwrap_or("John Doe");
// let unwrapped = name.unwrap_or_else(|| "John Doe");
// println!("name is {}", unwrapped);
// if name.is_some() {
//     println!("Value");
// } else {
//     println!("No Value");
// }

// if name.is_none() {
//     println!("No Value");
// } else {
//     println!("Value");
// }
// let value = Some(10);
// let name: Option<&str> = None;
// let name: Option<&str> = Some("John Doe");
// match name {
//     Some(name) => println!("Hello, {}", name),
//     None => println!("There is no name"),
// }
// let unwrapped_name = name.expect("Name was not provided");
// let unwrapped_name = name.unwrap();
// println!("Nama is: {}", unwrapped_name);
// let mut age: Option<i8> = Some(20);
// match age.as_mut() {
//     Some(age) => *age += 10,
//     None => {}
// }
// println!("Age is: {}", age.unwrap());
// let age1 = Some(20);
// let age2 = Some(30);
// let age3 = Some(40);
// if let (Some(age_1), Some(age_2), Some(age_3)) = (age1, age2, age3) {
//     println!("{}", age_1 + age_2 + age_3);
// }
// }

// use std::collections::HashMap;
// use std::vec;

// #[derive(Hash, Eq, PartialEq, Debug)]
// struct Person {
//     name: String,
//     age: u8,
// }

// fn main() {
// let values = vec![1, 2, 3, 4, 5];
// let multiplied_by_2: Vec<i32> = values.iter().map(|v| v * 2).collect();
// println!("{:?}", multiplied_by_2);
// let iter = values.iter();
// let sum1: i32 = iter.sum();
// let iter2 = values.iter();
// let sum2: i32 = iter2.sum();
// for value in values.iter() {
//     println!("{}", value);
// }
// let mut values: HashMap<Person, &str> = HashMap::new();
// let mut values: HashMap<&str, &str> = HashMap::new();
// values.insert("husband", "John Doe");
// println!("{:?}", values);
// if values.contains_key("name") {
//     println!("Name exists");
// } else {
//     println!("Not exists");
// }
// values.remove("foo");
// println!("{:?}", values);
// let bar = values["foo"];
// println!("{}", bar);
// let bar = values.get("foo");
// match values.get("foo") {
//     Some(value) => println!("{}", value),
//     None => println!("Not Found"),
// }
// for (&k, &v) in &values {
//     println!("{} {}", k, v);
// }
// let entry = values.entry("foo");
// match entry {
//     std::collections::hash_map::Entry::Occupied(value) => {
//         println!("{}", value.get());
//     }
//     _ => println!("Not found"),
// }
// values.entry("wife").or_insert("Jane Doe");
// println!("{:?}", values);
// let names = vec!["John", "Jane", "Mary", "Bob", "Tom"];
// for name in names.into_iter().filter(|name| name.len() == 3) {
//     println!("{}", name);
// }
// for name in names.into_iter() {
//     if name.len() != 3 {
//         break;
//     }
//     println!("{}", name);
// }
// }

// fn main() {
// let values = [10, 20];
// for value in values.iter() {
//     println!("{}", value);
// }

// let fooValue = &values[0];
// println!("Foo is {}", fooValue);
// let length = values.len();
// println!("length: {}", length);
// let doubled = values.iter().map(|x| x * 2);
// let mut values1 = vec![1, 2, 3];
// let mut values2 = vec![4, 5, 6];
// println!("Values1 {:?}", values1);
// println!("Values2 {:?}", values2);
// values1.append(&mut values2);
// println!("Values1 {:?}", values1);
// println!("Values2 {:?}", values2);
// values.push(4);
// let four = values.pop();
// println!("Values are {:?}", values);
// values.clear();
// values.extend_from_slice(&[9, 8]);
// println!("Values are {:?}", values);
// if values1.contains(&3) {
//     println!("yes")
// } else {
//     println!("no")
// }
// if values1.is_empty() {
//     println!("is empty")
// }
// }

// enum Pet {
//     Cat { name: String },
//     Dog { name: String },
// }

// fn main() {
//     let fluffy = Pet::Cat {
//         name: "Fluffy".to_string(),
//     };
//     let name = match fluffy {
//         Pet::Cat { name } => name,
//         Pet::Dog { name } => name,
//     };
//     println!("Hello, {}", name)
// }

// struct Size {
//     width: f32,
//     height: f32,
// }

// enum Shapes {
//     Rectangle(f32, f32, Size),
//     Circle(f32, f32, f32),
// }

// impl Shapes {
//     fn area(&self) -> f32 {
//         match self {
//             Shapes::Rectangle(x, y, size) => size.width * size.height,
//             Shapes::Circle(x, y, radius) => 3.14 * radius * radius,
//         }
//     }
// }

// fn main() {
//     let rectangle = Shapes::Rectangle(
//         1.0,
//         2.0,
//         Size {
//             width: 3.0,
//             height: 4.0,
//         },
//     );
// if let Shapes::Rectangle(x, y, Size { width, height }) = rectangle {
//     println!("{} {} {} {}", x, y, width, height)
// }
// match rectangle {
//     Shapes::Rectangle(x, y, Size { width, height }) => {
//         println!(
//             "Rectangle: x: {}, y: {}, width: {}, height: {}",
//             x, y, width, height
//         )
//     }
//     _ => println!("Not a rectangle"),
// }
//     let area = rectangle.area();
//     println!("Area is: {}", area)
// }

// enum Shapes {
//     Circle { radius: f64, center: (f64, f64) },
//     Rectangle { width: f32, height: f32 },
// }

// fn main() {
//     let rectangle = Shapes::Rectangle {
//         width: 3.0,
//         height: 4.0,
//     };

//     if let Shapes::Rectangle { width, height } = rectangle {
//         println!("width = {}, height = {}", width, height)
//     }

//     match rectangle {
//         Shapes::Rectangle { width, height } => {
//             println!("width = {}, height = {}", width, height)
//         }
//         _ => println!("Not a rectangle"),
//     }
// }

// #[derive(PartialEq)]
// enum AnimalType {
//     Dog,
//     Cat,
//     Rabbit,
// }

// fn main() {
//     let fluffy = AnimalType::Dog;
// if fluffy == AnimalType::Dog {
//     println!("Fluffy is a dog");
// }
//     match fluffy {
//         AnimalType::Dog => println!("Woof!"),
//         AnimalType::Cat => println!("Meow!"),
//         AnimalType::Rabbit => println!("Hoot!"),
//         _ => println!("Something Else"),
//     }
// }

// #[derive(Debug)]
// struct Point(f64, f64, f64);

// impl Point {
//     fn describe(&self) {
//         println!("Point is at ({}, {}, {})", self.0, self.1, self.2);
//     }

//     fn zero() -> Point {
//         Point(0.0, 0.0, 0.0)
//     }
// }

// impl Point {
//     fn make_twice(&mut self) {
//         self.0 *= 2.0;
//         self.1 *= 2.0;
//         self.2 *= 2.0;
//     }

//     fn twice(&self) -> Point {
//         Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
//     }
// }

// fn main() {
// let point = Point(0.0, 1.0, 2.0);
// println!("x = {}, y = {}, z = {}", point.0, point.1, point.2);
// point.describe();
// println!("{:?}", point);
// let mut point = Point(0.0, 1.0, 2.0);
// let twice = point.twice();
// point.make_twice();
// let point = Point::zero();
// let point2 = Point::zero();
// let point3 = Point::zero();
// }

// const MY_AGE: u8 = 22;

// fn greet(name: &String) {
//     println!("Hello, {}!", name);
// }

// fn empty_string(value: &mut String) {
//     value.clear();
// }

// fn say_hello_world(message: &str) {
// let message = String::from("Hello world");
//     println!("{}", message);
// }

// fn process_name(name: &str, callback: fn(&str) -> ()) {
//     callback(name);
// }

// struct Person {
//     name: String,
//     age: u8,
//     mothers_name: String,
// }

// fn create_person(name: String, age: u8, mothers_name: String) -> Person {
//     Person {
//         name,
//         age,
//         mothers_name,
//     }
// }

// fn main() {
// let person = Person {
//     name: "John".to_string(),
//     age: 20,
// };
// println!("{} is {} years old", person.name, person.age);
//     let person = create_person("John".to_string(), 20, "Jane".to_string());
//     println!("{} is {} years old", person.name, person.age);
//     let person2 = Person {
//         name: "Doe".to_string(),
//         ..person
//     };
//     println!("{} is {} years old", person2.name, person2.age);
// }

// fn main() {
// let mut name = "John";
// println!("Your name is {}", name);
// name = "Jane";
// println!("Your name is {}", name);
// let popultion = 26_000_000;
// println!("Population {}", popultion);
// let red = 0xFA;
// let rgb = 0xFF0000;
// println!("Red {}, rgb {}", red, rgb);
// let distance_in_km: f64 = 5.5;
// println!("Distance in KM {}", distance_in_km);
// let distance1 = 5.5;
// let distance2 = 6.2;
// let distance3 = 10.2;
// let total_distance = distance1 + distance2 + distance3;
// println!("Total Distance {}", total_distance);
// let data = "Foo";
// {
//     let _data = data.to_string();
// }
// println!("Data: {}", data);
// println!("My age is {}", MY_AGE);
// let personal_data = (22, "John");
// let (age, name) = personal_data;
// // let age = personal_data.0;
// // let name = personal_data.1;
// println!("Age: {}, Name: {}", age, name);
// let name1 = String::from("John");
// let name2 = name1;
// println!("Hello, {}", name1);
// println!("Hello, {}", name2)
// {
//     let name = "John".to_string();
//     println!("Hello, {}", name);
// }
// let age1 = 10;
// let age2 = age1;
// println!("You are {} years old", age1);
// println!("You are {} years old", age2);
// let name1 = String::from("John");
// let name2 = &name1;
// println!("Hello, {}", name1);
// println!("Hello, {}", name2);
// let s1 = String::from("John");
// let s2 = &s1;
// greet(&s1);
// greet(s2);
// let mut name = String::from("John");
// empty_string(&mut name);
// let mut name = String::from("John");
// let mut name2 = &mut name;
// let mut name3 = &mut name;
// empty_string(&mut name2);
// say_hello_world("Hello");
// let say_hello_to = |name: &str| format!("Hello, {}!", name);
// println!("{}", say_hello_to("Name"));
// let full_name = |first_name: &str, last_name: &str| format!("{} {}", first_name, last_name);
// println!("{}", full_name("First", "Second"));
// let multiply_by_2 = |x: i32| x * 2;
// println!("{}", multiply_by_2(10));
// let ask_for_age = || {};
// let multiply_by_2 = |x: i32| x * 2;
// let ptr = multiply_by_2;
// let result = ptr(10);
// println!("{}", result);
// }
