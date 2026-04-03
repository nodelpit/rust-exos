struct BankAccount {
    owner: String,
    balance: i32,
    status: AccountStatus,
}

struct Bank {
    accounts: Vec<BankAccount>,
}

#[derive(Debug, PartialEq)]
enum AccountStatus {
    Active,
    Closed,
}

impl BankAccount {
    fn new(owner: &str) -> BankAccount {
        BankAccount {
            owner: owner.to_string(),
            balance: 0,
            status: AccountStatus::Active,
        }
    }

    fn deposit(&mut self, amount: i32) {
        match self.status {
            AccountStatus::Active => self.balance += amount,
            AccountStatus::Closed => println!("Vous ne pouvez pas déposer de l'arget, votre compte est fermé =("),
        };
    }

    fn withdraw(&mut self, amount: i32) -> bool {
        if self.status == AccountStatus::Active && self.balance >= amount {
            if amount > 0 { self.balance -= amount;
                true
            } else { false }
        } else {
            false
        }
    }

    fn is_active(&self) -> bool {
        self.status == AccountStatus::Active
    }

    fn close(&mut self) {
        self.status = AccountStatus::Closed;
        self.balance = 0;
    }
}

impl Bank {
    fn new() -> Bank {
        Bank { accounts: Vec::new() }
    }

    fn add_account(&mut self, account: BankAccount) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        let mut total = 0;
        for account in self.accounts.iter() {
            if account.status == AccountStatus::Active { total += account.balance } else {}
        }
        total
    }

    fn list_accounts(&self) {
        for (i, account) in self.accounts.iter().enumerate() {
            println!(
                "{} - Propriétaire: {}, balance: {}, état: {:?}",
                i + 1,
                account.owner,
                account.balance,
                account.status,
            );
        }
    }
}


fn main() {
    let mut bank = Bank::new();

    let mut acc1 = BankAccount::new("NNo04aHh");
    let mut acc2 = BankAccount::new("RuU6Y");
    let mut acc3 = BankAccount::new("RU5ti");

    acc1.deposit(1378);
    acc2.deposit(6152);
    acc3.deposit(1220);

    bank.add_account(acc1);
    bank.add_account(acc2);
    bank.add_account(acc3);

    println!("-----------------");
    bank.list_accounts();
    println!("-----------------");

    let total = bank.total_balance();
    println!("Total des soldes: {}", total);
}
