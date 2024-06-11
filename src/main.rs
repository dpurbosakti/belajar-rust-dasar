use core::slice;
use std::{cmp::Ordering, fmt::{format, Formatter}, result};
mod first;
mod second;
mod third;
mod model;

use first::say_hello;
use second::say_hello as say_hello_second;
use model::User;

#[test]
fn test_use() {
    say_hello();
    say_hello_second();
    first::second::third::say_hello();
}

#[test]
fn test_module() {
    let user: User = User { 
        first_name: String::from("moko"), 
        last_name: String::from("sakti"), 
        username: String::from("mokosakti"), 
        email: String::from("moko@gmail.com"), 
        age: 30 
    };

    user.say_hello("budi");
}

fn main() {
    println!("Hello, world!");


    println!("Hello, moko!");
}

#[test]
fn hello_test(){
    println!("hello test");
}

#[test]
fn test_variable(){
    let mut nama= "empty";
    let name = "Dwiatmoko";
    println!("{}", nama);

    nama = "moko";
    println!("{}", name);
    println!("{}", nama);
}

#[test]
fn tuple(){
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    let (a,b,c) = data;

    println!("{} {} {}", a,b,c);
}

#[test]
fn array(){
    let array: [i32; 3] = [1, 2, 3];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} \n{}",a,b);

    let length = array.len();
    println!("{}", length);
}

#[test]
fn two_dimensional_array(){
    let matrix: [[i32;2];2] = [
        [1,2],
        [2,3],
        ];

    println!("{:?}", matrix);
}

#[test]
fn test(){
    let mut angka: i32;
    angka = 1;
    println!("{}",angka);
    angka = 2;

    // let angka: i32;
    println!("{}", angka);
}


#[test]
fn constant() {
    const MAXIMUM: i32 = 100; 
    const MINIMUM: i32= 0;
    println!("{} {}",MAXIMUM, MINIMUM);
}


fn function_a(){
    let a = 10;
    let b = String::from("moko");
    
    println!("{} {}", a,b)
}

fn function_b(){
    let a = 10;
    let b = String::from("sakti");
    
    println!("{} {}", a,b)
}

#[test]
fn stack_heap(){
    function_a();
    function_b();
}

#[test]
fn string() {
    let name = " dwi atmoko purbo sakti ";
    let trim = name.trim();

    println!("{}", name);
    println!("{}", trim);
}

#[test]
fn string_type() {
    let mut name: String= String::from("Dwi atmoko");
    println!("{}", name);

    name.push_str(" sakti");
    println!("{}", name);

    let moko = name.replace("Dwi", "moko");

    println!("{}", moko);
}

#[test]
fn ownership_rules() {
    let a = 10;

    {
        let b = 20;
        println!("{}",b);
    }

    println!("{}",a);
// let a 10; kode rust dijalankan dari atas ke bawah
}

#[test]
fn data_copy() {
    let a = 10;
    let b = a;
    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1 = String::from("moko");

    let name2 = name1;

    println!("{}", name2);
}

#[test]
fn clone() {
    let name1 = String::from("moko");
    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}

#[test]
fn if_expression() {
    let value = 7;
    let result =  if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Not Bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very Bad"
    };

    println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;

        if counter > 10 {
            break;
        } else if counter %2 == 0 {
            continue;
        }

        println!("Counter : {}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter +=1;

        if counter > 10 {
            break counter * 2;
        }
    };
    println!("result : {}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop{
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number , i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter : {}", counter);
        }

        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array: [&str;5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value : {}", array[index]);
        index += 1;
    }
}

#[test]
fn for_loop() {
    let array: [&str;5] = ["A", "B", "C", "D", "E"];

    for value in array {
        println!("Value : {}", value);
    }
}

#[test]
fn range_exclusive() {
    let array: [&str;5] = ["A", "B", "C", "D", "E"];

    let range = 0..5;
    println!("Start: {}", range.start);
    println!("End: {}", range.end);

    for i in range {
        println!("Value : {}", array[i]);
    }
}

#[test]
fn range_inclusive() {
    let array: [&str;5] = ["A", "B", "C", "D", "E"];

    let range = 0..=4;
    println!("Start: {}", range.start());
    println!("End: {}", range.end());

    for i in range {
        println!("Value : {}", array[i]);
    }
}

// fn say_hello(first_name: &str, last_name: &str) {
//     println!("Hello {} {}", first_name, last_name);
// }

