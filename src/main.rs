use core::slice;

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

fn say_hello(first_name: &str, last_name: &str) {
    println!("Hello {} {}", first_name, last_name);
}

#[test]
fn test_say_hello() {
    say_hello("moko", "sakti");
}

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