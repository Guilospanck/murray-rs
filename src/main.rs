use murray_rs::prices::{self, types::{ConvertCurrencyParams, Currency, GetTickerParams, Symbol}};



fn main() {
  /** Prices */
  prices::set_base_url("http://localhost:3000".to_string());
  let a = prices::convert_currency(ConvertCurrencyParams { currency: Currency::BRL, value: 100 });
  println!("{:?}", a.unwrap());
  let b = prices::get_ticker(GetTickerParams{symbol: Symbol::BTCUSD});
  println!("{:?}", b.unwrap());
  let c = prices::get_tickers(GetTickerParams{symbol: Symbol::BTCUSD});
  println!("{:?}", c.unwrap());
}
