use std::io;
use clearscreen::ClearScreen;
use rand::{thread_rng, Rng};
use crate::menus::menus as Menu;

#[derive(Debug)]
pub struct Hotel{
    id      : i16,
    name    : String,
    capacity: u8,
    range   : i32,
    location: String,
}

pub fn menus(input: &io::Stdin){
  loop{
    ClearScreen::default().clear().expect("failed to clear the screen");
    println!("{}", Menu::HOTEL_MENU);
    let mut menu_number: String = String::new();
    let _ = input.read_line(&mut menu_number);
    let menu_number: Result<u8, std::num::ParseIntError> = menu_number.trim().parse::<u8>();
    let menu_number: u8 = match menu_number {
        Ok(n) => n,
        Err(_) => 0,
    };
    
    if menu_number == 3 {
      break;
    }

  }
}

pub fn generate_hotels() -> Vec<Hotel> {
    let mut hotels = Vec::new();
  
    for _ in 0..10 {
        let mut rng = thread_rng();
        let id = rng.gen::<i16>();
        let name = String::from("Pak Ghor");
        let capacity = rng.gen_range(20..=35);
        let range = rng.gen_range(20..=35);
  
        let location = 
            match rng.gen_range(0..3) {
                0 => String::from("Khulna"),
                1 => String::from("Dhaka"),
                2 => String::from("Rajsahi"),
                _ => unreachable!(), 
            };
  
        hotels.push(Hotel { id, name, capacity, range, location });
    }
    hotels
  }