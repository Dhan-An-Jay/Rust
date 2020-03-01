/*
Function Types
1. Paramterized function
2. function with return type
3. function wirh paramter and return type 
*/

fn child_function()
{
    println!("child function");
}
//function with paramters
fn function_with_parameter(x:i32, y:f32)
{
    println!("param 1 X:{} and Param Y:{}",x,y);
}
fn function_continents_experession()
{
    //let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
//function  with reyutrn typs
fn return_value_function(x:i32) ->i32{
    return x+1;
}
fn return_value_five() ->i32{
   return  5;
}
pub fn functions_types()
{
    println!("----------------------------");
    println!("main function");
    child_function();
    println!("----------------------------");
    function_with_parameter(10, 20.2);
    println!("----------------------------");   
    function_continents_experession();
    println!("----------------------------");   
    println!("return value of funciton is :{}",return_value_function(3));
    println!("return value of funciton is :{}",return_value_five());
    println!("----------------------------");   
}