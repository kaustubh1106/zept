#![no_std]
use soroban_sdk::{contract,contractimpl,contracttype,log,Env,Address,Symbol};

#[contracttype]
pub enum Owner{
    Owner,
    Lvl1(Address),
    Lvl2(Address),
    Lvl3(Address)
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[contracttype]
pub enum Music{
    Issuer,
    Category,
    TokenName,
    Burnable,
    BaseUri,
    Timespan
}

pub fn get_status_lvl1(env:&Env, address:&Address) -> bool{
        env.storage().persistent().get(&Owner::Lvl1(address.clone())).unwrap()
    }

pub fn get_status_lvl2(env: &Env, address: &Address) -> bool{
        env.storage().persistent().get(&Owner::Lvl2(address.clone())).unwrap()
    }

pub fn get_status_lvl3(env: &Env, address: &Address) -> bool{
        env.storage().persistent().get(&Owner::Lvl3(address.clone())).unwrap()
    }
#[contract]
pub struct Contract;


#[contractimpl]
impl Contract{
    pub fn set(env: Env,address: Address) ->bool {
        if !env.storage().persistent().has(&Owner::Owner) {
            env.storage().persistent().set(&Owner::Owner,&address);
            true
        }
        else {
            false
        }
    }
    
    pub fn lvl1(env: Env,address: Address) {
        let owner:Address = env.storage().persistent().get(&Owner::Owner).unwrap();
        owner.require_auth();
        env.storage().persistent().set(&Owner::Lvl1(address), &true);
    }
    pub fn lvl2(env: Env,address: Address) {
        let owner:Address = env.storage().persistent().get(&Owner::Owner).unwrap();
        owner.require_auth();
        let a:bool = env.storage().persistent().get(&Owner::Lvl1(address.clone())).unwrap();
        if a {
            env.storage().persistent().set(&Owner::Lvl2(address), &true);
        }
        else{
            log!(&env, "lvl1 not done");
        }
    }

    pub fn lvl3(env: Env,address: Address) {
        let owner:Address = env.storage().persistent().get(&Owner::Owner).unwrap();
        owner.require_auth();
        let a:bool = env.storage().persistent().get(&Owner::Lvl1(address.clone())).unwrap();
        let b:bool = env.storage().persistent().get(&Owner::Lvl2(address.clone())).unwrap();
        if a && b{
            env.storage().persistent().set(&Owner::Lvl3(address), &true);
        }
        else{
            log!(&env, "below levels not done");
        }
    }
    

    pub fn create(env:Env,issuer: Address,category:  Symbol, token_name: Address, burnable: bool, baseuri: Symbol,timespan: u64){
        
        owner.require_auth();
        assert!(get_status_lvl1(&env, &issuer),"not verified at lvl1");        
        assert!(get_status_lvl2(&env, &issuer),"not verified at lvl2");  
        assert!(get_status_lvl3(&env, &issuer),"not verified at lvl3"); 
        env.storage().persistent().set(&Music::Issuer,&issuer);
        env.storage().persistent().set(&Music::Category,&category);
        env.storage().persistent().set(&Music::TokenName,&token_name);
        env.storage().persistent().set(&Music::Burnable,&burnable);
        env.storage().persistent().set(&Music::BaseUri,&baseuri);
        env.storage().persistent().set(&Music::Timespan,&timespan);
    
    }
}