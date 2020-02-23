fn main() {
    println!("----------------------------");
    variable_immutable();
    println!("----------------------------\nMutable variable");
    variable_mutable();
    println!("----------------------------");
}

fn variable_immutable()
{
    let i =10;
    println!("i value = {}", i);
    // Below line will give error 
    // The error message indicates that the cause of the error is that you cannot assign twice to immutable variable x, 
    // because you tried to assign a second value to the immutable x variable.
    //i=20;    // default variable is immutable so we are not able to reallocate the the values for immutabel object
}

fn variable_mutable()
{
    let  mut j =10;
    println!("j value = {}", j);
    j =20;
    println!("reallocated j value = {}", j);
}