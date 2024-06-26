use std::io;
use clearscreen::ClearScreen;
use crate::menus::menus as Menu;

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