mod developers;
mod hotels;
mod menus;

use clearscreen::ClearScreen;
use developers::developers as Developer;
use hotels::hotels as Hotel;
use menus::menus as Menu;
use std::io;

fn main() {
    let input     : io::Stdin = io::stdin();
    let developers: Vec<Developer::Developer> = Developer::generate_developers();
    
    loop{
        ClearScreen::default().clear().expect("failed to clear the screen");
        let mut menu_number: String = String::new();
        println!("{}", Menu::MAIN_MENU);
        let _ = input.read_line(&mut menu_number);
        let menu_number: Result<u8, std::num::ParseIntError> = menu_number.trim().parse::<u8>();
        let menu_number: u8 = match menu_number {
            Ok(n) => n,
            Err(_) => 0,
        };
        if menu_number == 0 {
            println!("Invalid option you entered");
            break;
        }
        if menu_number == 3 {
            println!("Exited successfully");
            break;
        }
        // Developer list here
        if menu_number == 1 {
            Developer::menus(&input);
        }
        // Hotel list here
        if menu_number == 2 {
            Hotel::menus(&input);
        }
    }
}
