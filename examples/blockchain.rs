use murray_rs::{
  GetAddressParams, GetBlockParams, GetTransactionParams, Murray, PostTransactionParams,
};

fn main() {
  let murray = Murray::default();

  let a = murray.blockchain.get_block(GetBlockParams {
    hash: None,
    height: Some(500000),
  });
  println!("{:?}\n", a.unwrap());

  let b = murray.blockchain.get_block2time(GetBlockParams {
    hash: None,
    height: Some(500000),
  });
  println!("{:?}\n", b.unwrap());

  let c = murray.blockchain.get_fees_recommended();
  println!("{:?}\n", c.unwrap());

  let d = murray.blockchain.get_fees_mempool_blocks();
  println!("{:?}\n", d.unwrap());

  let e = murray.blockchain.get_address_details(GetAddressParams {
    address: "1F1tAaz5x1HUXrCNLbtMDqcw6o5GNn4xqX".to_string(),
  });
  println!("{:?}\n", e.unwrap());

  let f = murray
    .blockchain
    .get_address_transactions(GetAddressParams {
      address: "3Brz916o2Ng2s6iYT9bgCpAxTRqd9b3GGW".to_string(),
    });
  println!("{:?}\n", f.unwrap());

  let g = murray.blockchain.get_address_utxos(GetAddressParams {
    address: "3Brz916o2Ng2s6iYT9bgCpAxTRqd9b3GGW".to_string(),
  });
  println!("{:?}\n", g.unwrap());

  let h = murray.blockchain.get_hashrate();
  println!("{:?}\n", h.unwrap());

  let i = murray.blockchain.get_health();
  println!("{:?}\n", i.unwrap());

  let j = murray.blockchain.get_mempool();
  println!("{:?}\n", j.unwrap());

  let k = murray.blockchain.get_transaction(GetTransactionParams {
    txid: "15e10745f15593a899cef391191bdd3d7c12412cc4696b7bcb669d0feadc8521".to_string(),
  });
  println!("{:?}\n", k.unwrap());

  let tx_hex = "0200000001fd5b5fcd1cb066c27cfc9fda5428b9be850b81ac440ea51f1ddba2f987189ac1010000008a4730440220686a40e9d2dbffeab4ca1ff66341d06a17806767f12a1fc4f55740a7af24c6b5022049dd3c9a85ac6c51fecd5f4baff7782a518781bbdd94453c8383755e24ba755c01410436d554adf4a3eb03a317c77aa4020a7bba62999df633bba0ea8f83f48b9e01b0861d3b3c796840f982ee6b14c3c4b7ad04fcfcc3774f81bff9aaf52a15751fedfdffffff02416c00000000000017a914bc791b2afdfe1e1b5650864a9297b20d74c61f4787d71d0000000000001976a9140a59837ccd4df25adc31cdad39be6a8d97557ed688ac00000000".to_owned();
  let l = murray
    .blockchain
    .post_transaction(PostTransactionParams { tx_hex });
  println!("{:?}\n", l.unwrap());
}
