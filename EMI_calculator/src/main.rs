use std::io;
fn main() {
    let mut input = String::new();
    let currency_symbol = '$';

    println!("Please enter the Principal!");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let principal: f64 = input.trim().parse().expect("input number only!");
    input.clear();

    println!("Please enter the annual rate!");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline");
    let annual_rate: f64 = input.trim().parse().expect("input number only!");
    input.clear();

    println!("Please Enter the number of months!");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to readline");
    let months: u32 = input.trim().parse().expect("Input number only!");
    input.clear();

    let emi = calculate_emi(principal, annual_rate, months);
    //let calculated_emi = format!("The EMI is equal to: {currency_symbol}{emi}",currency_symbol = currency_symbol, emi = emi);
    println!(
        "The EMI is equal to: {currency_symbol}{emi:.2}",
        currency_symbol = currency_symbol,
        emi = emi
    );
}
/*
   Formula for calculating the Loan EMI is: EMI = [P x R x (1+R)^N]/[(1+R)^N-1]
   where P is the principal loan amount
   R is the monthly interest rate (annual rate divided by 12)
   N is the number of monthly instalments or the loan tenure in months.
*/
fn calculate_emi(principal: f64, annual_rate: f64, months: u32) -> f64 {
    let monthly_rate = annual_rate / 12.0 / 100.0; //convert percentage to a decimaland annual to monthly
    let numerator = principal * monthly_rate * (1.0 + monthly_rate).powf(months as f64);
    let denominator = (1.0 + monthly_rate).powf(months as f64) - 1.0;
    numerator / denominator
}
