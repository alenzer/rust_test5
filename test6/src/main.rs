use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

use test4::main::{calc_balance, input_funds};
use test5::main::calc;

fn main() {
   let mut coin_balance: [i32; 8] = [20; 8];

   let f_handle = File::open("coin_balance.txt");
   if f_handle.is_ok() {
      let file = BufReader::new(f_handle.unwrap());
      let lines: Vec<_> = file.lines().map(|line| line.unwrap()).collect();
      for i in 0..coin_balance.len() {
         coin_balance[i] = lines[i].parse::<i32>().expect("Invalid Number Format");
      }
   }

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
   let f_handle = File::create("coin_balance.txt");
   if f_handle.is_ok() {
      let mut file = f_handle.unwrap();
      for i in 0..coin_balance.len() {
         file.write_all(coin_balance[i].to_string().as_bytes()).unwrap();
         file.write_all("\n".as_bytes()).unwrap();
      }
   }
}
