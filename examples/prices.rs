use murray_rs::{ConvertCurrencyParams, Currency, GetTickerParams, Murray, Symbol};

fn main() {
  let murray = Murray::default();
  
  let a = murray.prices.convert_currency(ConvertCurrencyParams { currency: Currency::BRL, value: 100 });
  println!("{:?}", a.unwrap());

  let b = murray.prices.get_ticker(GetTickerParams{symbol: Symbol::BTCUSD});
  println!("{:?}", b.unwrap());

  let c = murray.prices.get_tickers(GetTickerParams{symbol: Symbol::BTCUSD});
  println!("{:?}", c.unwrap());

  let d = murray.prices.get_health();
  println!("{:?}", d.unwrap());
}