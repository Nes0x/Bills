use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Bill {
    name: String,
    amount: i32,
}
struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bill {
    fn new(name: String, amount: i32) -> Self {
        Self { name: name.trim().to_owned(), amount }
    }
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) -> Result<String, String> {
        if self.inner.contains_key(&bill.name) {
            Err("Nie możesz dodać rachunku który już istnieje!".to_owned())
        } else {
            self.inner.insert(bill.name.clone(), bill);
            Ok("Pomyślnie dodałeś rachunek!".to_owned())
        }
    }


    fn search_by_name(&self, name: &str) -> Option<&Bill> {
        self.inner.get(name.trim())
    }

    fn find_all(&self) -> Vec<&Bill> {
        let mut bills = Vec::new();

        for bill in self.inner.values() {
            bills.push(bill)
        }

        bills
    }

    fn remove_bill(&mut self, name: &str) -> Result<String, String> {
        if self.inner.contains_key(name.trim()) {
            self.inner.remove(name.trim());
            Ok("Pomyślnie usunięto rachunek.".to_owned())
        } else {
            Err("Taki rachunek nie istnieje!".to_owned())
        }
    }


    fn update_bill(&mut self, name: &str, new_name: &str, new_amount: i32) -> Result<String, String> {
        if self.inner.contains_key(name.trim()) {
            self.remove_bill(name);
            self.add(Bill::new(new_name.to_owned(), new_amount));
            Ok("Pomyślnie zedytowałeś rachunek!".to_owned())
        } else {
            Err("Taki rachunek nie istnieje!".to_owned())
        }
    }

}

fn get_input() -> String {
    let mut typed = String::new();
    let input = io::stdin().read_line(&mut typed);

    match input {
        Ok(_) => typed,
        Err(reason) => reason.to_string().to_owned(),
    }
}

fn add_bill(bills: &mut Bills) {
    println!("Wpisz nazwe rachunku:");
    let name = get_input();
    println!("Wpisz kwote:");
    let amount = get_input()
        .trim()
        .parse()
        .expect("Nie podałeś liczby, żegnam!");
    bills.add(Bill::new(name.to_owned(), amount));
}

fn search_bill(bills: &Bills) {
    println!("Wpisz nazwe rachunku lub wpisz 'all' aby pokazać wszystkie:");
    let name = get_input();

    if name.trim() == "all" {
        for bill in bills.find_all() {
            println!("{:?}", bill);
        }
    } else {
        let result = bills.search_by_name(&name);

        match result {
            None => println!("Taki rachunek nie istnieje!"),
            Some(value) => println!("Nazwa: {}, Do zapłaty: {}", value.name.trim(), value.amount),
        }
    }
}


fn remove_bill(bills: &mut Bills) {
    println!("Wpisz nazwe rachunku którego chcesz usunąć:");
    let name = get_input();

    let result = bills.remove_bill(&name);

    match result {
        Ok(result) => println!("{}", result),
        Err(reason) => println!("{}", reason),
    }

}

fn update_bill(bills: &mut Bills) {
    println!("Aktualnie dostępne rachunki:");
    let all_bills = bills.find_all();
    for bill in all_bills {
        println!("{:?}", bill);
    }

    println!("Wpisz nazwe rachunku którego chcesz zedytować:");
    let name = get_input();
    println!("Wpisz nową nazwe:");
    let new_name = get_input();
    println!("Wpisz nową kwotę:");
    let new_amount = get_input()
        .trim()
        .parse()
        .expect("Nie podałeś liczby, żegnam!");

    let result = bills.update_bill(&name, &new_name, new_amount);


    match result {
        Ok(result) => println!("{}", result),
        Err(reason) => println!("{}", reason),
    }
}


fn main() {
    let mut bills = Bills::new();
    println!("Witaj w zarządzaniu rachunkami!\n1. Dodaj rachunek.\n2. Wyszukaj rachunek.\
    \n3. Usuń rachunek.\n4. Edytuj rachunek.\n5. Zakończ sesje.");

    loop {
        println!("Wpisz co chcesz wybrać:");
        let result = get_input();

        match result.trim() {
            "1" => add_bill(&mut bills),
            "2" => search_bill(&bills),
            "3" => remove_bill(&mut bills),
            "4" => update_bill(&mut bills),
            "5" => {
                println!("Żegnaj!");
                break
            }
            ,
            &_ => println!(
                "Nie wybrałeś poprawnej cyfry lub wystąpił błąd! {}",
                result.trim()
            ),
        }
    }
}
