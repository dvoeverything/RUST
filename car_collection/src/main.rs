struct Car{
    make: String,
    model: String,
    price: u32,
}
struct CarCollection{
    cars: Vec<Car>,
    price_range: (u32, u32), //price range (min, max)
}

impl CarCollection{
    fn new(cars: Vec<Car>, price_range: (u32, u32))-> Self{
        Self{
            cars,
            price_range
        }
    }
}

//IntoIterator for iteration by value (CarCollection)
//IntoIterator for iteration by immutable borrow
fn main() {
    let cars = vec![
        Car {make: "Maruti Suzuki".to_string(), model: "Swift".to_string(), price: 8000},
        Car {make: "Honda".to_string(), model: "City".to_string(), price: 12000},
        Car {make: "Tata Motors".to_string(), model: "Mexon".to_string(), price: 10000},
        // add more cars if needed
    ];

    let car_collection1 = CarCollection::new(cars, (8000, 11000));
    // iterate over collection by value
    for car in car_collection1{

    }
}
