mod account_manager;
mod account;
use std::io;
use account_manager::Manager;
use account::Account;

fn main() {
    println!("Hello, Welcome to the Banking System! Application");
    let mut user_command = String::new(); //variable for the user command
    let mut acm: Manager = Manager::new(String::from("Daramola Bank"), String::from("Abuja"));
    println!("========================================\n========================================\n
    \tPlease enter: \n\t1 to create a new account \n\t2 to show all list of accounts \n\t3 to update account. \n\t4  to delete account. \n\t 5 to deposit\
    \n\t6 to withdraw from account\n\t7 to transfer\n\t8 to terminate application");

    io::stdin().read_line(&mut user_command); // read input from keyboard
    let user_command: u32 = user_command.trim().parse().expect("could not convert"); //convert the input to integer value
    match user_command{
        1 => create_account(&mut acm),
        2 => show_account_list(&mut acm),
        3 => update_account(&mut acm),
        4 => delete_account(&mut acm),
        5 => deposit_account(&mut acm),
        6 => withdraw_account(&mut acm),
        7 => transfer(&mut acm),
        _ => println!("Invalid command")
    }


}

//restart the process to ask users their command
fn app_continues(mut acm: &mut Manager){
    let mut user_command = String::new();
    println!("========================================\n========================================\n
    \tPlease enter: \n\t1 to create a new account \n\t2 to show all list of accounts \n\t3 to update account. \n\t4  to delete account. \n\t 5 to deposit\
    \n\t6 to withdraw from account\n\t7 to transfer\n\t8 to terminate application");
    io::stdin().read_line(&mut user_command);
    let user_command: u32 = user_command.trim().parse().expect("could not convert");
    match user_command{
        1 => create_account(&mut acm),
        2 => show_account_list(&mut acm),
        3 => update_account(&mut acm),
        4 => delete_account(&mut acm),
        5 => deposit_account(&mut acm),
        6 => withdraw_account(&mut acm),
        7 => transfer(&mut acm),
        _ => println!("Invalid command")
    }
}

//create an account
fn create_account(acm: &mut Manager){
    println!("You have chosen to create a new account");
    let mut fullname: String = String::new();
    let mut email: String = String::new();
    println!("Enter User full name:");
    io::stdin().read_line(&mut fullname);

    println!("Please Enter User Email:");
    io::stdin().read_line(&mut email);


    acm.register_account(&fullname,&email);
    println!("The account has been successfully registered.");
    app_continues(acm);
}

//update the bank account details
fn update_account(acm: &mut Manager){
    println!("Enter account number you want to edit:");
    let mut acc_id: String = String::new();
    io::stdin().read_line(&mut acc_id);
    let acc_id: u32 = acc_id.trim().parse().expect("Invalid input");
    println!("Enter new email address: press enter if you dont want to edit.");
    let mut email = String::new();
    io::stdin().read_line(&mut email);

    println!("Enter new user full name: press enter if you dont want to edit.");
    let mut name = String::new();
    io::stdin().read_line(&mut name);

    acm.update_account(&acc_id,&name,&email);
    app_continues(acm);


}

//deposit to bank account
fn deposit_account(acm: &mut Manager){
    println!("Enter account number you want to deposit to:");
    let mut acc_id: String = String::new();
    io::stdin().read_line(&mut acc_id);
    let acc_id: u32 = acc_id.trim().parse().expect("Invalid input");
    println!("Enter the amount you want to deposit.");
    let mut amount = String::new();
    io::stdin().read_line(&mut amount);

    acm.deposit(&acc_id, (&amount).parse().unwrap());
    app_continues(acm);


}
//withdraw from bank account
fn withdraw_account(acm: &mut Manager){
    println!("Enter account number you want to withdraw from:");
    let mut acc_id: String = String::new();
    io::stdin().read_line(&mut acc_id);
    let acc_id: u32 = acc_id.trim().parse().expect("Invalid input");
    println!("Enter the amount you want to withdraw.");
    let mut amount = String::new();
    io::stdin().read_line(&mut amount);
    acm.withdraw(&acc_id, (&amount).parse().unwrap());
    app_continues(acm);
}
//transfer between bank accounts
fn transfer(acm: &mut Manager){
    println!("Enter account number you want to transfer from:");
    let mut acc_id_from: String = String::new();
    io::stdin().read_line(&mut acc_id_from);
    let acc_id_from: u32 = acc_id_from.trim().parse().expect("Invalid input");

    //Request for the account to transfer to:
    println!("Enter the receiver account number:");
    let mut acc_id_to: String = String::new();
    io::stdin().read_line(&mut acc_id_to);
    let acc_id_to: u32 = acc_id_to.trim().parse().expect("Invalid input");

    println!("Enter the amount you want to transfer.");
    let mut amount = String::new();
    io::stdin().read_line(&mut amount);

    if acm.transfer(&acc_id_from,&acc_id_to, (&amount).parse().unwrap()){
        println!("{} was successfully transfered from {} to {}",amount, acc_id_from, acc_id_to)
    }else{
        println!("The transaction was not successful.")
    }
    app_continues(acm);
}

fn delete_account(acm: &mut Manager){
    println!("Please enter account number you want to delete");
    let mut acc_id: String = String::new();
    io::stdin().read_line(&mut acc_id);
    let acc_id: u32 = acc_id.trim().parse().expect("Invalid input");
    if acm.remove_account_records(&acc_id) {
        println!("Account: {} has been deleted successfully",acc_id);

    }else{
    println!("Account number is not registered with us");
    }
    app_continues(acm);
}


fn show_account_list(acm: &mut Manager){
    acm.list_all_accounts();
    app_continues(acm);
}