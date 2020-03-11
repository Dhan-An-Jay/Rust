fn main() {

    /*
    Memories allocating :
    1. heap - dynamic type of data types are allocating memories into heap -m string , Clone - deep copy
    2. stack - fixed size data allocated into stack, copy - deep copy

    Fixed Size data types :
    1. All the integer types, such as u32.
    2. The Boolean type, bool, with values true and false.
    3. All the floating point types, such as f64.
    4. The character type, char.
    5. Tuples, if they only contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not.
    */

    //Borrowing Fixed type value 
    let a:i32 =10;
    let b =a;

    // above values are get created inside stack example
    // a =10;
    // b =10;

    println!("values of a : {}  and b :{}", a,b );    //values of a : 10  and b :10

    let mut str_value =  String::from("ahire");
    str_value = String::from(" dhananjay");
    str_value.push_str(" ahire");

    let string_dummy = &str_value;  // borrowing read only memory accesss
    println!("{}",str_value);

    let string_dummy = &mut str_value; // borrowing mutable memory accesss
    string_dummy.push_str(" patil");
    println!("{}",str_value);

    //shallow copy
    let str_new:String = str_value;  // deallocate memory of str_value and allocate same memory to str_new
    println!("{}",str_new);          // as per at time one owner role, deallocate owener str_value and make str_new owner 

    //deep copy
    let str1:String = str_new.clone(); // deep copy is clone
    println!("cloned String  : {}",str1);    
    
    //copy fixed values 
    let x:i32 =30;
    let y = x.copy();
    println!("copy x {} to y value is : {}",y);   

}
