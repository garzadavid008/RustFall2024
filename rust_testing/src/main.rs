mod bank_account;

use bank_account :: BankAccount;

fn main() {

    let mut account1= BankAccount::new(500.0);
    let mut bank_account_2 = BankAccount::new(100.0);
    println!("The balance for account1 is {}, and the balance for account 2 is : {}", account1.balance(),bank_account_2.balance());
    println!("Account 1 withdrew $50.0, Account 2 withdrew $150.0");
    account1.withdraw(50.0);
    bank_account_2.withdraw(150.0);
    println!("New balance for Account 1: {}, New balance for Account 2: {}", account1.balance(), bank_account_2.balance());
    println!("Account 1 deposited $150.0, Account 2 deposited -75.0");
    account1.deposit(150.0);
    bank_account_2.deposit(-75.0);
    println!("New balance for Account 1: {}, New balance for Account 2: {}",account1.balance(),bank_account_2.balance());
}