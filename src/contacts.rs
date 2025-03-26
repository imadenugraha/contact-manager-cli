use std::{collections::VecDeque, fmt, io};

pub enum ContactType {
    Work,
    Home,
    Other
}

pub struct Contact {
    name: String,
    phone: String,
    contact_type: ContactType
}

impl fmt::Display for ContactType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self {
            ContactType::Home => "Home",
            ContactType::Work => "Work",
            ContactType::Other => "Other",
        };
        write!(f, "{}", type_str)
    }
}

impl Contact {
    pub fn add(contacts: &mut VecDeque<Self>, name: String, phone: String, contact_type: ContactType) {
        contacts.push_back(Self { name, phone, contact_type })
    }

    pub fn search(contacts: &VecDeque<Self>, name: String) {
        match contacts.iter().find(|contact| contact.name.to_lowercase() == name.to_lowercase()) {
            Some(contact) => {
                println!("Contact ditemukan!\n");
                println!("{} - {} [{}]", contact.name, contact.phone, contact.contact_type);
            },
            None => {
                println!("Contact tidak ditemukan!\n");
                return;
            }
        };
    }

    pub fn delete(contacts: &mut VecDeque<Self>, name: String) {
        match contacts.iter_mut().position(|contact| contact.name.to_lowercase() == name.to_lowercase()) {
            Some(contact) => {
                contacts.swap_remove_back(contact);
                println!("Contact berhasil dihapus!\n");
            },
            _ => {
                println!("Contact tidak ditemukan!\n");
                return;
            }
        };
    }
}

pub fn add_contact(contacts: &mut VecDeque<Contact>) {
    println!("Contact Name: ");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("Cannot read input!");
    let name: String = name.trim().to_string();

    println!("Phone Number: ");
    let mut phone: String = String::new();
    io::stdin().read_line(&mut phone).expect("Cannot read input!");
    let phone: String = phone.trim().to_string();

    println!("Contact Type => (1) Work (2) Home (3) Other: ");
    let mut contact_type: String = String::new();
    io::stdin().read_line(&mut contact_type).expect("Cannot read input!");
    let contact_type: String = contact_type.trim().to_string();

    match contact_type.trim() {
        "1" => Contact::add(contacts, name, phone, ContactType::Work),
        "2" => Contact::add(contacts, name, phone, ContactType::Home),
        "3" => Contact::add(contacts, name, phone, ContactType::Other),
        _ => {
            println!("Invalid contact type!");
            return;
        }
    };

    println!("Contact berhasil ditambahkan!\n");
}

pub fn list_contact(contacts: &VecDeque<Contact>) {
    if contacts.is_empty() {
        println!("Contact empty!");
        return;
    }

    println!("Daftar Contact: ");
    for (num, contact) in contacts.iter().enumerate() {
        println!("{}. {} - {} [{}]", num + 1, contact.name, contact.phone, contact.contact_type);
    };
}

pub fn search_contact(contacts: &VecDeque<Contact>) {
    println!("Masukkan Nama: ");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("Cannot read input!");

    Contact::search(contacts, name);
}

pub fn delete_contact(contacts: &mut VecDeque<Contact>) {
    println!("Masukkan Nama: ");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("Cannot read input!");

    Contact::delete(contacts, name);
}
