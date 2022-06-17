use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use crate::account::Account;

//The manager is responsible for controlling the activiities of the account
pub(crate) struct Manager {
  pub  bank_name: String,
   pub branch_location: String,
   pub account_list: HashMap<u32,Account>
}

impl Manager {
    pub fn new(bank_name: String, branch_location: String) -> Self{
        Self{ bank_name, branch_location,account_list: HashMap::new()}
    }

    pub(crate) fn register_account(&mut self, username: &str, email: &str) {
        let account: Account = Account::new(username.to_string(),email.to_string());
        self.account_list.insert(account.account_id, account);

    }


    pub(crate) fn update_account(&mut self, account_id: &u32, username:&str, email:&str) {
        let acc: Option<&mut Account> = self.account_list.get_mut(account_id);
        if(acc.is_some()){
            let mut macc: &mut Account = acc.unwrap();
            macc.update(username, email);
        }

    }

    pub fn remove_account_records(&mut self, account_id: &u32 ) -> bool {
        self.account_list.remove(&account_id).expect("Could not find any user with your email");
        !self.account_list.contains_key(account_id)
    }

    pub(crate) fn deposit(&mut self, account_id: &u32, amount: i32) {
        let acc: Option<&mut Account> = self.account_list.get_mut(account_id);
        if(acc.is_some()){
            let mut macc: &mut Account = acc.unwrap();
            macc.deposit(amount);
        }
    }

    pub(crate) fn withdraw(&mut self, account_id: &u32, amount: i32) {
        let acc: Option<&mut Account> = self.account_list.get_mut(account_id);
        if acc.is_some() {
            let mut macc: &mut Account = acc.unwrap();
            macc.withdraw(amount);
        }
    }

    pub(crate) fn transfer(&mut self, account_id_from: &u32, account_id_to: &u32, amount: i32) -> bool {
        self.withdraw(account_id_from,amount);
        self.deposit(account_id_to,amount);
        true

    }


    pub fn list_all_accounts(&self) {
        let acc_list: &HashMap<u32, Account> = self.account_list.borrow();
        for (key, value) in acc_list {
            println!("{}", value);
        }
    }
}