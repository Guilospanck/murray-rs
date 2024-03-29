
# Murray-rs [![Tests](https://github.com/Guilospanck/murray-rs/actions/workflows/tests.yml/badge.svg?branch=main&event=push)](https://github.com/Guilospanck/murray-rs/actions/workflows/tests.yml) [![codecov](https://codecov.io/gh/Guilospanck/murray-rs/graph/badge.svg?token=E87lAHhkfC)](https://codecov.io/gh/Guilospanck/murray-rs) [![crates.io](https://img.shields.io/crates/v/murray-rs.svg)](https://crates.io/crates/murray-rs) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)

Murray-rs is the rust version of the comprehensive NPM package module designed for seamless integration with Murray Rothbot's suite of APIs, including Blockchain, Lightning, and Prices - [Murray JS](https://github.com/murray-rothbot/murray-js). This powerful library enables developers to quickly incorporate bitcoin data and interactions within their applications.

## Installation

To get started, install the package using cargo:

```bash
cargo add murray-rs
```

## Usage

After installation, import `murray-rs` into your project to access the various APIs.

### Blockchain API

```rs
use murray_rs::{GetBlockParams, Murray, BlockchainError};

fn main() -> std::result::Result<(), BlockchainError> {
  let murray = Murray::default();

  let a = murray.blockchain.get_block(GetBlockParams {
    hash: None,
    height: Some(500000),
  })?;
  println!("{:?}", a);

  Ok(())
}
```

### Lightning API

```rs
use murray_rs::{Murray, GetNodeDetailsParams, LightningError};

fn main() -> std::result::Result<(), LightningError> {
  let murray = Murray::default();

  let a = murray.lightning.get_node_details(GetNodeDetailsParams {
    public_key: "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f".to_string(),
  })?;
  println!("{:?}", a);

  Ok(())
}
```

### Prices API

```rs
use murray_rs::{Murray, ConvertCurrencyParams, Currency, PriceError};

fn main() -> std::result::Result<(), PriceError> {
  let murray = Murray::default();

  let a = murray.prices.convert_currency(ConvertCurrencyParams {
    currency: Currency::BRL,
    value: 100,
  })?;
  println!("{:?}", a);

  Ok(())
}
```

## Custom Endpoints

Customize endpoints for each API to suit your specific needs:

```rs
use murray_rs::{Murray, BaseEndpointsParams};

fn main() {
  let murray = Murray::new(BaseEndpointsParams {
    blockchain_endpoint: Some("https://your-custom-domain.com/".to_string()),
    prices_endpoint: Some("https://your-custom-domain.com/".to_string()),
    lightning_endpoint: Some("https://your-custom-domain.com/".to_string()),
  });

  let blockchain = murray.blockchain.get_health();
  println!("{:?}", blockchain.unwrap());
  let prices = murray.prices.get_health();
  println!("{:?}", prices.unwrap());
  let lightning = murray.lightning.get_health();
  println!("{:?}", lightning.unwrap());
}
```

## Examples

Find more examples in the repository to guide your implementation:

- [Blockchain](./examples/blockchain.rs)
- [Lightning](./examples/lightning.rs)
- [Prices](./examples/prices.rs)

## Self Hosted APIs

Leverage your self-hosted APIs for enhanced control and customization:

- [Service Blockchain](https://github.com/Murray-Rothbot/service-blockchain)
- [Service Lightning](https://github.com/Murray-Rothbot/service-lightning)
- [Service Prices](https://github.com/Murray-Rothbot/service-prices)

## Contributing

Contributions are welcome! For significant changes or enhancements, please open an issue first to discuss your ideas.

## License

Murray-rs is open-sourced software licensed under the [MIT license](./LICENSE).
