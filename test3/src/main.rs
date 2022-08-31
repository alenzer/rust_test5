use std::io;
use test1::main::input_funds;
use test2::main::calc_by_coin;

struct Product {
   title: String,
   value: f32,
}

pub fn get_price() -> (bool, f32) {
   let products: Vec<Product> = vec![
      Product {
         title: "1".to_string(),
         value: 1.0,
      },
      Product {
         title: "2".to_string(),
         value: 2.1,
      },
      Product {
         title: "3".to_string(),
         value: 3.3,
      },
      Product {
         title: "4".to_string(),
         value: 4.4,
      },
      Product {
         title: "5".to_string(),
         value: 5.5,
      },
      Product {
         title: "6".to_string(),
         value: 6.6,
      },
      Product {
         title: "7".to_string(),
         value: 7.7,
      },
      Product {
         title: "8".to_string(),
         value: 8.8,
      },
      Product {
         title: "9".to_string(),
         value: 9.8,
      },
      Product {
         title: "10".to_string(),
         value: 10.10,
      },
   ];

   loop {
      println!("-------Products-------");
      for i in 0..products.len() {
         println!("{} : {:.2}", products[i].title, products[i].value);
      }

      let mut input = String::new();
      println!("Select the Product(1..10)");

      io::stdin()
         .read_line(&mut input)
         .expect("Not a valid string");

      if input.trim() == "q" {
         return (true, 0.0)
      }

      let res = input.trim().parse::<i32>();
      if res.is_ok() {
         let index = (res.unwrap() - 1) as usize;
         if index < products.len() {
            println!("Price: {}", products[index].value);
            return (false, products[index].value)
         }
      }
   }
}

pub fn calc() -> (bool, f32) {
   let (quite, price) = get_price();
   if quite {
      return (quite, price)
   }
   
   loop {
      let (quite, funds) = input_funds();
      if quite {
         return (quite, funds);
      }

      if funds < price {
         println!("Not Sufficient Fund! Input again!");
      } else {
         let remain = funds - price;
         return (false, remain);
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
