// data types implementation is done inside this file for basic understandings
// programmer name  : Dhananjay Ahire 
// Basics of rust data types
/*
1. scalar types 
   a. int
   b. float
   c. char
   d. number
   e. bool
2. compound types 
   a. tuple
   b. array 
*/
use std::io;
#[allow(unused_imports)]
#[allow(dead_code)]
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
fn char_datatypes()
{
    let a = 'A';
    let b = '💣';
    let c:char ='😻';
    println!("values a:{}, b:{} and c:{}",a,b,c);
}
pub fn basics_datatypes()
{
    //data_conversion();
    //println!("----------------------------");
    int_datatypes();
    println!("----------------------------");
    float_datatype();
    println!("----------------------------");
    bool_datatype();
    println!("----------------------------");
    char_datatypes();
}
fn tuple_datatypes()
{
    // tuple can content different set of values
    // tuples structure can not be srinked, its a fixed size, we can not manipulate in size of tuple
    let tup =(1,2.3,4);
    println!("Tuple values tup1:{}, tup2:{} and tup3:{}",tup.0,tup.1,tup.2);
    //tup.2 =1.1; // by default its a immutable in nature so we can not reassign values if its is immutable


    let tuple1:(i32,f64,u8) =(-80,56.6,5);
    println!("Tuple values tuple1:{}, tupple2:{} and tuple3:{}",tuple1.0,tuple1.1,tuple1.2);

    let (x,y,z) = tuple1;
    println!("Tuple values x:{}, y:{} and z:{}",x,y,z);

    //mutable tuple - Reassign a value
    let mut tup1:(i32,f64,u8) =(-1,1.1,111);
    println!("Tuple values tup1:{}, tup2:{} and tup3:{}",tup1.0,tup1.1,tup1.2);
    tup1.1=2.2;
    println!("Tuple values tup1:{}, tup2:{} and tup3:{}",tup1.0,tup1.1,tup1.2);
    //tup1.3 =4; not able to go beyond its size
}
fn arrays_datatype()
{
    // Allocate size 5 to array and initialize those 5 slots with 3
    let arr1=[3;5];
    for i in 0..5{
        println!("at {} Possition {}",i+1,arr1[i]);
    }
    println!("----------------------------");
    // definied array with 5 size and iniitalize it
    let arr2:[i32;5] =[11,12,13,14,15];
    for i in 0..5{
        println!("at {} Possition {}",i+1,arr2[i]);
    }

   
    /*
    for i in 0..6{ // array size is 5 and we are printing up to 6
        println!("at {} Possition {}",i+1,arr1[i]);
    }

    DhanAnJays-MacBook:common_prog_concepts dhananjay$ cargo run
    thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 5', src/data_type.rs:120:43
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

    The compilation didn’t produce any errors, but the program resulted in a 
    runtime error and didn’t exit successfully. When you attempt to access an 
    element using indexing, Rust will check that the index you’ve specified is 
    less than the array length. If the index is greater than or equal to the array length, Rust will panic
    */
}
pub fn basic_compound_datatypes()
{
    println!("----------------------------");
    tuple_datatypes();
    println!("----------------------------");
    arrays_datatype();

}