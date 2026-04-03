struct Account {
    owner: String,
    balance: i32,
    active: bool,
}

struct AccountRegistry {
    accounts: Vec<Account>,
}

impl Account {
    fn new(owner: &str) -> Account {
        Account {
            owner: owner.to_string(),
            balance: 0,
            active: true,
        }
    }

    fn withdraw(&mut self, amount: i32) {
        self.balance -= amount;
    }

    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    fn is_active(&self) -> bool { self.active }
}

impl AccountRegistry {
    fn new() -> AccountRegistry {
        AccountRegistry {
            accounts: Vec::new(),
        }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn list_account(&self) {
        for account in self.accounts.iter() {
            println!("
                Owner: {}, Balance: {}, Active: {}",
                account.owner, account.balance, account.active
            );
        }
    }

    fn total_sum_all_account(&self) -> i32 {
        let mut total = 0;
        for account in self.accounts.iter() {
            total += account.balance;
        }
        total
    }
}


fn main() {
    let mut registery = AccountRegistry::new();

    let mut acc1 = Account::new("ruby");
    acc1.deposit(1000);

    let mut acc2 = Account::new("rusty");
    acc2.deposit(2000);

    let mut acc3 = Account::new("boby");
    acc3.deposit(800);


    registery.add_account(acc1);
    registery.add_account(acc2);
    registery.add_account(acc3);

    registery.list_account();

    let total = registery.total_sum_all_account();
    println!("Somme total des balances: {}", total);
}
