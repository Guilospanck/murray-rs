use murray_rs::prices::{self, types::{ConvertCurrencyParams, Currency}};

fn main(){
  let a = prices::convert_currency(ConvertCurrencyParams { currency: Currency::BRL, value: 100 });
  println!("{:?}", a.unwrap());
}