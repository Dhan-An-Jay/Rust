/*
primitive data types 
integer:  u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Float: f32, f64
Boolean: (bool)
chararcter (char)
Tuples
Arrays
*/

pub fn run()
{
    // Variable Declaration in Rust
    let name = "dhananjay";
    let mut age = 10;
    age =20;
    println!("My name is {} and gage of mine is {}",name , age);

    //constant
    const ID:i32 =  1001;
    println!("Employee ID is {}",ID );

    let (name, age) =("ahire",23);
    println!("Name :{}, age: {}",name, age );

    //Primitive data types 
    let a:i8 = 10;
    println!("Hi a Value is {}",a );

    let a_float:f32 =  201.222;
    println!("Float value is :{}", a_float );

    println!("Size  of Int: {} \nSie of Float: {}",std::i32::MAX, std::f32::MAX );

    let a_bool:bool = true;

    let face = '\u{1f600}';

    println!("{:?}",(age,name,a,a_float,a_bool, face) );


    //Strings Basics
    /// Primitive Strings =  imutable , FIxed Length Somewhere in memory
    /// String =  Growable, Heap allocated  data structure, Modify String and own it 
    /// /0 is not appended in memory and not wested by one Byte
    let mut hello = String::from("Hello ");
    println!("Size of Hello String is  : {}", hello.len() );
    hello.push_str("Hello World ");
    println!("{}",hello );
    println!("Size of Hello String is  : {}", hello.len() );
    hello.push('h');
    hello.push('i');
    println!("{}",hello );
    println!("Size of Hello String is  : {}", hello.len() );

    // Working on the strings for the basiics
}