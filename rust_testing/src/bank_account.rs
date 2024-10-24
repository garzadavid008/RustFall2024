#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount{
balance: initial_balance,
        }
        
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0
        {
        self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount < self.balance && amount > 0.0
        {
        self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance,100.0); // testing if it matches 100.0
        let account2 = BankAccount::new(5050.55);
        assert_eq!(account2.balance,5050.55);
    }

    #[test]
    fn test_deposit() {
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0); // balance is 150.0 
        assert_eq!(account.balance,150.0);
        let mut account2 = BankAccount::new(200.0);
        account2.deposit(-50.0); 
        assert_eq!(account2.balance,200.0);

    }

    #[test]
    fn test_withdraw() {
        let mut account3 = BankAccount::new(500.0);
        account3.withdraw(1000.0);
        assert_eq!(account3.balance,500.0);
        let mut account4 = BankAccount::new(500.0);
        account4.withdraw(-30.0); 
        assert_eq!(account4.balance,500.0);
    }

    
}
    