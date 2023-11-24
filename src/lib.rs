#![no_std]
use soroban_sdk::{contract,contractimpl,contracttype,log,Env,Address,Symbol,Vec,token,symbol_short,vec};

    const Categoryid: Symbol = symbol_short!("Categryid");
    const Accounts: Symbol = symbol_short!("Accounts");
    const AskTable: Symbol = symbol_short!("asktable");
 
#[contracttype]
pub enum Owner{
    Owner,
    Lvl1(Address),
    Lvl2(Address),
    Lvl3(Address)
}

// #[repr(u32)]
#[contracttype]
#[derive(Clone,Debug, PartialEq, Eq)]
pub struct Music{
    Issuer: Address,
    Category: Symbol,
    TokenName: Address,
    Burnable: bool,
    BaseUri: Symbol,
    Timespan: u64,
    CatgoryID: u64,
}

#[contracttype]
#[derive(Clone,Debug, PartialEq, Eq)]
pub struct SaleDetails {
    Seller: Address,
    CategoryID: u64,
    PercentShare: u64,
    PerPercentAmt: u64,
    StartTime: u64,
    EndTime: u64,
    MinContribution: u64,
    MaxContribution: u64,
    SoftCap: u64,
    }

    #[contracttype]
    #[derive(Clone,Debug, PartialEq, Eq)]
    pub struct Asks {
        Saleid:u64,
        Tokename: Symbol,
        Category: Symbol,
        Seller: Address,
        Peramount: u64,
        Pershare:u64,
        Start:u64,
        End:u64,
        Min:u64,
        Max:u64,
        Softcap:u64,
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

pub fn showcategoryid(env: &Env) -> u64{
        let mut categoryid = 
env.storage().persistent().get(&Categoryid).unwrap_or_else(|| 0);
        categoryid
    }

pub fn increasecategoryid(env: &Env) -> u64{
        let mut categoryid = 
env.storage().persistent().get(&Categoryid).unwrap_or_else(|| 0);
    categoryid = categoryid+1;
    env.storage().persistent().set(&Categoryid,&categoryid);
        categoryid
    
    }

pub fn transfer1(env:&Env,to:&Address,amount:&i128,token_id:&Address){
    let client = token::Client::new(env, token_id);
    client.transfer(&env.current_contract_address(), to, amount);
}

pub fn transfermto(env:&Env,from:&Address , to:&Address,amount:&i128,token_id:&Address){
    let client = token::Client::new(env, token_id);
    client.transfer(from, to, amount);
}

pub fn createvector(env: &Env) -> Vec<Music>{
    let mut vec2= Vec::new(env);
    vec2
}

pub fn createasktable(env: &Env) -> Vec<Asks>{
    let mut vec2= Vec::new(env);
    vec2
}

#[contract]
pub struct Contract;

// let mut music_list: Vec<Music> = vec![&Env];

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
        //owner.require_auth();
        env.storage().persistent().set(&Owner::Lvl1(address), &true);
    }
    pub fn lvl2(env: Env,address: Address) {
        let owner:Address = env.storage().persistent().get(&Owner::Owner).unwrap();
        //owner.require_auth();
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
        //owner.require_auth();
        let a:bool = env.storage().persistent().get(&Owner::Lvl1(address.clone())).unwrap();
        let b:bool = env.storage().persistent().get(&Owner::Lvl2(address.clone())).unwrap();
        if a && b{
            env.storage().persistent().set(&Owner::Lvl3(address), &true);
        }
        else{
            log!(&env, "below levels not done");
        }
    }
    

    pub fn create(env:Env,issuer: Address,category:  Symbol, token_name: Address, burnable: bool, baseuri: Symbol,timespan: u64,category_id:u64){
        
        //issuer.require_auth();
        assert!(get_status_lvl1(&env, &issuer),"not verified at lvl1");  
        assert!(get_status_lvl2(&env, &issuer),"not verified at lvl2");  
        assert!(get_status_lvl3(&env, &issuer),"not verified at lvl3"); 
          let mut categoryid: u64 = showcategoryid(&env);
        let music = Music {
        Issuer: issuer,
        Category: category,
        TokenName: token_name,
        Burnable: burnable,
        BaseUri: baseuri,
        Timespan: timespan,
        CatgoryID: category_id,
        };
        let mut music_list: Vec<Music> = env.storage().persistent().get(&Accounts).unwrap_or_else(|| createvector(&env));
        music_list.push_back(music);

categoryid = increasecategoryid(&env);        
        env.storage().persistent().set(&Accounts, &music_list);
        env.storage().persistent().set(&Categoryid, &categoryid);
}



    pub fn listsale(
    env: Env, seller: Address, category: u64, pershare: u64, peramount: u64, starttime: u64,endtime: u64,
    mincontribution: u64,
    maxcontribution: u64,
    softcap: u64,
    ) {
    // seller.require_auth();

    let sale_details = SaleDetails {
        Seller: seller.clone(),
        CategoryID:category,
        PercentShare: pershare,
        PerPercentAmt: peramount,
        StartTime: starttime,
        EndTime: endtime,
        MinContribution: mincontribution,
        MaxContribution: maxcontribution,
        SoftCap: softcap,
    };

    //env.storage().persistent().set(&seller, &sale_details);
    env.storage().persistent().set(&Accounts, &sale_details)
}

    // pub fn transfer(env:Env,from : Address, amount:i128,token_id: Address){
    //     transfer1(&env,&from,&amount,&token_id);
    // }

    //TABLES ----------
    pub fn saletable(env: Env) -> Vec<Music>{
        env.storage().persistent().get(&Accounts).unwrap()
    }

    pub fn addask_table(
        env:Env,
        saleid:u64,
        tokename: Symbol,
        seller: Address,
        peramount: u64,
        pershare:u64,
        start:u64,
        end:u64,
        min:u64,
        softcap:u64 )->Vec<Asks>
    {
        let music:Symbol = symbol_short!("music");
        let ask = Asks {
            Saleid:saleid,
            Tokename: tokename,
            Category: music,
            Seller: seller,
            Peramount: peramount,
            Pershare:pershare,
            Start:start,
            End:end,
            Min:min,
            Max:100,
            Softcap:softcap,
            };
        let mut ask_list: Vec<Asks> = env.storage().persistent().get(&AskTable).unwrap_or_else(|| createasktable(&env));
        ask_list.push_back(ask);
        env.storage().persistent().get(&AskTable).unwrap()
    }

    //suspend sale

    pub fn suspendsale(env: Env, category_id: u32){
            let ask: Vec<Asks> = env.storage().persistent().get(&AskTable).unwrap();
        let result = ask.get(category_id);
        match result {
    Some(resultsc) => {
        let sc = resultsc.Softcap;
        let endtime = resultsc.End;
        assert!(endtime<env.ledger().timestamp(),"sale not ended");
    }
            None =>{
                // return karo jo bhi error hai
            }
     }
    
    
    //logic ke hisab se handle karna hai

    // if sc>
    }

    pub fn 

}
