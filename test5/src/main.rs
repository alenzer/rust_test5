use std::io;
use test3::main::{get_products};
use test4::main::{calc_balance, input_funds};

pub fn select_product() -> (bool, f32, bool) {
   let products = get_products();
   loop {
      println!("-------Products-------");
      for i in 0..products.len() {
         println!("{} : {:.2}", products[i].title, products[i].value);
      }

      let mut input = String::new();
      println!("Select the Product(1..10) or Input 0 to Service Menu");

      io::stdin()
         .read_line(&mut input)
         .expect("Not a valid string");

      if input.trim() == "q" {
         return (true, 0.0, false);
      }

      let res = input.trim().parse::<i32>();
      if res.is_ok() {
         let index = res.unwrap() as usize;
         if index == 0 {
            return (false, 0.0, true);
         } else if index < products.len() {
            println!("Price: {}", products[index-1].value);
            return (false, products[index-1].value, false);
         }
      }
   }
}
pub fn calc() -> (bool, [i32; 8], f32, bool) {
   let (quite, price, service) = select_product();
   if quite || service {
      return (quite, [0; 8], 0.0, service);
   }
   loop {
      let (quite, coin_input, funds) = input_funds();
      if quite {
         return (quite, coin_input, funds, false);
      }

      if funds < price {
         println!("Not Sufficient Fund! Input again!");
      } else {
         let remain = funds - price;
         return (false, coin_input, remain, false);
      }
   }
}

fn main() {
   let mut coin_balance: [i32; 8] = [20; 8];

   loop {
      let (quite, coin_input, remain, service) = calc();
      if quite {
         break;
      }
      if service {
         let (quite, coin_input, _) = input_funds();
         if quite {
            break;
         }
         coin_balance = coin_input;
      } else {
         let (skip, balance) = calc_balance(coin_balance, coin_input, remain);
         if skip {
            continue;
         }
         coin_balance = balance;
      }
   }
}
