![Header](https://github.com/Acria-Network/Acria-Oracle-Node-Qt/blob/main/img/New%20Project.png)

# Acria Oracle Node

This module listen to events on the blockchain and when a request for updating data matches the local configuration, it contacts the configured API and write back the data in the blockchain.  

## Requirements

Install nodejs, please refer to: [nodejs](https://nodejs.dev) documentation.  
Install yarn, please refer to [yarn](https://yarnpkg.com/) documentation and installation guides.  

## How to Build
From the folder oracle-node execute:  
```sh
yarn install  
```
It will install the required dependencies  


## How to Configure

You can change upon your specific requirement the "configuration section" inside  acria-oracle-node.js:  
```js
// customization section - you can change the following constants upon your preferences
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
const CONFILE="./acria-oracle-node.conf";           //configuration file
const KEYFILE="./acria-oracle-node.key";            // where to store the keys
const LOGFILE="./acria-oracle-node.log";            //log file name
// end customization section
```
You should add the Oracle configuration to the blockchain by the extrinsic "acria.newOracle" from the user interface.  
You should configure the file acria-oracle-node.conf to match your endpoints to send back the data.  

## How to Run
Once the Acria Substrate Node is started, launch:  
```sh
node acria-oracle-node.js  
```
At the first run, a secret seed will be generated automatically for the "well known" account Alice used for testing in the user interface.  
An event log will be shown on the screen.  





