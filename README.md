![Header](https://github.com/Acria-Network/Acria-Oracle-Node-Qt/blob/main/img/New%20Project.png)

# Acria Substrate

Acria Substrate Blockchain implemented in Substrate/Rust

![GitHub](https://img.shields.io/github/license/Acria-Network/acria-substrate)
![GitHub last commit](https://img.shields.io/github/last-commit/Acria-Network/acria-substrate)
![GitHub](https://img.shields.io/badge/Substrate-2.0.1-brightgreen)
![GitHub](https://img.shields.io/badge/OS-Linux%2FMacOS%2FWindows-brightgreen)
![GitHub](https://badgen.net/twitter/follow/acrianetwork)

One of the most pressing issues when developing smart contracts is the lack of real-world data. But due to technical limitations, such as the consensus protocol, no blockchain has been able to solve this major limitation. The Acria Network solves exactly this problem with the help of so-called Oracle Nodes that don't require a middleman. In addition to this, it offers cross-chain support to supply various blockchains with real-world data.

[https://acria.network/](https://acria.network/)

# Acria Node

Welcome to the Acria Substrate Node documentation. 

One of the most pressing issues when developing smart contracts is the lack of real-world data. But due to technical limitations, such as the consensus protocol, no blockchain has been able to solve this major limitation. The Acria Network solves exactly this problem with the help of so-called Oracle Nodes that don't require a middleman. In addition to this, it offers cross-chain support to supply various blockchains with real-world data.  
(https://acria.network)[https://acria.network]

Please have a look at the [doc](./doc) directory for more information regarding
* [How to do the Rust setup](./doc/rust-setup.md)
* [How to create accounts](./doc/create-accounts.md)
* [How to do balance transfers](./doc/balance-transfers.md)
* [More details about running a node](./doc/run-node.md)

## Run the chain as standalone node

Use Rust's native `cargo` command to build and launch the Acria node:

```sh
cargo run --release -- --dev --tmp
```
### Build the chain

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### User Interface

For debugging and testing the node, you can use this (web user interface)[https://ipfs.io/ipns/dotapps.io/].  
You should select the connection to your node on top left, for example if your node is installed in the same machine you are connecting from,
it will be `ws://127.0.0.1`   or `ws://localhost`.


### Api Interface

The node offers the following application programming interfaces, accessible from the user interface above:

 - acria.newOracle(oracleid,oracledata), a function to create a new ORACLE, the oracleid is an integer (u32) not already used and in the oracledata is a json structure with the following fields:  
    - shortdescription - a short description not longer than 64 bytes  
	- description  - a long description not longer than 6144 bytes  
    - apiurl  - an https address as reference for the API, explaining the possible parameters if any.  
    - fees - amount of fees applied to the requester.  
    example: {"shortdescription":"xxxxxxxxxxxxxxxxxx","description":"xxxxxxxxxxxxxxxxxxxxxxxxx","apiurl":"https://api.supplier.com/documentation","fees":0.0000001}  
 
 - acria.removeOracle(oracleid), a function to remove an ORACLE, only the original creator can remove it.  
 
 - acria.requestOracleUpdate(oracleaccount,oracleid), is the function used to request a data update to the Acria Oracle Node.  
 
 - acria.oracleUpdate(oracleid,oracledata), is the internal function used from the Oracle, to update the data on the blockchain.  

 - acria.oracle(AccountId,Oracleid), allows to query the data written from the Oracle matching the AccountId and Oracleid. From the user interface you should select "Chain State","Acria", "Oracle".


 For testing you should:  
 1) start the Blockchain node,  
 2) open the user interface (web user interface)[https://ipfs.io/ipns/dotapps.io/],  
 3) click on "Developer", "Extrinsics", "acria" and "newOracle",  
 4) select "Alice" account that will be the owner of the Oracle  
 5) insert "1" in the "oracleid" field  
 6) insert: 
 ```
 {"shortdescription":"Coingecko - Price BTC/USD","description":"Coingecko collect in real time the transaction from >20 exchanges and calculcate the average price every 60 seconds.","apiurl":"https://www.coingecko.com/","fees":0.1}  
```
in the field "oracledata",  
7) Click on "Submit Transaction" and check for the events shown.  
You should have stored a public Oracle in your blockchain.

8) now start the (Acria Oracle Node)[./oracle-node/README.md] in another screen  
9) from the web user interface select "acria" and "requestOracleUpdate",  
10) insert "1" in the field "oracleid" and "na" in the field "parameters" that could used to send some data to the API, in this case "na" = not applicable.  
11) click on "Submit Transaction",  
12) check the log of the AON and you should see that it has received the update request from the events generated and it has update the blockchain with its own data.  


		


