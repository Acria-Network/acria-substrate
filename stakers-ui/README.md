![Header](https://github.com/Acria-Network/Acria-Oracle-Node-Qt/blob/main/img/New%20Project.png)

# Acria - Stakers - User Interface

This is a web app to allow stakers staking and un-staking funds to an Oracle.

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

You can change upon your specific requirement the "configuration section" inside  stakers.js:  
```js
// customization section - you can change the following constants upon your preferences
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
// end customization section 
```

## How to Run
Once the Acria Substrate Node is started, launch:  
```sh
node stakers.js  
```

and open your browser to:  
```
http://yourhostname:3000
```
if your are not able to connect please check the firewall configuration.  







