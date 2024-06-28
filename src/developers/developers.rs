use std::{io, ptr::null};
use Default;
use clearscreen::ClearScreen;
use rand::{thread_rng, Rng};

use crate::menus::menus as Menu;

#[derive(Debug)]
pub struct Developer{
    id  : i16,
    name: String,
    age : u8,
    location: String,
}

static mut developerData: Option<&'static Vec<Developer>> = None;

pub fn menus(input: &io::Stdin){
  loop{
    ClearScreen::default().clear().expect("failed to clear the screen");
    println!("{}", Menu::DEVELOPER_MENU);
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
    
    if menu_number == 1 {
      listOfDeveloper();
    }

  }
}

pub fn developerStore(developer: &Vec<Developer>)
{
  unsafe{
    developerData = Some(developer)
  }
}

fn listOfDeveloper()
{
  unsafe{
    let developerLen: usize = developerData.unwrap().len();
    for index in 0..developerLen{
        println!("{:?}", developerData.unwrap()[index]);
    }
  }
  true;
}

pub fn generate_developers() -> Vec<Developer> {
  let mut developers = Vec::new();

  for _ in 0..10 {
      let mut rng = thread_rng();
      let id = rng.gen::<i16>();
      let name = String::from("Dev Name ");
      let age = rng.gen_range(20..=35);
      let location = String::from("Khulna");

      let location = 
          match rng.gen_range(0..3) {
              0 => String::from("Khulna"),
              1 => String::from("Dhaka"),
              2 => String::from("Rajsahi"),
              _ => unreachable!(), 
          };

      developers.push(Developer { id, name, age, location });
  }
  developers
}
