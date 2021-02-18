# How to do transfers?

You can either use the [Polkadot Js Apps](https://wiki.polkadot.network/docs/en/learn-balance-transfers) or call the exposed RPC endpoint
```js
// Import
import { ApiPromise, WsProvider } from '@polkadot/api';

// Construct
const defaultDevWs = 'ws://127.0.0.1:9944';
const wsProvider = new WsProvider(defaultDevWs);
const api = await ApiPromise.create({ provider: wsProvider });

// Sign and send a transfer from Alice to Bob
const txHash = await api.tx.balances
  .transfer(BOB, 12345)
  .signAndSend(alice);

// Show the hash
console.log(`Submitted with hash ${txHash}`);
```

Please refer to the Polkadot Js documentation on how to
* [Simple transactions](https://polkadot.js.org/docs/api/start/api.tx)
* [Batch transactions](https://polkadot.js.org/docs/api/cookbook/tx#how-can-i-batch-transactions)
* [Listen to balance changes](https://polkadot.js.org/docs/api/examples/promise/listen-to-balance-change)
* [Send an unsigned extrinsic](https://polkadot.js.org/docs/api/cookbook/tx#how-do-i-send-an-unsigned-extrinsic)
* [Get the decoded enum for a failed extrinsic](https://polkadot.js.org/docs/api/cookbook/tx#how-do-i-get-the-decoded-enum-for-an-extrinsicfailed-event)
* [Estimate transaction fees](https://polkadot.js.org/docs/api/cookbook/tx#how-do-i-estimate-the-transaction-fees) 

## How to enable/disable transfers

Transfers are enabled when adding the `pallet_balances` as a module to your [runtime](../runtime/src/lib.rs#L268) via the `construct_runtime!` macro:

```rust
// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
		Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
		Aura: pallet_aura::{Module, Config<T>},
		Grandpa: pallet_grandpa::{Module, Call, Storage, Config, Event},
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		TransactionPayment: pallet_transaction_payment::{Module, Storage},
		Sudo: pallet_sudo::{Module, Call, Config<T>, Storage, Event<T>},
		// Include the custom logic from the template pallet in the runtime.
		TemplateModule: pallet_template::{Module, Call, Storage, Event<T>},
	}
);
```

Please note that you can keep the module in your runtime but disable tranfers by removing the `Call` trait:
```rust
		Balances: pallet_balances::{Module, Storage, Config<T>, Event<T>},
```