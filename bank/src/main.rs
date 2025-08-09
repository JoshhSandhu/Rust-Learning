
#[derive(Debug)]
struct Bank{
    accounts: Vec<Account>,
} 
impl Bank{
    fn new() -> Self {
        Bank { accounts: vec![]}
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
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn change_bank(account: &mut Account){
    account.balance = 10;
}

fn make_and_print_account(){
    let account = Account::new(1, String::from("hello"));

    println!("{:#?}", account);
}

fn main() {
    let bank = Bank::new();
    let num = 5;
    let other_num = num;
    let mut account = Account::new(1, String::from("josh"));

    println!("{:#?}", bank);

    // let account_ref1 = &account;
    // let account_ref2 = &account;

    change_bank(&mut account);
    println!("{:#?}", account);
    println!("{} {}", num, other_num);
    // print_account(account_ref1);
    // print_account(account_ref2);
    
    //Lifetimes
    make_and_print_account();
}
