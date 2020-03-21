#[derive(Debug)]
#[allow(non_camel_case_types)]

//defining struct
struct student{
    name:String,
    email:String,
    roll_no:i32,
    ab_flag:bool,
}

pub fn basic_struct()
{
    let stud = student{
        name:String::from("Ahire Dhananjay"),
        email:String::from("ahire.dhananjay12@gmail.com"),
        roll_no:2,
        ab_flag:true,
    };
    println!("Struct values:\n{:#?}",stud);

    let mut stud1 = student{
        name:String::from("Ahire Dhananjay"),
        email:String::from("ahire.dhananjay12@gmail.com"),
        roll_no:2,
        ab_flag:true,
    };

    stud1.email=String::from("ahire.dhananjay18@gmail.com");
    println!("----------------------\nStruct values:\n{:#?}",stud1);

    let x = String::from("ahire.dhananjay19@gmail.com");
    let y = String::from("Dhananjay Patil"); 
    let stud2 = get_details(x,y);
    println!("----------------------\nStruct values:\n{:#?}",stud2);

    let stud2 = get_details("ahire.dhananjay20@gmail.com".to_string(),"Dhananjay Patil".to_string());
    println!("----------------------\nStruct values:\n{:#?}",stud2);

}

fn get_details(email_id:String, name_1:String)-> student{
    student{
        name:name_1,
        email:email_id,
        roll_no:4,
        ab_flag:false,
    }
}
