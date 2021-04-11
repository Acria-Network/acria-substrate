// Acria Oracle Bridge
// It allows to edit the configuration file and save it

// pulling required libraries
let express = require('express');
let fs = require('fs');

//start program
console.log("[info] - Acria Oracle Bridge - ver. 1.00 - Starting");
mainloop();
async function mainloop(){
    //setup express server for http interaction
    let app = express();
    app.use(express.urlencoded({ extended: true })); // for parsing application/x-www-form-urlencoded

    // get main dashboard
    app.get('/',function(req,res){     
        let v=read_file("acria-oracle-bridge.html");
        res.send(v);
    });
    // get configuration in json format
    app.get('/getconfiguration',function(req,res){     
        let confFile="./acria-oracle-node.conf";
        let j='{}';
        if (fs.existsSync(confFile)) {
            j=read_file(confFile);   
            try{
                let conf=JSON.parse(j); 
            } catch (error) {
                console.log('[Error] decoding Json configuration:' +error);
                j='{}';
            }
        }        
        // send back json structure
        res.send(j);
    });

    // acria-logo.png 
    app.route('/logo').get(function(req,res)
    {
        let s = fs.createReadStream("acria-logo.png");
        s.on('open', function () {
            res.set('Content-Type', 'image/png');
            s.pipe(res);
        });
        s.on('error', function () {
            res.set('Content-Type', 'text/plain');
            res.status(404).end('logo.png not found');
        });
    });
    // acria-oracle-banner.png 
    app.route('/banner').get(function(req,res)
    {
        let s = fs.createReadStream("acria-oracle-banner.png");
        s.on('open', function () {
            res.set('Content-Type', 'image/png');
            s.pipe(res);
        });
        s.on('error', function () {
            res.set('Content-Type', 'text/plain');
            res.status(404).end('logo.png not found');
        });
    });
    //save Oracle Data
    app.get('/saveoracle',function(req, res) {    
        // checking data
        if(req.query.method!="get" && req.query.method!="post"){
            res.send('{"answer":"KO","message":"Wrong method, it can be only post or get"}');        
            return;
        }
        if(req.query.endpoint.length==0){
            res.send('{"answer":"KO","message":"Endpoint cannot be empty"}');  
            return;
        }
        if(req.query.oracleid<=0){
            res.send('{"answer":"KO","message":"Oracle id must be set to a numeric value"}');  
            return;
        }
        if(req.query.accountid.length==0){
            res.send('{"answer":"KO","message":"Account id cannot be empty"}');  
            return;
        }
        save_oracle(req.query.accountid,req.query.oracleid,req.query.endpoint,req.query.method);
        res.send('{"answer":"OK","message":"Oracle configuration has been saved"}');  
    });
    //delete Oracle Data
    app.get('/deleteoracle',function(req, res) {    
        // checking data
        if(req.query.oracleid<=0){
            res.send('{"answer":"KO","message":"Oracle id is missing"}');  
            return;
        }
        if(req.query.accountid.length==0){
            res.send('{"answer":"KO","message":"Account id cannot be empty"}');  
            return;
        }
        delete_oracle(req.query.accountid,req.query.oracleid,res);
    });

    // server http is listening to  port tcp/3000
    console.log("[info] - listening for connections on port TCP/3000...");
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
// function to add/replace an oracle in the configuration
function save_oracle(accountid,oracleid,endpoint,method){
    const fs = require('fs');
    // load current configuration
    let confFile="./acria-oracle-node.conf";
    let j=read_file(confFile);
    let conf='';
    try{
        conf=JSON.parse(j); 
    } catch (error) {
        console.log('[Error] decoding Json configuration:' +error);
        j='';
    }
    // if the configuration file is empty we create the file
    if(j.length==0){
        let w='{"api":[{"accountid":"'+accountid+'","oracleid":'+oracleid+',"endpoint":"'+endpoint+'","method":"'+method+'"}]}';
        fs.writeFileSync(confFile,w);
        return;
    }else {
        // search for the same accountid and oracle id
        let flag=false;
        for (r in conf.api){
            if(conf.api[r].oracleid==oracleid && conf.api[r].accountid==accountid){
                conf.api[r].endpoint=endpoint;
                conf.api[r].method=method;
                flag=true;
                let w=JSON.stringify(conf);
                fs.writeFileSync(confFile,w);
                return;
            }
        }
        if(flag==false){
            let row={};
            row.accountid=accountid;
            row.oracleid=oracleid;
            row.endpoint=endpoint;
            row.method=method;
            conf.api.push(row);
            let w=JSON.stringify(conf);
                fs.writeFileSync(confFile,w);
                return;
        }
    }
}
// function to delete an oracle configuration
async function delete_oracle(accountid,oracleid,res){
    const fs = require('fs');
    // load current configuration
    let confFile="./acria-oracle-node.conf";
    let j=read_file(confFile);
    let conf='';
    try{
        conf=JSON.parse(j); 
    } catch (error) {
        console.log('[Error] decoding Json configuration:' +error);
        res.send('{"answer":"OK","message":"Oracle configuration has been deleted"}');  
        return;
    }
    if(j.length==0){
        console.log("[Info] Empty configuraton, nothing to remove");
        res.send('{"answer":"OK","message":"Oracle configuration has been deleted"}');  
        return;
    }
    // search for accountid,oracleid
    for (r in conf.api){
        if(conf.api[r].oracleid==oracleid && conf.api[r].accountid==accountid){
            conf.api.splice(r,1);
            let w=JSON.stringify(conf);
            fs.writeFileSync(confFile,w);
            res.send('{"answer":"OK","message":"Oracle configuration has been deleted"}');  
            console.log("[Info] Oracle removed");
            return;
        }
    }
}


