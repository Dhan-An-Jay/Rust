
pub const CONST_VARIABLE:u32 =3000;
pub fn variable_immutable()
{
    let i =10;
    println!("i value = {}", i);
    // Below line will give error 
    // The error message indicates that the cause of the error is that you cannot assign twice to immutable variable x, 
    // because you tried to assign a second value to the immutable x variable.
    //i=20;    // default variable is immutable so we are not able to reallocate the the values for immutabel object
}

pub fn variable_mutable()
{
    let  mut j =10;
    println!("j value = {}", j);
    j =20;
    println!("reallocated j value = {}", j);
}
pub fn shadow_variable()
{
    let shadow_variable1= "dhananjay";
    // shadow variables shoud be immutable 
    // we can change its type anywhere as a shadow variable if it immutable
    let shadow_variable1=shadow_variable1.len();   
    println!("Len of shadow_variable is :{}",shadow_variable1);
}
pub fn const_variables()
{
    // dont use let keyword if you are declaring any const variable 
    // if possible the use variable name in capital
    // Const variable will not allow to assign value at runtime . ex, if you are assigning return value of any function to const variable
    const CONST_VARIABLE:u32 =2000;
    println!("Const Variable 'CONST_VARIABLE' value is:{} ",CONST_VARIABLE);
}