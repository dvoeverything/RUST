
use std::io::{self,Write};
enum AddStringError{
    EmptyString,
    LengthMismatch,
}

impl AddStringError{
    fn description(&self)->String{

        match self{
            AddStringError::EmptyString => "The string is empty".to_string(),
            AddStringError::LengthMismatch => "The lengths of the string mismatch".to_string(),
        }
       
    }


}

fn add_strings(s1:&str, s2: &str)->Result<String, AddStringError>{
    if s1.len()!= s2.len(){
        return Err(AddStringError::LengthMismatch);
    }
    if s1.is_empty() || s2.is_empty(){
        return Err(AddStringError::EmptyString);
    }
    Ok(format!("{} {}", s1, s2))
    
}
fn main() {
    println!("Enter your first String");
    let s1  = get_user_input();
    println!("Enter your first String");
    let s2 = get_user_input();


    match add_strings(&s1, &s2) {
        Err(e) => println!("We got an error: {}",AddStringError::description(&e)),
        Ok(v) => println!("Result: {}",v ),
    }
}

fn get_user_input()->String{
    io::stdout().flush().expect("flush failed");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()

}
