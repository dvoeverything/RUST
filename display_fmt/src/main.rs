
use std::fmt;
struct Dog{
    weight: u8,
    age: u8,
    name: String,
}

impl fmt::Display for Dog{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)->fmt::Result{
        writeln!(f,"{:#^17}","Dog Details")?;
        writeln!(f, "{:<8}{:>8}", "Age", self.age)?;
        writeln!(f, "{:<8}{:>8}", "Weight", self.weight)?;
        writeln!(f, "{:<8}{:>8}", "Name", self.name)?;
        Ok(())


    }
}
fn main() {
    let my_dog = Dog{
        weight:2,
        age: 2,
        name: "Bow Wow".to_string(),
    };
    println!("{}", my_dog);
}
