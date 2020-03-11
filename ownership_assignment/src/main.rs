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
    
    //copy fixed values - 
    let x:i32 =30;
    let y:i32 = x;
    println!("copy x {} to y value is : {}",x,y);   

    let p:String = String::from("hallo");
    let q:String = p;

    println!("Size of string is : {}",cal_string_len(&q));
    let d  = return_string_reference(q);
    // println!("Size of string is : {}",cal_string_len(&q));  after above move we can not use q because q is moved to d

    // slices
    slices_outpuy();
}
fn cal_string_len(str1 :&String) -> usize{
    str1.len()
}

fn return_string_reference(str1:String)->String{
   str1
}

fn slices_outpuy(){
    // slices can not borrow as a mutable 
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("value of Hello is: {}, Value of World: {}",hello, world);

    println!("first word from value :{}",first_word(&s));

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    println!("slice value :{:?}",slice);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}