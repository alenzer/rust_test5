use test1::main::calc;

pub fn calc_by_coin(funds: f32) {
   let coins: [f32; 8] = [2.00, 1.00, 0.50, 0.20, 0.10, 0.05, 0.02, 0.01];
   let mut coin_num: [i32; 8] = [0; 8];

   let mut i_funds: i32 = (funds * 100.0).round() as i32;
   for i in 0..coins.len() {
      let i_coin = (coins[i] * 100.0).floor() as i32;

      coin_num[i] = i_funds / i_coin;
      i_funds = i_funds % i_coin;
   }
   for i in 0..coin_num.len() {
      if coin_num[i] != 0 {
         println!("{:.2}:{}", coins[i], coin_num[i]);
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
      calc_by_coin(remain);
      println!("");
   }
}
