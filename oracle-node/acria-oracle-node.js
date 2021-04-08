// Acria Oracle Node 
// AON listens to events propagated on the blockchain, when a data update request match its own configuration,
// it make its work writing the blockchain with the new data requested.
const { ApiPromise, WsProvider } = require('@polkadot/api');    
const { Keyring } = require('@polkadot/api');
const {mnemonicGenerate,mnemonicToMiniSecret,naclKeypairFromSeed,naclDecrypt,naclEncrypt,} = require('@polkadot/util-crypto');
const { u8aToHex,stringToU8a,u8aToString,hexToU8a,isHex } =require('@polkadot/util');
const fetch = require('node-fetch');
const fs = require('fs');

// customization section - you can change the following constants upon your preferences
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
const CONFILE="./acria-oracle-node.conf";           //configuration file
const KEYFILE="./acria-oracle-node.key";            // where to store the keys
const LOGFILE="./acria-oracle-node.log";            //log file name
// end customization section

// we launch the main loop in async
console.log("[Info] Acria Oracle Node v. 1.00 - Starting");
mainloop();

// main loop
async function mainloop(){
    
    const api = await ApiPromise.create({ provider: wsProvider });      // create API object
    // generate local keys if not yet done
    if(!fs.existsSync(KEYFILE) ) {
        
        console.log("[Info] Generating Key Pairs");
        //**  FOR PRODUCTION
        //const randomseed = mnemonicGenerate(24);
        //**  FOR DEVELOPMENT we use Alice account
        const randomseed = 'bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice';
        console.log(`[Info] Generated mnemonic seed: ${randomseed}`);
        fs.appendFileSync(KEYFILE, randomseed, 'utf8');
    }
    // Load secret seed
    let secretseed="";
    try {
        secretseed= fs.readFileSync(KEYFILE, 'utf8')
    } catch (err) {
        console.log("[Error] " +err);
        process.exit(1);
    }
    // generating key ring
    const username="AON"
    const keyring = new Keyring({ type: 'sr25519' });
    console.log("[Info] generating key ring from secret seed");
    const keysPair = keyring.addFromUri(secretseed, { name: username });
    const aonAccountId=`${keysPair.address}`;
    console.log("[Info] Account Id of this node: "+aonAccountId);
    
    //console.log(keyspair);
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

