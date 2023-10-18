#![no_std]
use soroban_sdk::{contract,contractimpl,contracttype,log,Env,Address};

#[contracttype]
pub enum Owner{
    Owner,
    Lvl1(Address),
    Lvl2(Address)
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
    
    pub fn get_status_lvl1(env:Env, address:Address) -> bool{
        env.storage().persistent().get(&Owner::Lvl1(address.clone())).unwrap()
    }
    pub fn get_status_lvl2(env:Env, address:Address) -> bool{
        env.storage().persistent().get(&Owner::Lvl2(address.clone())).unwrap()
    }
}
    





