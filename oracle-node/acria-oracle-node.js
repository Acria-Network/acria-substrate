// Acria Oracle Node 
// AON listens to events propagated on the blockchain, when a data update request match its own configuration,
// it make its work writing the blockchain with the new data requested.
const { ApiPromise, WsProvider } = require('@polkadot/api');    
const { Keyring } = require('@polkadot/api');
const {mnemonicGenerate,naclDecrypt,naclEncrypt} = require('@polkadot/util-crypto');
const { stringToU8a,u8aToString} =require('@polkadot/util');
const fetch = require('node-fetch');
const fs = require('fs');
const Base64 =require('crypto-js/enc-base64');
const Utf8 = require('crypto-js/enc-utf8');
const crypto = require('crypto');
let readlineSync = require('readline-sync'); // library to read keyboard input from command line
const { Console } = require('console');
const process = require('process');

// customization section - you can change the following constants upon your preferences
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
const CONFILE="./acria-oracle-node.conf";           //configuration file
const KEYFILE="./acria-oracle-node.key";            // where to store the keys
const LOGFILE="./acria-oracle-node.log";            //log file name
// end customization section


console.log("[Info] Acria Oracle Node v. 1.00 - Starting");
// check for parameters from command line, default is development mode for production: --production
let devmode = true;
if(process.argv.length>1){
   if(process.argv[2]=="--production"){
       devmode=false;
   }
   console.log(process.argv);
}
// we launch the main loop in async
mainloop(devmode); // jum to async function to use await

