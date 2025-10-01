use std::io;
use enigo::{Enigo, MouseButton, MouseControllable};
use std::{thread, time::Duration};



fn main(){
    print!("Enter Delay: ");
    let mut del4y = String::new();
    io::stdin().read_line(&mut del4y).expect("Failed to read line");
    let del4y:u64 = del4y.trim().parse().expect("Please enter a number");

    let mut enigo = Enigo::new();
    let delay = Duration::from_millis(del4y);

    print!("Enter limit of presses (0 = infinite): ");
    let mut limit = String::new();
    io::stdin().read_line(&mut limit).expect("Failed to read line");
    let mut limit:i32 = limit.trim().parse().expect("Please enter a number");

    if limit == 0{
        loop {
            enigo.mouse_click(MouseButton::Left);
            thread::sleep(delay);
        }
    } else {
        while 0 > limit{
            enigo.mouse_click(MouseButton::Left);
            thread::sleep(delay);
            limit -= 1;
        }
    }
}