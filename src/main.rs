use murray_rs::prices::{self, types::{ConvertCurrencyParams, Currency, GetTickerParams, Symbol}};



fn main() {
  let a = prices::convert_currency(ConvertCurrencyParams { currency: Currency::BRL, value: 100 });
  println!("{:?}", a.unwrap());
  let b = prices::get_ticker(GetTickerParams{symbol: Symbol::BTCUSD});
  println!("{:?}", b.unwrap());
  let c = prices::get_tickers(GetTickerParams{symbol: Symbol::BTCUSD});
  println!("{:?}", c.unwrap());


  // prices::set_base_url("http://blockchain.murrayrothbot.com".to_string());
  // print!("{:?}", prices::convert_currency(ConvertCurrencyParams { currency: Currency::BRL, value: 100 }).unwrap());
}
