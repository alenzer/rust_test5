use std::io;
use test2::main::{calc_by_coin, get_coins};
use test3::main::get_price;

pub fn input_funds() -> (bool, [i32; 8], f32) {
   loop {
      println!("Enter funds");

      let coins = get_coins();
      let mut coin_input: [i32; 8] = [0;8];
      let mut funds: f32 = 0f32;

      for i in 0..coins.len() {
         loop {
            let mut input = String::new();
            println!("{:.2} COIN : ", coins[i]);
            io::stdin()
               .read_line(&mut input)
               .expect("Not a valid string");

            if input.trim() == "q" {
               return (true, [0; 8], 0.0);
            }
            if input.trim() == "" {
               break;
            }
            let res = input.trim().parse::<i32>();
            if res.is_ok() {
               coin_input[i] = res.unwrap();
               funds += coins[i] * coin_input[i] as f32;
               break;
            }
            println!("Invalid Input Format! Input again!");
         }
      }
      return (false, coin_input, funds);
   }
}

pub fn calc() -> (bool, [i32;8], f32) {
   let (quite, price) = get_price();
   if quite {
      return (quite, [0;8], 0.0);
   }
   loop {
      let (quite, coin_input, funds) = input_funds();
      if quite {
         return (quite, coin_input, funds);
      }

      if funds < price {
         println!("Not Sufficient Fund! Input again!");
      } else {
         let remain = funds - price;
         return (false, coin_input, remain);
      }
   }
}
pub fn calc_balance(coin_balance: [i32;8], coin_input: [i32;8], funds: f32) -> (bool, [i32;8])
{
   let coins = get_coins();
   let max = 50;

   let mut temp_balance: [i32;8] = coin_balance;
   for i in 0..coin_balance.len() {
      temp_balance[i] += coin_input[i];

      if temp_balance[i] > max {
         println!("Exceed Max storage, {:.2} COIN", coins[i]);
         return (true, [0;8])
      }
   }

   println!("Remain funds: {:.2}", funds);
   let coin_count = calc_by_coin(funds);

   for i in 0..coin_balance.len() {
      temp_balance[i] -= coin_count[i];
      if temp_balance[i] < 0 {
         println!("Insufficient storage, {:.2} COIN", coins[i]);
         return (true, [0;8])
      }
   }

   println!("----Storage Balance-----------");
   for i in 0..coin_balance.len() {
      println!("{:.2} - {}", coins[i], temp_balance[i]);
   }
   
   println!("");
   return (false, temp_balance)
}
fn main() {
   let mut coin_balance: [i32;8] = [20;8];


   loop {
      let (quite, coin_input, remain) = calc();
      if quite {
         break;
      }

      let (skip, balance) = calc_balance(coin_balance, coin_input, remain);
      if skip {
         continue
      }
      coin_balance = balance;
   }
}
