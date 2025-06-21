struct TaxCalculator{
    calculation: Box<dyn Fn(f32) -> f32>,
}
impl TaxCalculator{
    fn new(calculation: Box<dyn Fn(f32)-> f32>)-> TaxCalculator{
        TaxCalculator{calculation}
    }

    fn calculate(&self, amount: f32)->f32{
        (self.calculation)(amount)
    }
}
fn main() {
    let vat_calculator = TaxCalculator::new(Box::new(|amount| amount*0.2));
    let income_tax_calculator = TaxCalculator::new(Box::new(|amount| amount*0.3));
    println!("{}", vat_calculator.calculate(10000f32));
    println!("{}", income_tax_calculator.calculate(10000f32));
}
