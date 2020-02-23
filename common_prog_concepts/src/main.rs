mod data_type;      // basics of data types are included into this file 
mod shadow_mutable_immutable;  // all immutable and mutable and shadow variable concept are implemented into this file 

fn main() {
    println!("----------------------------");
    shadow_mutable_immutable::variable_immutable();
    println!("----------------------------\nMutable variable");
    shadow_mutable_immutable::variable_mutable();
    println!("----------------------------");
    shadow_mutable_immutable::shadow_variable();
    println!("----------------------------");
    shadow_mutable_immutable::const_variables();
    println!("----------------------------");
    // Global constant and local constanct are in there scope so you can use same name for both of them
    println!("Const Variable 'CONST_VARIABLE' value is:{} ",shadow_mutable_immutable::CONST_VARIABLE);
    println!("----------------------------");
    data_type::basics_datatypes();

}
