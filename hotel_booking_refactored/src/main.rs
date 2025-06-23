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
    booking_iter: std::slice::iter<'a, Booking >,
}
impl <'a> BookingOnDate<'a>{
    fn new(date: &'a str, all_bookings: &'a Vec<Booking>)-> Self{
        BookingOnDate{ date , all_bookings.iter()}
    }
}
struct BookingOnDateMut<'a>{
    date: &'a str,
    booking_iter: std::slice::iter_mut<'a, Booking >,
}



impl <'a> BookingOnDateMut<'a>{
    fn new(date: &'a str, all_bookings: &'a mut Vec<Booking>)-> Self{
        BookingOnDate{ date , all_bookings.iter_mut()}
    }
}
impl <'a> Iterator for BookingOnDate <'a>{
    type Item =  &'a Booking;
    fn next(&mut self)-> Option<Self::Item>{
        self.booking_iter.find(|booking| self.date == booking.date) // type of booking is &&Booking
    }
}
impl <'a> Iterator for BookingOnDateMut <'a>{
    type Item =  &'a Booking;
    fn next(&mut self)-> Option<Self::Item>{
        self.booking_iter.find(|booking| self.date == booking.date) // type of booking is &&Booking
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

    let bookings2 = BookingOnDateMut::new("2023-10-30", &mut all_bookings);
    for booking in bookings2{
            println!("{:?}", booking);
            booking.room_number = 100;
    }

}
