use test1::main::calc;

const COINS: [f32; 8] = [2.00, 1.00, 0.50, 0.20, 0.10, 0.05, 0.02, 0.01];
pub fn get_coins() -> [f32; 8]{
   COINS
}
pub fn calc_by_coin(funds: f32) -> [i32;8]{
   let mut coin_count: [i32; 8] = [0; 8];

   let mut i_funds: i32 = (funds * 100.0).round() as i32;
   for i in 0..COINS.len() {
      let i_coin = (COINS[i] * 100.0).floor() as i32;

      coin_count[i] = i_funds / i_coin;
      i_funds = i_funds % i_coin;
   }
   println!("Out Coins");
   for i in 0..coin_count.len() {
      if coin_count[i] != 0 {
         println!("{:.2} : {}", COINS[i], coin_count[i]);
      }
   }
   println!("");
   coin_count
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
