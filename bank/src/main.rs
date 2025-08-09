
#[derive(Debug)]
struct Bank{
    accounts: Vec<Account>,
} 
impl Bank{
    fn new() -> Self {
        Bank { accounts: vec![]}
    }

    fn add_account(&mut self, account: Account){
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32{
        self.accounts.iter()
        .map(|account|account.balance)
        .sum()
    }

    fn summery(&self) -> Vec<String>{
        self.accounts.iter()
        .map(|account|account.summery())
        .collect::<Vec<String>>()
    }
}

#[derive(Debug)]
struct Account{
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String ) -> Self {
        Account { id, holder, balance: 0, }
    }

    fn deposit(&mut self, amount:i32) -> i32{
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount:i32) -> i32{
        self.balance -= amount;
        self.balance
    }

    fn summery(&self) -> String {
        format!("{} has a balance of {}", self.holder, self.balance)
    }
}



// fn print_account(account: &Account) {
//     println!("{:#?}", account);
// }

// fn change_bank(account: &mut Account){
//     account.balance = 10;
// }

// fn make_and_print_account() -> &Account{
//     let account = Account::new(1, String::from("hello"));

//     println!("{:#?}", account);

//     &account
// }

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("josh"));

    account.deposit(200);
    account.withdraw(50);
    
    bank.add_account(account);

    println!("{:#?}", bank.summery());
    println!("{}", bank.total_balance());

    // let num = 5;
    // let other_num = num;
    // let account_ref1 = &account;
    // let account_ref2 = &account;

    //change_bank(&mut account);
    // println!("{:#?}", account);
    // println!("{} {}", num, other_num);
    // print_account(account_ref1);
    // print_account(account_ref2);
    
    //Lifetimes
    // let account_reference = make_and_print_account();
    // println!("{:#?}",account_reference.balance);
}
