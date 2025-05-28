use std::io::{self, Write};
fn main() {
    println!("Please enter the time in seconds: ");
    io::stdout().flush().unwrap();
    let time_seconds = get_user_input();

    println!("{}", military_time(time_seconds));
}

fn military_time(input_seconds: i32) -> String {
    if input_seconds < 0 || input_seconds > 86399 {
        panic!("Your input should be between 0  and 86,399");
    }
    let hours = input_seconds / 3600;
    let mut seconds = input_seconds % 3600;
    let minutes = seconds / 60;
    seconds = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn get_user_input() -> i32 {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Enter valid time in seconds!");

    let time_seconds: i32 = buf.trim().parse().expect("enter a valid integer");
    return time_seconds;
}
#[cfg(test)] // shows that these are test cases, ignored when running the code in production 
mod tests {
    mod time_format {
        //keep all the test cases to unit test military_time
        //1. When time_seconds is zero , function must return 00:00:00
        #[test]
        fn midnight(){
            assert_eq!(super::super::military_time(0),"00:00:00");
        }
        //2. When time_seconds is 86400, function must panic
        #[test]
        #[should_panic(expected = "Your input should be between 0  and 86,399")]
        fn too_large(){
            super::super::military_time(86400);
        }
        //3. When time_seconds is 86399, function must return 23:59:59
        #[test]
        fn last_second(){
            assert_eq!(super::super::military_time(86399), "23:59:59");
        }
        //4. When total_seconds is -ve, function must panic
        #[test]
        #[should_panic(expected = "Your input should be between 0  and 86,399")]
        fn negative(){
            super::super::military_time(-1);
        }
    }
//     mod user_input {
//     // keep all the test cases to unit test get_user_input
//     //. User enters 0 , function must return 0
//     // #[test]
//     // fn test_user_enters_zero_function_must_return_zero(){
//     //     assert_eq!(0, super::super::get_user_input("0\n\r".to_string()));
//     // }

//     //2. When user enters whole number, function must return whole number
//     //3. When user enters negatiove number function must panic
//     // 4. When user enter a decimal function must panic
//     // 5. When user enter any other character other than a number fuction must panic
//     //6. When user enters nothing and just presses enter function must panic
//     //7. When user enters a number which is greater than max value of u32, function must panic
// }
}
