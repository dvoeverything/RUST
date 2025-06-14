fn length_greater(s : &mut String)-> bool{
    if s.len()>6{
        
        s.push(':'); 
        s.push_str(&s.len().to_string());
        true
    }else{
        false
    }

}
fn length_lesser(s : &&str)-> bool{
    s.len()<6

}
fn main() {
    // retain those whose length is less than 6
    let mut planets = vec![
        "Mecury", "Jupiter","Earth", "Saturn", "Mars"
    ];

    planets.retain(length_lesser);
    println!("{:?}", planets);

    //retain those whose length is greater than 6
    // and modify the retained elements by appending length information
    //eg Hoth: 4
    let mut star_wars_planets = vec![
        String::from("Tatooine"),
        String::from("Coruscant"),
        String::from("Hoth"),
        String::from("Dagobah"),
        String::from("Mustafar"),
    ];

    star_wars_planets.retain_mut(length_greater);
    println!("{:?}", star_wars_planets);
}
