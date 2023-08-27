# Fork of pybit python libary in Rust,

> For the rustcean lovers and creators for bybit exchange version 5

Official Python3 API connector for Bybit's HTTP and WebSockets APIs.

## Table of Contents

- [About](#about)
- [Development](#development)
- [Installation](#installation)
- [Usage](#usage)
- [Contact](#contact)
- [Contributors](#contributors)
- [Donations](#donations)

## About

Put simply, `bybit-rs` (Bybit + Rust) is the fork of the python (pybit library) official lightweight one-stop-shop module for the Bybit HTTP and WebSocket APIs. Originally created by [Verata Veritatis](https://github.com/verata-veritatis), it's now maintained by [Omambia Dauglous](https://github.com/domambia) & [Dennis Mwangi](https://github.com/Dennis-Codes) – however, you're still welcome to contribute!

It was designed with the following vision in mind:

> I was personally never a fan of auto-generated connectors that used a mosh-pit of various modules you didn't want (sorry, `bravado`) and wanted to build my own Python3-dedicated connector with very little external resources. The goal of the connector is to provide traders and developers with an easy-to-use high-performing module that has an active issue and discussion board leading to consistent improvements.

## Development

`bybit-rs` is being actively developed, and new Bybit API changes should arrive on `bybit-rs` very quickly. `pybit` uses `reqwest` and `websocket` for its methods, alongside other built-in modules. Anyone is welcome to branch/fork the repository and add their own upgrades. If you think you've made substantial improvements to the module, submit a pull request and we'll gladly take a look.

## Installation

`bybit` requires Python 3.9.1 or higher. The module can be installed manually or via [PyPI](https://pypi.org/project/pybit/) with `pip`:

```
cargo add bybit-rs
```

## Usage

You can retrieve a specific market like so:

```rust
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use bybit::{
    asset::{self, Asset},
    http_manager::{HttpManager, Manager},
    market::{self, Market},
    trade::{self, Trade},
};
```

Create an HTTP session and connect via WebSocket for Inverse on mainnet:

```rust
let http_api_key =""
let http_api_secret =""

let testnet = ""

let manager = Arc::new(HttpManager::new(http_api_key, http_api_secret, testnet));
```

### TO MARKET GET KLINE DAT

```rust
let mut query: HashMap<String, String> = HashMap::new();
query.insert("category".to_string(), "inverse".to_string());
query.insert("symbol".to_string(), "BTCUSD".to_string());
query.insert("interval".to_string(), "60".to_string());
query.insert("start".to_string(), "1670601600000".to_string());
query.insert("end".to_string(), "1670608800000".to_string());

// market object
let market: market::MarketHTTP = market::MarketHTTP::new(manager.clone());

match market.get_kline(query).await {
    Ok(result) => println!("{:?}", result),
    Err(e) => println!("{:?}", e),
}

```

### TO PLACE AN ORDER

```rust
let mut query: HashMap<String, String> = HashMap::new();
query.insert("category".to_owned(), "linear".to_owned());
query.insert("symbol".to_owned(), "BTCUSDT".to_owned());
query.insert("orderType".to_owned(), "Limit".to_owned());
query.insert("qty".to_owned(), "0.06".to_owned());
query.insert("price".to_owned(), "25000".to_owned());
query.insert("side".to_owned(), "Buy".to_owned());

let trade: trade::TradeHTTP = trade::TradeHTTP::new(manager.clone());

match trade.place_order(query).await {
    Ok(result) => println!("{:?}", result),
    Err(e) => println!("{:?}", e),
}
```

### TO AN ORDER

```rust
let mut query: HashMap<String, String> = HashMap::new();
query.insert("category".to_owned(), "linear".to_owned());
query.insert("limit".to_owned(), "1".to_owned());
query.insert("symbol".to_owned(), "BTCUSDT".to_owned());
query.insert("openOnly".to_owned(), "0".to_owned());

match trade.get_open_orders(query).await {
    Ok(result) => println!("{:?}", result),
    Err(e) => println!("{:?}", e),
}
```

### TO CANCEL AN ORDER

````rust
let mut query: HashMap<String, String> = HashMap::new();
query.insert("category".to_owned(), "linear".to_owned());
query.insert(
    "orderId".to_owned(),
    "3380b972-a334-4d00-87e9-3423fa27602f".to_owned(),
);
query.insert("symbol".to_owned(), "BTCUSDT".to_owned());
query.insert("settleCoin".to_owned(), "USDT".to_owned());

match trade.cancel_order(query).await {
    Ok(result) => println!("{:?}", result),
    Err(e) => println!("{:?}", e),
}

### TO CANCEL ALL ORDERS

```rust
let mut query: HashMap<String, String> = HashMap::new();
query.insert("category".to_owned(), "linear".to_owned());
query.insert("symbol".to_owned(), "".to_owned());
query.insert("settleCoin".to_owned(), "USDT".to_owned());

match trade.cancel_all_orders(query).await {
    Ok(result) => println!("{:?}", result),
    Err(e) => println!("{:?}", e),
}
````

Check out the example rust files or the list of endpoints below for more information on available
endpoints and methods. Usage examples on the `libary Manager` methods can
be found in the [examples folder](https://github.com/domambia/bybit_rs/examples_folder).

## Contact

You can reach out for support on the [Ngeni Labs Support Telegram](https://t.me/ng.NgeniLabs Dev Support) group chat.

## Credits

Thanks goes to these wonderful contritors of pybit libary [official commutity library](https://github.com/bybit-exchange/pybit) & [NGENI LABs](https://ngeni.io) [github](https://github.com/devngeni) team for your technical support

## Donations

If you find this libary useful, donate to

> USDT

    1. TRC20 TVGdWAZ3MetiRyUhkR4CjR7YFi99TQvZ2L
    2. ERC20: 0x43bFd041eB6F6ccdC247B4162EB7D056B4bF97BA

> BTC: bc1q8mcktjuwh9ufy7tcz4ep7u5uhef3z2m8qdnurs
> ETH:0x43bFd041eB6F6ccdC247B4162EB7D056B4bF97BA
