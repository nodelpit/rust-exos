struct Account {
    owner: String,
    balance: i32,
    active: bool,
}

impl Account {
    fn new(owner: &str) -> Account {
        Account {
            owner: owner.to_string(),
            balance: 0,
            active: true,
        }
    }

    fn deposit(&mut self, amount: i32) {
        if !self.active {
            return;
        } else {
            if amount >= 0 {
                self.balance += amount;
            }
        }
    }

    fn withdraw(&mut self, amount: i32) -> bool {
        if self.can_withdraw(amount) {
            self.balance -= amount;
            true
        } else {
            false
        }
    }

    fn can_withdraw(&self, amount: i32) -> bool {
        self.active && self.balance >= amount && amount > 0
    }

    fn is_active(&self) -> bool { self.active }

    fn close(&mut self) {
        self.active = false;
        self.balance = 0;
    }
}

fn main() {
    let mut new_account = Account::new("Noah");

    new_account.deposit(12);
    new_account.deposit(20);
    new_account.withdraw(2);
    println!("Bonjour {}, il vous reste {}€ sur votre compte", new_account.owner, new_account.balance);

    println!("Compte ouvert: {}", new_account.is_active());

    new_account.withdraw(30);
    println!("Bonjour {}, il vous reste {}€ sur votre compte", new_account.owner, new_account.balance);

    new_account.close();
    println!("Compte ouvert: {}", new_account.is_active());
}