// main loop
async function mainloop(devmode){

    const api = await ApiPromise.create({ provider: wsProvider });      // create API object
    let secretseed="";
    // generate local keys if not yet done
    if(!fs.existsSync(KEYFILE) ) {
        let password = readlineSync.question('Please insert a new password: ');
        let password2 = readlineSync.question('Please insert the same password: ');
        if( password != password2){
            console.log("[Error] Passwords are not matching,closing the program.");
            process.exit(1) 
        }
        if(password.length<8){
            console.log("[Error] Passwords must be minimum 8 chars");
            process.exit(1) 
        }
        console.log("[Info] Generating Key Pairs");
        //**  FOR PRODUCTION
        let randomseed="";
        console.log("devmode:",devmode)
        if (devmode===false){
            randomseed = mnemonicGenerate(24);
        }
        else {
            //**  FOR DEVELOPMENT we use Alice account
            randomseed = 'bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice';
        }
        secretseed=randomseed;
        // password derivation -> 256 bit hash
        const hashSECRET = crypto.createHash('sha256');
        hashSECRET.on('readable', () => {
            const data = hashSECRET.read();
        });
        hashSECRET.write(password);
        hashSECRET.end();
        let SECRETSHA256=hashSECRET.digest();
        // encrypt the randomseed
        const { encrypted, nonce } = naclEncrypt(stringToU8a(randomseed), SECRETSHA256);
        let de = Buffer.from(encrypted);
        let d64 = de.toString('base64');
        let n = Buffer.from(nonce);
        let n64 = n.toString('base64');
        let randomseedencb64=n64+"###"+d64;
        console.log("****************** ATTENTION ***********************");
        console.log(`[Info] Generated mnemonic seed: ${randomseed}`);
        console.log("Please store in a safe place.");
        console.log("****************** ATTENTION ***********************");
        fs.appendFileSync(KEYFILE, randomseedencb64, 'utf8');
    }else {
        // request password
        let password = readlineSync.question('Please insert your password: ');
        if(password.length==0){
            console.log("[Error] Passwords cannot be empty.");
            process.exit(1) 
        }
        //password derivation to 256 bits
        const hashSECRET = crypto.createHash('sha256');
        hashSECRET.on('readable', () => {
            const data = hashSECRET.read();
        });
        hashSECRET.write(password);
        hashSECRET.end();
        let SECRETSHA256=hashSECRET.digest();
        // Load secret seed from file
        let secretseedencb64= fs.readFileSync(KEYFILE, 'utf8');
        let b = secretseedencb64.split('###');
        let nonceb64=b[0];
        let encseedb64=b[1];
        // decrypt secret seed
        const nonce = Buffer.from(nonceb64,'base64');
        const encseed = Buffer.from(encseedb64,'base64');
        let secretseedu8 = naclDecrypt(encseed, nonce, SECRETSHA256);
        secretseed =u8aToString(secretseedu8);
        if(secretseed.length==0){
            console.log("[Error] Password is wrong.");
            process.exit(1) 
        }
    }
    // generating key ring from secret seed
    const username="AON"
    const keyring = new Keyring({ type: 'sr25519' });
    console.log("[Info] generating key ring from secret seed");
    const keysPair = keyring.addFromUri(secretseed, { name: username });
    const aonAccountId=`${keysPair.address}`;
    console.log("[Info] Account Id of this node: "+aonAccountId);
    
    // check for configuration file
    if(!fs.existsSync(CONFILE)){
        console.log("[Error] Configuration file not found: "+CONFILE);
        return;
    }
    // load configuration file
    let confjson=""
    try {
        confjson= fs.readFileSync(CONFILE, 'utf8')
    } catch (err) {
        console.log("[Error] " +err);
        process.exit(1);
    }
    //parse Json configuration
    let conf;
    try {
        conf = JSON.parse(confjson);
    }catch (err) {
        console.log("[Error] " +err);
        process.exit(1);
    }
    // get last block number
    const lastHeader = await api.rpc.chain.getHeader();                
    let header = JSON.parse(lastHeader);
    console.log("[Info] Last block stored: "+header.number);

    // listen to events
    api.query.system.events((events) => {
        console.log(`[Info] Received ${events.length} events:`);
        // Loop through the Vec<EventRecord>
        events.forEach((record) => {
            // Extract the phase, event and the event types
            const { event, phase } = record;
            const types = event.typeDef;
            // Show the events details
            const eventdesc=`${event.section}:${event.method}`;
            console.log(`[Info] ${event.section}:${event.method}:: (phase=${phase.toString()})`);
            // check for a requet of update
            if(eventdesc=="acria:RequestOracleUpdate"){
                let oracleid=event.data[0].toString();
                let oracleaccount=event.data[1].toString();
                let oracleparameters=event.data[2].toString();
                //check if the target is this Oracle
                conf.api.forEach( oracleconf => {
                    if (oracleconf.accountid==oracleaccount && oracleconf.oracleid==oracleid) {
                        console.log("[Info] Calling Endpoint: "+oracleconf.endpoint);
                        // replace variables eventually present in the endpoint
                        let endpoint=aon_replace_url_with_parameters(oracleconf.endpoint,oracleparameters);
                        // POST OF JSON fields to the endpoint
                        if(oracleconf.method.toUpperCase()=="POST"){
                            fetch(endpoint,
                                { method: 'POST',
                                  headers:{'Content-Type': 'application/json;charset=utf-8'},
                                  body: JSON.stringify(oracleparameters)
                                })
                                .then(res => res.text())
                                .then(text => update_blockchain(api,keysPair,oracleconf.oracleid,text));                
                        }
                        // GET of the endpoint
                        else{
                            method='GET';
                            fetch(endpoint)
                                .then(res => res.text())
                                .then(text => update_blockchain(api,keysPair,oracleconf.oracleid,text));                
                        }
                        
                    }
                });
            } else {
            // Loop through each of the parameters, displaying the type and data
            event.data.forEach((data, index) => {
                console.log(`[Info] Data Info ${types[index].type}: ${data.toString()}`);
            });
          }
        });
    }); // end events listening
}

// function to write the result into the blockchain
async function update_blockchain(api,keysPair,oracleid,data){
    console.log(`[Info] Writing Oracle Data`);
    const unsub =  api.tx.acria.oracleUpdate(oracleid,data).signAndSend(keysPair,(result) => {
        if (result.status.isInBlock) {
            console.log(`[Info] Writing Oracle Data - Transaction included at blockHash ${result.status.asInBlock}`);
          } else if (result.status.isFinalized) {
            console.log(`[Info] Writing Oracle Data - Transaction finalized at blockHash ${result.status.asFinalized}`);
          }else{
            console.log(`[Info] Writing Oracle Data - Status ${result.status}`);
          }
    });
}

// function to replace the variables in the endpoint with the parameters received
// the parameters are URL escaped
function aon_replace_url_with_parameters(url,parameters){
    let jp;
    let urlv=url;
    // try to decode a json structure, if it's not valid return the url without changes
    try{
         jp=JSON.parse(parameters);
    }
    catch(err){
        return url;
    }
    //replace variable by variable
    for (var j in jp) {
        if( jp.hasOwnProperty(j) ) {
            let v='%'+j+'%';
            console.log("urlv:",urlv);
            urlv=urlv.replace(v,encodeURI(jp[j]));
            console.log("name:",j,"value:",jp[j]);
        }
    }
    return(urlv);
}

