#![allow(unused_doc_comments)]
use murray_rs::{
  blockchain::{self, types::GetBlockParams},
  // prices::{
  //   self,
  //   types::{ConvertCurrencyParams, Currency, GetTickerParams, Symbol},
  // },
};

fn main() {
  /** Prices
   * 
    prices::set_base_url("http://localhost:3000".to_string());
    let a = prices::convert_currency(ConvertCurrencyParams { currency: Currency::BRL, value: 100 });
    println!("{:?}", a.unwrap());
    let b = prices::get_ticker(GetTickerParams{symbol: Symbol::BTCUSD});
    println!("{:?}", b.unwrap());
    let c = prices::get_tickers(GetTickerParams{symbol: Symbol::BTCUSD});
    println!("{:?}", c.unwrap());
  */

  /** Blockchain */
  blockchain::set_base_url("http://localhost:3000".to_string());
  // let a = blockchain::get_block(GetBlockParams {
  //   hash: None,
  //   height: None,
  // });
  // println!("{:?}", a.unwrap());

  // let b = blockchain::get_block2time(GetBlockParams {
  //   hash: None,
  //   height: Some(500000),
  // });
  // println!("{:?}", b.unwrap());

  let c = blockchain::get_fees_recommended();
  println!("{:?}", c.unwrap());
}
