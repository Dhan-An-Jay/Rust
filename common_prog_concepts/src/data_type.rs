// data types implementation is done inside this file for basic understandings
// programmer name  : Dhananjay Ahire 
// Basics of rust data types

use std::io;
#[allow(unused_imports)]
//#[allow(non_snake_case)]
fn data_conversion()
{
    // parse is used to convert a string into number
    let  x:u32 =10;
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("error");    // from user get string 
    //y.insert_str(12, "20");   // assign a string or simply initialize a string object with a new string value 
    //let x =  y.parse().expect_err("error");
    println!("value of X:{} and Value of Y: {}", x, y);
}
fn int_datatypes()
{
    /*
    i8/u8
    i16/u16
    i32/u32
    i64/u64
    i128/u128
    isize/usize -- based on arch of Processor it will get allocated to 32 or 64 bit

    i: size of signed integer is = -(2^(n-1)) to 2^(n-1)-1
    u: size of unsigned integer is = 2^(n)-1
    */
    let a:u8 =10;
    //let b:u8 =-10; // will show error becasue its a unsigned datatype so we'll not able to assigned signed number to it - compile time error
    let b:i8 =-10;
    println!("unsignd number a:{} and signed number b:{}",a,b);
}
fn float_datatype()
{
    /*
    f32/f64 data types for float
    Default float type is 64 bit -i.e f64
    */
    let a:f32 = 2.0; // defined f32
    let b:f64 = 3.0; // defined f64
    let c = 4.0; // default 64 bit float data types 
    println!("Float numbers a:{}, b:{} and c:{}",a,b,c);
}
fn bool_datatype()
{
    /*
    boolean data types : bool
    */
    let a:bool =  true;
    let b = false;
    println!("Bolean a:{} and Bolean b:{}",a,b);

}
pub fn basics_datatypes()
{
    data_conversion();
    println!("----------------------------");
    int_datatypes();
    println!("----------------------------");
    float_datatype();
    println!("----------------------------");
    bool_datatype();
}