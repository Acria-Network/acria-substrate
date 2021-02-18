
# How to setup an account?

There a various possibilities to setup an account. Please refer to this [official Wiki](https://wiki.polkadot.network/docs/en/learn-account-generation).

## Development

For development purposes, it suffices to use the [Polkadot Js Apps](https://wiki.polkadot.network/docs/en/learn-account-generation#polkadot-js-apps). 

Please note that the chain spec which is automatically used when running your chain in single node development mode with `--dev`, the default Accouns Alice, Bob, Charlie, Eve and Ferdie and their corresponding stash accounts already have enough balance to play around with.

## Chain Spec

Once you create your own chain spec and change the endowed accounts, we recommend to add these accounts to the [Polkadot Js Browser Plugin](https://wiki.polkadot.network/docs/en/learn-account-generation#polkadotjs-browser-plugin) for your staging-net or testnet.

Please refer to this [official tutorial](https://substrate.dev/docs/en/tutorials/start-a-private-network/customspec) on how to create your own chain spec.