// #[test]
// fn test_say_hello() {
//     say_hello("moko", "sakti");
// }

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    return result;
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(5);
    println!("Result {}", result);

    let result = factorial_loop(-10);
    println!("Result {}", result);
}

fn print_text(value: String, times:u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value)
    }

    print_text(value, times -1);
}

#[test]
fn test_print_text() {
    print_text(String::from("moko"), 5);
}

fn factorial_recursive(n: u32) -> u32 {
    if n <=1 {
        return 1;
    }

    n * factorial_recursive(n-1)   
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("Result {}", result);
}

fn print_number(number: i32) {
    println!("number {}", number);
}

fn hi(name: String) {
    println!("name {}", name);
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("moko");
    hi(name);
    // println!("{}", name); borrow of moved value: `name`
}

fn full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("moko");
    let last_name = String::from("sakti");
    let full_name = full_name(first_name, last_name);
    println!("fullname {}", full_name);
    // println!("firstname {}", first_name);
    // println!("lastname {}", last_name);
}

// fn change_value(value: &String) {
//     value.push_str("test");
// }

// #[test]
// fn test_change_value() {
//     let mut value = String::from("moko");
//     change_value(&value);
//     println!("{}", value);
// }

// mutable reference
// fn change_value(value: &mut String) {
//     value.push_str("test");
// }

// #[test]
// fn test_change_value() {
//     let mut value = String::from("moko");
//     change_value(&mut value);
//     println!("{}", value);
// }

// fn get_full_name(first_name: &String, last_name: &String) -> &String {
//     let result = format!("{} {}", first_name, last_name);
//     return &result;
// }

// #[test]
// fn test_get_full_name() {
//     let first_name = String::from("moko");
//     let last_name = String::from("sakti");
//     let full_name = get_full_name(&first_name, &last_name);
//     println!("fullname {}", full_name);
//     // println!("firstname {}", first_name);
//     // println!("lastname {}", last_name);
// }

#[test]
fn slice_reference() {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice1: &[i32] = &array[..];
    println!("{:?}", slice1);

    let slice2: &[i32] = &array[0..5];
    println!("{:?}", slice2);

    let slice3: &[i32] = &array[5..];
    println!("{:?}", slice3);
}

#[test]
fn string_slice() {
    let name: String = String::from("moko sakti");
    let first_name: &str = &name[0..4];
    println!("{}", first_name);

    let last_name: &str = &name[5..];
    println!("{}", last_name);
}

