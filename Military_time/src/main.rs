use std::io::{self, Write};
fn main() {
    println!("Please enter the time in seconds: ");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin()
    .read_line(&mut buf)
    .expect("Enter valid time in seconds!");

    let time_seconds: i32 = buf.trim().parse().expect("enter a valid integer");
    println!("{}",military_time(time_seconds));

}

fn military_time(input_seconds: i32)->String{
    let hours = input_seconds/3600;
    let mut seconds = input_seconds%3600 ;
    let minutes = seconds/60;
    seconds = seconds%60;
    format!("{:02}:{:02}:{02}", hours,minutes,seconds)
}
