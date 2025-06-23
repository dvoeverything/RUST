#[derive(Debug)]
struct Booking{
    date: String, 
    guest_name: String,
    room_number: u32,
}

impl Booking{
    fn new(date: String, guest_name: String, room_number: u32)-> Self{
        Self{
                date,
                guest_name,
                room_number,
        }
    }
}

#[derive(Debug)]
struct BookingOnDate<'a>{
    date: &'a str,
    all_bookings: &'a Vec<Booking>,
    index: usize,
}

impl <'a> BookingOnDate<'a>{
    fn new(date: &'a str, all_bookings: &'a Vec<Booking>)-> Self{
        BookingOnDate{ date , all_bookings, index:0}
    }
}

impl <'a> Iterator for BookingOnDate <'a>{
    type Item =  &'a Booking;
    fn next(&mut self)-> Option<Self::Item>{
        while self.index < self.all_bookings.len(){
            let booking = &self.all_bookings[self.index];
            self.index += 1;
            if self.date ==booking.date{
                return Some(booking)
            }
        }
        None
    }
}
fn main() {
    let mut all_bookings: Vec<Booking> = Vec::new();

    let booking_1 = Booking::new("2023-10-30".to_string(), "Mr. X".to_string(), 103);
    let booking_2 = Booking::new("2023-10-30".to_string(), "Mr. Y".to_string(), 122);
    let booking_3 = Booking::new("2023-11-03".to_string(), "Mr. Z".to_string(), 145);

    all_bookings.push(booking_1);
    all_bookings.push(booking_2);
    all_bookings.push(booking_3);

    let bookings = BookingOnDate::new("2023-10-30", &all_bookings);
    for booking in bookings{
            println!("{:?}", booking);
    }



}
