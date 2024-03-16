#![allow(unused_doc_comments)]
use murray_rs::blockchain::{self, types::{GetAddressParams, GetBlockParams}};

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
  // blockchain::set_base_url("http://localhost:3000".to_string());
  // let a = blockchain::get_block(GetBlockParams {
  //   hash: None,
  //   height: None,
  // });
  // println!("{:?}\n", a.unwrap());

  // let b = blockchain::get_block2time(GetBlockParams {
  //   hash: None,
  //   height: Some(500000),
  // });
  // println!("{:?}\n", b.unwrap());

  // let c = blockchain::get_fees_recommended();
  // println!("{:?}\n", c.unwrap());

  // let d = blockchain::get_fees_mempool_blocks();
  // println!("{:?}\n", d.unwrap());

  // let e = blockchain::get_address_details(GetAddressParams {address: "1F1tAaz5x1HUXrCNLbtMDqcw6o5GNn4xqX".to_string()});
  // println!("{:?}\n", e.unwrap());

  // let f = blockchain::get_address_transactions(GetAddressParams {address: "3Brz916o2Ng2s6iYT9bgCpAxTRqd9b3GGW".to_string()});
  // println!("{:?}\n", f.unwrap());

  // let g = blockchain::get_address_utxos(GetAddressParams {address: "3Brz916o2Ng2s6iYT9bgCpAxTRqd9b3GGW".to_string()});
  // println!("{:?}\n", g.unwrap());

  // let h = blockchain::get_hashrate();
  // println!("{:?}\n", h.unwrap());

  let i = blockchain::get_health();
  println!("{:?}\n", i.unwrap());
}
