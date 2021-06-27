# open-runtime-module-library

[![Crates.io](https://img.shields.io/crates/v/orml-tokens)](https://crates.io/search?q=orml)
[![GitHub](https://img.shields.io/github/license/open-web3-stack/open-runtime-module-library)](https://github.com/open-web3-stack/open-runtime-module-library/blob/master/LICENSE)

The Open Runtime Module Library (ORML) is a community maintained collection of Substrate runtime modules.

## Runtime Modules Overview

- [orml-traits](./traits)
    - Shared traits including `BasicCurrency`, `MultiCurrency`, `Auction` and more.
- [orml-utilities](./utilities)
	- Various utilities including `OrderSet`.
- [orml-tokens](./tokens)
    - Fungible tokens module that implements `MultiCurrency` trait.
- [orml-currencies](./currencies)
	- Provide `MultiCurrency` implementation using `pallet-balances` and `orml-tokens` module.
- [orml-oracle](./oracle)
    - Oracle module that makes off-chain data available on-chain.
- [orml-auction](./auction)
	- Auction module that implements `Auction` trait.
- [orml-vesting](./vesting)
    - Provides scheduled balance locking mechanism, in a *graded vesting* way.
- [orml-gradually-update](./gradually-update)
    - Provides way to adjust numeric parameter gradually over a period of time.

## Example

Checkout [orml-workshop](https://github.com/xlc/orml-workshop) for example usage.

## Development

### Makefile targets

- `make check`
	- Type check the code, without std feature, excluding tests.
- `make check-tests`
	- Type check the code, with std feature, including tests.
- `make test`
	- Run tests.

### `Cargo.toml`

ORML use `Cargo.dev.toml` to avoid workspace conflicts with project cargo config. To use cargo commands in ORML workspace, create `Cargo.toml` by running

- `cp Cargo.dev.toml Cargo.toml`, or
- `make Cargo.toml`, or
- change the command to `make dev-check` etc which does the copy. (For the full list of `make` commands, check `Makefile`)

# Web3 Foundation Grant Project
ORML is part of the bigger `Open-Web3-Stack` initiative, that is currently under a General Grant from Web3 Foundation. See Application details [here](https://github.com/open-web3-stack/General-Grants-Program/blob/master/grants/speculative/open_web3_stack.md). The 1st milestone has been delivered. 

# Projects using ORML
- [If you intend or are using ORML, please_add your project here_](https://github.com/open-web3-stack/open-runtime-module-library/edit/master/README.md)

_In alphabetical order_

- [Acala Network](https://github.com/AcalaNetwork/Acala)
- [Bit.Country](https://github.com/bit-country/Bit-Country-Blockchain)
- [ChainX](https://github.com/chainx-org/ChainX)
- [HydraDX](https://github.com/galacticcouncil/hack.HydraDX-node)
- [Laminar Chain](https://github.com/laminar-protocol/laminar-chain)
- [Setheum Network](https://github.com/Setheum-Labs/Setheum)
