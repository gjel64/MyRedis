use std::collections::HashMap;
use crate::values::{Value};

// TODO: passer tout les option => Result<>

pub struct Database {
    data: HashMap<String, Value>
}


impl Database {

    // ------------------------- PRIVATE -------------------------

    fn unpack_args(&self, args: String) -> Vec<String> {
        // returns a vec of differents args
        args.split_whitespace().map(|v| v.to_string()).collect()
    }



    // ------------------------- PUBLIC -------------------------

    pub fn get(&mut self, args: String) -> Result<&Value, &'static str> {
        // return all the value contained for this key
        // for exemple : get = hgetall for the hash in redis

        let params = self.unpack_args(args);

        if params.len() > 1 || params.len() == 0 {
            return Err("ERROR: wrong number of arguments");
        }
        match self.data.get(&params[0]) {
            None => return Err("ERROR: Fail to get the value of the key (probably doesn't exit)"),
            Some(val) => return Ok(val)
        }
    }


    pub fn set(&mut self, args: String) -> Result<String, &'static str>{
        // If the map did not have this key present, None is returned.
        // If the map did have this key present, the value is updated, and the old value is returned. The key is not updated, though; this matters for types that can be == without being identical. See the module-level documentation for more.
        let params = self.unpack_args(args);

        let len = params.len();
        if len >= 3 || len <= 1 {
            return Err("ERROR: Wrong number of params")
        }

        match  self.data.insert(params[0].clone(), params[1].clone().into()) {
            _ => Ok("OK: Inserted".to_string())
        }
    }


    pub fn hset(&mut self, args: String) -> Result<String, &'static str>{
        let params = self.unpack_args(args);
        if params.len() <= 2 || (params.len() - 1) % 2 != 0 {
            return Err("ERROR: wrong number of arguments for HSET");
        }

        let value = self.data
            .entry(params[0].clone())
            .or_insert_with(Value::new_hash);

        let hash = match value.data.as_hash_mut() {
            Some(h) => h,
            None => return Err("ERROR: found another type in this key instead of hash"),
        };

        for pair in params[1..].chunks_exact(2) {
            hash.insert(pair[0].clone().into_bytes(), pair[1].clone().into_bytes());
        }

        Ok("OK: Inserted".to_string()) 
    }
    


    // ------------------------- SPECIAL METHODS -------------------------

    pub fn new() -> Database {
        Database { data: HashMap::new() }
    }


    pub fn exec_command(&mut self, command: String) -> Result<String, &'static str> {

        let args: String;
        
        let fn_name = match command.find(" ") {
            Some(idx) => {
                args = command[idx..].to_string();
                &command[..idx]
            },
            None => return Err("ERROR: syntax error on command : <COMMAND> <ARGS> "),
        };
        
        let res = match fn_name {
            // special cases
            "get" | "GET" => {
                match self.get(args) {
                    Ok(v) => Ok(v.data().to_str().unwrap()),
                    Err(s) => return Err(s)
                }
            },

            // normal cases
            "set" | "SET" => self.set(args),
            "hset" | "HSET" => self.hset(args),
            _ => return Err("ERROR: command don't exist")
        };

        res
    }


}