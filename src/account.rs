use rand::Rng;

// struct for our account
pub(crate) struct Account{
    username: String,
    email: String,
    pub(crate) account_id: u32,
    balance: i32

}


//implementaiton of Account Struct
impl Account {
    pub(crate) fn new(username: String, email: String) -> Self{
    let mut rng = rand::thread_rng();
        let acc_id = rng.gen::<u32>(); // generate random number for account id
        Self {username,email,account_id:acc_id,balance:0}
    }

    pub(crate) fn update(&mut
                         self,username: &str,email: &str){
        if !username.is_empty() {self.username = username.to_string();}
        if !email.is_empty() {self.email = email.to_string();}
    }


    pub(crate) fn deposit(&mut self, amount: i32 ){
        if amount != 0 {self.balance += amount;}
    }

    pub(crate) fn withdraw(&mut self, amount: i32 ){
        if amount != 0 {self.balance -= amount;}
    }



}

//handles the display of our struct
impl std::fmt::Display for Account {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, " ================\n««««««  Account Name: {} »»»»»\n««««««  Account number: {} »»»»»\n««««««  Balance: {}. »»»»»\n
        »»»»»»»»»»»»»»»»»»»»«««««««««««««««««««««««««««", self.username,self.account_id, self.balance)
    }
}