struct Person{
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

impl Person{
    fn say_hello(&self, name: &str) {
        println!("hello{}, my name is {}", name, self.first_name);
    }
}

fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.middle_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

#[test]
fn struct_person() {
    let person: Person = Person{
        first_name: String::from("dwi"),
        middle_name: String::from("moko"),
        last_name: String::from("sakti"),
        age: 30,
    };

    print_person(&person);

    let person2: Person = Person{..person};
    print_person(&person2);
}

struct GeoPoint(f64, f64);

impl GeoPoint{
    fn new(long:f64, lat:f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn test_associated_function() {
    let geo_point: GeoPoint = GeoPoint::new(10.0, 10.0);
    println!("geopoint {}", geo_point.0);
    println!("geopoint {}", geo_point.1);
}

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-6.200000, 106.816666);
    println!("long : {}", geo_point.0);
    println!("lat : {}", geo_point.1);
}

struct Nothing;

#[test]
fn test_nothing() {
    let _nothing1: Nothing = Nothing;
    let _nothing2: Nothing = Nothing{};
}

enum Level{
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_enum() {
    let level: Level = Level::Regular;

    match level {
        Level::Regular => {
            println!("Regular");
        }
        Level::Premium => {
            println!("Premium");
        }
        Level::Platinum => {
            println!("Platinum");
        }
    }
}

enum Payment{
    CreditCard(String),
    BankTranksfer(String, String),
    EWallet(String, String),
}

impl Payment{
    fn pay(&self, amount: u32){
        match self{
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTranksfer(bank, number) => {
                println!("Paying with bank transfer {} {} amount {}", bank, number, amount);

            }
            Payment::EWallet(wallet, number) => {
                println!("Paying with e-wallet {} {} amount {}", wallet, number, amount);
            }

        }
    }
}

#[test]
fn test_payment() {
    let _payment1: Payment = Payment::CreditCard(String::from("423424242"));
    _payment1.pay(20);
    let _payment2: Payment = Payment::BankTranksfer(String::from("BCA"), String::from("23535353"));
    _payment2.pay(20);
    let _payment3: Payment = Payment::EWallet(String::from("Gopay"), String::from("23535353"));
    _payment3.pay(20);
}

#[test]
fn test_match_value() {
    let name = "Joko";

    match name{
        "moko" => {
            println!("hello moko");
        }
        "sakti" => {
            println!("hello sakti");
        }
        other => {
            println!("hello {}", other);
        }
    }

    match name{
        "moko" | "sakti" | "Joko" => {
            println!("hello bro");
        }
        
        other => {
            println!("hello {}", other);
        }
    }
}

#[test]
fn test_range_pattern() {
    let value = 100;
    match value {
        75..=100 => {
            println!("Great");
        }
        50..=74 => {
            println!("Good");
        }
        25..=49 => {
            println!("Not Bad");
        }
        0..=24 => {
            println!("Bad");
        }
        other => {
            println!("Invalid value {}", other);
        }
    }
}

#[test]
fn test_struct_pattern() {
    let point = GeoPoint(0.0, 1.0);
    match point {
        GeoPoint(long, 0.0) => {
            println!("long : {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("lat : {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("long :{}, lat :{}", long, lat);
        }
    }

    let person = Person{
        first_name: String::from("dwi"),
        middle_name: String::from("moko"),
        last_name: String::from("sakti"),
        age: 30,
    };

    match person {
        Person{first_name, last_name,..} => {
            println!("firstname: {}, lastname: {}", first_name, last_name)
        }
    }
}

#[test]
fn test_ignoring() {
    let point = GeoPoint(0.0, 1.0);
    match point {
        GeoPoint(long, _) => {
            println!("long : {}", long);
        }
    }
}

#[test]
fn test_match_expression() {
    let value: i32 = 2;
    let result: &str = match value {
        0 => "nol",
        1 => "satu",
        2 => "dua",
        _ => "invalid"
    };

    println!("result: {}", result);
}

type Age = u8;
type IdentityNumber = String;

struct Customer{
    id: IdentityNumber,
    name: String,
    age: Age
}

#[test]
fn test_type_alias() {
    let customer = Customer{
        id: String::from("12345"),
        name: String::from("moko"),
        age: 30,
    };

    println!("{} {} {}", customer.id, customer.name, customer.age);
}
struct Manusia {
    name: String,
    tinggi: u16
}

struct SimplePerson {
    name: String,
}

trait CanSayHello{
    // default method, yg mengimplementasi trait ini otomatis mempunyai method ini
    fn hello(&self) -> String {
        return String::from("Hello");
    }

    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String; 
}

trait CanSayGoodBye{
    fn good_bye(&self) -> String;
    fn good_bye_to(&self, name: &str) -> String; 
}

impl CanSayGoodBye for SimplePerson {
    fn good_bye(&self) -> String {
        format!("goodbye my name is {}", self.name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!("goodbye {}, my name is {}", name, self.name)
    }
}

impl CanSayGoodBye for Person {
    fn good_bye(&self) -> String {
        format!("goodbye my name is {}", self.first_name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!("goodbye {}, my name is {}", name, self.first_name)
    }
}

impl CanSayHello for Manusia {
    fn say_hello(&self) -> String {
        format!("hello my name is {}", self.name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("hello {}, my name is {}", name, self.name)
    }
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String {
        format!("hello my name is {}", self.first_name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("hello {}, my name is {}", name, self.first_name)
    }
    
}

fn say_hello_trait(person: &impl CanSayHello) {
    println!("{}", person.say_hello());
}

#[test]
fn test_trait() {
    let person: Person = Person{
        first_name: String::from("dwi"),
        middle_name: String::from("moko"),
        last_name: String::from("sakti"),
        age: 30,
    };

    let manusia: Manusia = Manusia{
        name: String::from("mokosakti"),
        tinggi: 100,
    };

    say_hello_trait(&person);
    say_hello_trait(&manusia);
    hello_and_goodbye(&person);

    let result = person.say_hello_to("budi");
    println!("{}", result);

    let result = person.hello();
    println!("{}", result);

    let result = manusia.hello();
    println!("{}", result);
}

fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodBye)) {
    println!("{}", value.say_hello());
    println!("{}", value.good_bye());
}

fn create_person(name: String) -> impl CanSayGoodBye {
    // belum bisa dynamic return
    // if name == String::from("moko") {
    //     SimplePerson{name}
    // } else {
    //     Person{
    //         first_name: name.clone(),
    //         middle_name: name.clone(),
    //         last_name: name.clone(),
    //         age: 30,
    //     }
    // }
    SimplePerson{name}
}

#[test]
fn test_impl_trait() {
    let person = create_person(String::from("saktisakti"));
    println!("{}", person.good_bye());
}

trait CanSay: CanSayHello + CanSayGoodBye {
    fn say(&self) {
        println!("{}", self.say_hello());
        println!("{}", self.good_bye());
    }
}

struct Point<T> {
    x: T,
    y: T,
}

#[test]
fn test_generic_struct() {
    let integer = Point::<i32> {
        x: 1, y: 2,
    };

    let float = Point::<f64> {
        x: 1.0, y: 2.0,
    };

    println!("{} {}", integer.y, integer.x);
    println!("{} {}", float.y, float.x);
}

enum Value<T> {
    NONE,
    VALUE(T)
}

#[test]
fn test_generic_enum() {
    let value: Value<i32> = Value::<i32>::VALUE(10);

    match value {
        Value::NONE => {
            println!("none")
        }
        Value::VALUE(value) => {
            println!("value {}", value)
        }
    }
}

struct Hi<T: CanSayGoodBye> {
    value: T
}

#[test] 
fn test_generic_bound() {
    let hi = Hi::<SimplePerson>{
        value: SimplePerson {
            name: String::from("Moko")
        }
    };
    println!("{}", hi.value.name);
}

fn min<T: PartialOrd>(value1: T, value2: T) -> T {
    if value1 < value2 {
        value1
    } else {
        value2
    }
}

#[test]
fn generic_in_function() {
    let result = min::<i32>(10, 20);
    println!("{}", result);

    let result = min(10.0, 20.0);
    println!("{}", result);
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

#[test]
fn test_generic_method() {
    let point: Point<i32> = Point {x:5, y:10};
    println!("x: {}", point.get_x());
    println!("y: {}", point.get_y());
}

trait GetValue<T> {
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for Point<T> {
    fn get_value(&self) -> &T {
        &self.x
    }
}


struct PointPoint<T = i32> {
    x: T,
    y: T,
}

#[test]
fn test_generic_trait() {
    let point: Point<i32> = Point {x:5, y:10};
    println!("x: {}", point.get_x());
    println!("y: {}", point.get_y());
    println!("y: {}", point.get_value());
}

use core::ops::Add;

struct Apple {
    quantity: i32,
}

impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output{
        Apple{
            quantity: self.quantity + rhs.quantity,
        }
    }
}

#[test]
fn test_operator_add() {
    let apple1 = Apple{quantity:10};
    let apple2 = Apple{quantity:10};

    let apple3 = apple1 + apple2;
    println!("{}", apple3.quantity);
}

fn double(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(i *2),
    }
}

#[test]
fn test_option() {
    let result = double(Some((10)));
    println!("{:?}", result);

    let result = double(None);
    println!("{:?}", result);
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn test_compare() {
    let apple1 = Apple{quantity:10};
    let apple2 = Apple{quantity:20};

    println!("Apple1 == Apple2: {}", apple1 == apple2);
    println!("Apple1 > Apple2: {}", apple1 > apple2);
    println!("Apple1 < Apple2: {}", apple1 < apple2);
}

#[test]
fn test_string_manipulation() {
    let s = String::from("Dwi atmoko purbo sakti");

    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());
    println!("{}", s.len());
    println!("{}", s.replace("Dwi", "moko"));
    println!("{}", s.contains("sakti"));
    println!("{}", s.starts_with("moko"));
    println!("{}", s.ends_with("sakti"));
    println!("{}", s.trim());
    println!("{:?}", s.get(0..4));
}

struct Category {
    id: String,
    name: String,
}

use std::fmt::Debug;

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

#[test]
fn test_formatting() {
    // let person = SimplePerson{
    //     name: String::from("moko")
    // };

    // println!("{}", person);

    let category = Category{
        id: String::from("1"),
        name: String::from("Category 1"),
    };

    println!("{:?}", category);
}

#[test]
fn test_closure() {
    let sum: fn(i32, i32) -> i32 = |value1: i32, value2: i32| -> i32  {
        value1 + value2
    };

    let result = sum(1, 2);
    println!("result: {}", result);
}