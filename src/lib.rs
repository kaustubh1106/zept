#![no_std]
use soroban_sdk::{contract,contractimpl,contracttype,log,Env,Address};

#[contracttype]
pub enum Owner{
    Owner,
    Lvl1(Address),
    Lvl2(Address),
    Lvl3(Address)
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

    pub fn get_status_lvl1(env:Env, address:Address) -> bool{
        env.storage().persistent().get(&Owner::Lvl1(address.clone())).unwrap()
    }
    pub fn get_status_lvl2(env:Env, address:Address) -> bool{
        env.storage().persistent().get(&Owner::Lvl2(address.clone())).unwrap()
    }

    pub fn get_status_lvl3(env:Env, address:Address) -> bool{
        env.storage().persistent().get(&Owner::Lvl3(address.clone())).unwrap()
    }

    pub fn create(env:Env, issuer: Address, category: &str, token_name: &str, burnable: bool, base_uri : String, timespan: ){
        issuer.require_auth();
        
    }
}
    // #[action]
// pub fn deletedelphi(delphi_pair_name: Name) {
//     require_auth(&self);

//     let mut config_table = s_configtoken(&self, &self.into());
//     assert!(config_table.exists(), "Config table does not exist, setconfig first");

//     let mut current_config = config_table.get();

//     let pairs_supported = current_config.supported_symbol_pairs.clone();
//     let mut new_pairs_supported = Vec::<SYMBOLPAIR>::new();

//     for i in 0..pairs_supported.len() {
//         if pairs_supported[i].delphi_pair_name == delphi_pair_name {
//             continue;
//         }
//         new_pairs_supported.push(pairs_supported[i].clone());
//     }

//     current_config.supported_symbol_pairs = new_pairs_supported;

//     config_table.set(&current_config, &self);
// }


// #[action]
// pub fn setconfig(version: String, admin: Name) {
//     require_auth(&self);

//     let mut configs = s_configtoken(&self, &self.into());
//     configs.set(&configtoken {
//         name: "dnft".into(),
//         version,
//         supported_symbol_pairs: vec![],
//         delphioracle_account: delphioracle::DELPHIORACLE_ACCOUNT.into(),
//     }, &self);

//     let mut state_table = s_stateinfo(&self, &self.into());
//     state_table.set(&stateinfo {
//         admin,
//         value: 91,
//     }, &self);

// }
    





