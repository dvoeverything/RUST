enum ButtonData{
        Count(i32),
        Message(String),
}
struct Button <F> where F: Fn(&mut ButtonData){
        click_handler: F,
        button_data: ButtonData
}

impl<F> Button<F> where F: Fn(&mut ButtonData){
        fn new(click_handler: F, button_data: ButtonData)->Self {
                Button{
                        click_handler,
                        button_data,
                }
        }

        fn click(&mut self){
                (self.click_handler)(&mut self.button_data)
        }

        fn set_message(&mut self, message: String){
                self.button_data = ButtonData::Message(message)
        }
}

fn main(){
        let mut subscribe_btn = Button::new(|btn_data|
        if let ButtonData::Count(sub_count)= btn_data{
                *sub_count += 1;
                println!("Subscribed!! total subscription {}", sub_count); 
        }, ButtonData::Count(0));
        subscribe_btn.click();
        subscribe_btn.click();
        subscribe_btn.click();
        let mut send_btn = Button::new(|btn_data|
        if let ButtonData::Message(msg)= btn_data{
                println!("Your message sent: {}", msg);
        }, ButtonData::Message(String::new()));

        send_btn.set_message("Hello from the other side".to_string());
        send_btn.click();


}