pub fn control_flow_basics_imp()
{
    if_control_flow();
    println!("----------------------------");   
    
}

fn if_control_flow()
{
    let i =3;
    if i < 5{
        println!("yes, value is lesser than 3");
    }
    else{
        println!("No, value is not lesser than 3");
    }

    let number =3;
    if number !=5{
        println!("Number value is not equals to 5");
    }
    else{
        println!("Number value is equals to 5");
    }

    // nested  if else block
    let number = 16;

    if number % 5 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //using if in let statement used
    let condition = 3>2;
    let xy = if condition{  // or if 3>2
        3
    }
    else{
        2
    };
    println!("XY values is :{}",xy);


    //loopin
    /*loop {
        println!("again!");
    }*/
}