use rand::Rng;
use std::io;

pub fn get_price() -> f32 {
   let mut rng = rand::thread_rng();
   let mut price: f32 = rng.gen_range(1.0..10.0) * 100.0;
   price = price.floor() / 100.0;
   println!("Price: {}", price);

   price
}
pub fn input_funds() -> (bool, f32) {
   loop {
      let funds: f32;
      let mut input = String::new();
      println!("Enter funds: ");
      io::stdin()
         .read_line(&mut input)
         .expect("Not a valid string");

      if input.trim() == "q" {
         return (true, 0.0);
      }

      let res = input.trim().parse::<f32>();
      if res.is_ok() {
         funds = res.unwrap();
         return (false, funds);
      }
      println!("Invalid Input Format! Input again!");
   }
}
pub fn calc() -> (bool, f32) {
   let price = get_price();

   loop {
      let (quite, funds) = input_funds();
      if quite {
         return (quite, funds)
      }

      if funds < price {
         println!("Not Sufficient Fund! Input again!");
      } else {
         let remain = funds - price;
         return (false, remain)
      }
   }
}
fn main() {
   loop {
      let (quite, remain) = calc(); 
      if quite {
         break;
      }

      println!("Remain funds: {:.2}", remain);
      println!("");
   }
}
