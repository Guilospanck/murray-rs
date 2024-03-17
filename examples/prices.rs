use murray_rs::{ConvertCurrencyParams, Currency, GetTickerParams, Murray, PriceError, Symbol};

fn price_example() -> std::result::Result<(), PriceError> {
  let murray = Murray::default();

  let a = murray.prices.convert_currency(ConvertCurrencyParams {
    currency: Currency::BRL,
    value: 100,
  })?;
  println!("{:?}", a);

  let b = murray.prices.get_ticker(GetTickerParams {
    symbol: Symbol::BTCUSD,
  })?;
  println!("{:?}", b);

  let c = murray.prices.get_tickers(GetTickerParams {
    symbol: Symbol::BTCUSD,
  })?;
  println!("{:?}", c);

  let d = murray.prices.get_health()?;
  println!("{:?}", d);

  Ok(())
}

fn main() {
  let _ = price_example();
}
