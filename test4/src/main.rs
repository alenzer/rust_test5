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
fn main() {
   let coins = get_coins();
   let mut coin_balance: [i32;8] = [20;8];
   let max = 50;

   loop {
      let (quite, coin_input, remain) = calc();
      if quite {
         break;
      }

      let mut temp_balance: [i32;8] = coin_balance;
      let mut skip = false;
      for i in 0..coin_balance.len() {
         temp_balance[i] += coin_input[i];

         if temp_balance[i] > max {
            skip = true;
            println!("Exceed Max storage, {:.2} COIN", coins[i]);
            break;
         }
      }
      if skip {
         continue;
      }

      println!("Remain funds: {:.2}", remain);
      let coin_count = calc_by_coin(remain);

      for i in 0..coin_balance.len() {
         temp_balance[i] -= coin_count[i];
         if temp_balance[i] < 0 {
            skip = true;
            println!("Insufficient storage, {:.2} COIN", coins[i]);
            break;
         }
      }
      if skip {
         continue;
      }

      println!("----Storage Balance-----------");
      coin_balance = temp_balance;
      for i in 0..coin_balance.len() {
         println!("{:.2} - {}", coins[i], temp_balance[i]);
      }
      
      println!("");
   }
}
