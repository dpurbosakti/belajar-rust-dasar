use std::num::ParseIntError;


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