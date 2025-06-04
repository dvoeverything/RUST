use std::io::{self, Write};
fn main() {
    println!(
        "############# Set New Password #################
        1. atleast 1 letter between a-z
        2. atleast 1 number btween 0-9
        3. atleast 1 letter between A-Z
        4. atleast 1 character from [$#*]
        5. minimum length of password : 6
        6. Maximum length of password: 12
        
        Enter New Password: "
    );
    io::stdout().flush().expect("Failed to flush stdout");

    let password1: String = get_user_input();
    println!("Reenter your password: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let password2: String = get_user_input();

    if !password_match(&password1, &password2){
        return error_output();
    }
        
    if password_properties(&password1){
        return success_output();
    }else{
        return error_output();
    }
    
}
fn get_user_input()-> String{

    let mut buf = String::new();
    io::stdin()
    .read_line(&mut buf)
    .expect("Error");
    let input = buf.trim();
    input.to_string()

}
fn password_match(p1: &str, p2: &str)->bool{
        if p1 == p2 {
            println!("password are the same");
            return true;
       }else{
        println!("password not the same");
        return false;
       }

}

fn password_properties(password: &str)-> bool{
    if password.chars().count()<6 && password.chars().count()>12{
        println!("Password too short!");
        return false;
    }
    let mut has_digit = false;
    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_special = false; 

    for ch in password.chars() {
        match ch {
            '0'..='9'      => has_digit     = true,
            'A'..='Z'      => has_uppercase = true,
            'a'..='z'      => has_lowercase = true,
            '_'            => {},             
            _ if ch.is_ascii() && !ch.is_alphanumeric() =>
                               has_special  = true,
            _ => {}                           // has to be exhaustive
        }
      if has_digit && has_uppercase && has_lowercase && has_special {
            return true;
        }
    }

    false
}

fn success_output(){
    println!("Password Successfully updated!");

}
fn error_output(){
    println!("Password update failed!");
}

