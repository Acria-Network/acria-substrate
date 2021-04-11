// Web App for Staking/Un-Staking Acria tokens to an Oracle Account/Oracle id

// pull required modules
let express = require('express');
const { ApiPromise, WsProvider } = require('@polkadot/api');   
const { Keyring } = require('@polkadot/api');
const cookieParser = require('cookie-parser')


console.log("[info] - Acria Network - Stakers User Interface - Starting");
// customization section - you can change the following constants upon your preferences
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
// end customization section 

// execute main loop as async function because of "await" requirements that cannot be execute from the main body
mainloop();
// main loop function
async function mainloop(){
    //connect to local substrate node (it will retry automatically the connection if not reachable)
    const api = await ApiPromise.create({ provider: wsProvider });  
    // configure http server
    let app = express();
    app.use(express.urlencoded({ extended: true })); // for parsing application/x-www-form-urlencoded
    app.use(cookieParser());
    //main dashboard loaded from index.html
    app.get('/',function(req,res){             
        let v=read_file("stakers.html");
        res.send(v);
    });
    //get last block written on the blockchain
    app.route('/lastblock').get(function(req,res)
    {
         get_last_block(res,api);
    });
    // get account balance from session token
    app.route('/accountbalance').get(function(req,res)
    {
        try {
            sender=req.cookies.sender;
        }
        catch{
            sender='';
        }
        if(sender.length==0){
            res.send('{"balance":"0","account":"Waiting for a first transaction"}');
        }else{
            get_balance_account(res,api,sender);
        }
    });
    //get last log data
    app.route('/logdata').get(function(req,res)
    {
        const fs = require('fs')
        let logh='<center><h2>Events</h2></center><table class="table table-striped"><tr><th>Date/Time</th><th>Event Description</th><tr>';
        let logf='</table>'
        let filename='./acria-stakers.log';
        let log=read_file(filename);
        if(log==undefined){
            res.send(logh+logf);
            return;
        }
        let logr=log.split("\n");
        let logs=logh;
        let lst=logs.length-1;
        for(i=lst;i>=0;i--){
            if(logr[i]==undefined)
                continue;
            if(logr[i].length==0)
                continue;
            logs=logs+logr[i];
        }
        logs=logs+logf;
        res.send(logs);
    });
    // stake funds
    app.route('/stakeoracle').get( async function(req,res)
    {
        // check data
        accountid=req.query.accountid;
        if(accountid.length==0){
            res.send('{"answer":"KO","message":"account id is missing"}');        
            return;
        }
        oracleid=req.query.oracleid;
        if(oracleid<=0){
            res.send('{"answer":"KO","message":"oracle id is missing"}');        
            return;
        }
        amountlock=req.query.amountlock;
        if(amountlock==0){
            res.send('{"answer":"KO","message":"amount to lock cannot be zero"}');        
            return;
        }
        secretseed=req.query.secretseed;
        if(secretseed.length==0){
            res.send('{"answer":"KO","message":"Secret seed cannot be empty"}');        
            return;
        }
        const keyring = new Keyring({ type: 'sr25519' });
        const loggeduser = keyring.addFromUri(secretseed,{name: '' });
        //store account in cookie
        const sender=`${loggeduser.address}`;
        res.cookie('sender', encodeURI(sender));
        let am=amountlock*10000000000; //conversion in 10m
        //write blockchain for staking
        const unsub = await api.tx.acria.lockOracleStakes(accountid,am).signAndSend(loggeduser,(result) => {
            if (result.status.isInBlock) {
                console.log(`[info] Staking funds - Transaction included at blockHash ${result.status.asInBlock}`);
                write_log(`[info] Staking funds - Transaction included at blockHash ${result.status.asInBlock}`);
            } else if (result.status.isFinalized) {
                console.log(`[info] Funds locked - Transaction finalized at blockHash ${result.status.asFinalized}`);
                write_log(`[info] Funds locked - Transaction finalized at blockHash ${result.status.asFinalized}`);
                unsub();
            }
        });
        res.send('{"answer":"OK","message":"transaction has been submitted"}');        

    });
    // unstake funds
    app.route('/unstakeoracle').get( async function(req,res)
    {
        // check data
        accountid=req.query.accountid;
        if(accountid.length==0){
            res.send('{"answer":"KO","message":"account id is missing"}');        
            return;
        }
        oracleid=req.query.oracleid;
        if(oracleid<=0){
            res.send('{"answer":"KO","message":"oracle id is missing"}');        
            return;
        }
        secretseed=req.query.secretseed;
        if(secretseed.length==0){
            res.send('{"answer":"KO","message":"Secret seed cannot be empty"}');        
            return;
        }
        const keyring = new Keyring({ type: 'sr25519' });
        const loggeduser = keyring.addFromUri(secretseed,{name: '' });
        //store account in cookie
        const sender=`${loggeduser.address}`;
        res.cookie('sender', encodeURI(sender));
        //write blockchain for staking
        const unsub = await api.tx.acria.unlockOracleStakes(accountid).signAndSend(loggeduser,(result) => {
            if (result.status.isInBlock) {
                console.log(`[info] Unstaking funds - Transaction included at blockHash ${result.status.asInBlock}`);
                write_log(`[info] Unstaking funds - Transaction included at blockHash ${result.status.asInBlock}`);
            } else if (result.status.isFinalized) {
                console.log(`[info] Funds unlocked - Transaction finalized at blockHash ${result.status.asFinalized}`);
                write_log(`[info] Funds unlocked - Transaction finalized at blockHash ${result.status.asFinalized}`);
                unsub();
            }
        });
        res.send('{"answer":"OK","message":"transaction has been submitted"}');        

    });

    // logo output
    app.route('/logo').get(function(req,res)
    {
        const fs = require('fs')
        let s = fs.createReadStream("stakers-logo.png");
        s.on('open', function () {
            res.set('Content-Type', 'image/png');
            s.pipe(res);
        });
        s.on('error', function () {
            res.set('Content-Type', 'text/plain');
            res.status(404).end('stakers-logo.png not found');
        });
    });
    // get oracle list
    app.route('/oracleslist').get(async function(req,res)
    {
        const oracles = await api.query.acria.oracle.entries();
        let j='{"oracles":[';
        let x=0;
        oracles.forEach(([key, oracle]) => {
            let k=key.args.map((k) => k.toHuman());
            let accountid=k[0];
            let oracleid=k[1];
            let oracledata=oracle.toHuman();
            //get stakes if any
            //const stakes = await api.query.acria.oracleStakes(accountid);
            let jr = '{"accountid":"'+accountid+'","oracleid":"'+oracleid+'",'+oracledata.substring(1);
            if(x>0){
                j=j+',';
            }
            j=j+jr;
            x=x+1;
        });
        j=j+']}';
        res.send(j);
    });

    // http server listening to server port 3000
    console.log("[info] - listening for connections on port 3000...");
    let server=app.listen(3000,function() {});

}
//function to return content of a file name
function read_file(name){
    const fs = require('fs');
    if(!fs.existsSync(name))
        return(undefined);
    try {
        const data = fs.readFileSync(name, 'utf8')
        return(data);
      } catch (err) {
        console.error(err);
        return(undefined);
      }
}
//function to get last block of the blockchain
async function get_last_block(res,api){
    const lastHeader = await api.rpc.chain.getHeader();                
    res.send(lastHeader);
}
//function to get last block of the blockchain
async function get_balance_account(res,api,accountid){
    let { data: { free: previousFree }, nonce: previousNonce } = await api.query.system.account(accountid);           
    let balance=previousFree/10000000000;
    let balancestr=balance.toLocaleString();
    let a='{"balance":"'+(balancestr)+'","account":"'+accountid+'"}'
    res.send(a);
}
//function to write log file
function write_log(d){
    const fs = require('fs')
    let filename='./acria-stakers.log';
    console.log("[info] - Writing log to "+filename+" ["+d+"]");
    let dt=new Date().toISOString().replace(/T/, ' ').replace(/\..+/, '')
    let dtlog='<tr><td>'+dt+'</td><td>'+d+'</td></tr>\n';
    fs.appendFileSync(filename, dtlog, 'utf8');
}


