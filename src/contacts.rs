use std::io;

#[derive(Debug)]
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

impl Contact {
    pub fn add(contacts: &mut Vec<Self>, name: String, phone: String, contact_type: ContactType) {
        contacts.push(Self { name, phone, contact_type })
    }

    pub fn search(contacts: &[Self], name: String) {
        let _ = match contacts.iter().find(|contact| contact.name.to_lowercase() == name.to_lowercase()) {
            Some(contact) => {
                println!("Contact ditemukan!\n");
                println!("{} - {} [{:?}]", contact.name, contact.phone, contact.contact_type);
            },
            None => {
                println!("Contact tidak ditemukan!\n");
                return;
            }
        };
    }

    pub fn delete(contacts: &mut Vec<Self>, name: String) {
        let _ = match contacts.iter_mut().position(|contact| contact.name.to_lowercase() == name.to_lowercase()) {
            Some(contact) => {
                contacts.remove(contact);
                println!("Contact berhasil dihapus!\n");
            },
            _ => {
                println!("Contact tidak ditemukan!\n");
                return;
            }
        };
    }
}

pub fn add_contact(contacts: &mut Vec<Contact>) {
    println!("Contact Name: ");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("Cannot read input!");

    println!("Phone Number: ");
    let mut phone: String = String::new();
    io::stdin().read_line(&mut phone).expect("Cannot read input!");

    println!("Contact Type => (1) Work (2) Home (3) Other: ");
    let mut contact_type: String = String::new();
    io::stdin().read_line(&mut contact_type).expect("Cannot read input!");

    match contact_type.trim() {
        "1" => Contact::add(contacts, name, phone, ContactType::Home),
        "2" => Contact::add(contacts, name, phone, ContactType::Work),
        "3" => Contact::add(contacts, name, phone, ContactType::Other),
        _ => {
            println!("Invalid contact type!");
            return;
        }
    };

    println!("Contact berhasil ditambahkan!\n");
}

pub fn list_contact(contacts: &[Contact]) {
    println!("Daftar Contact: ");
    for (num, contact) in contacts.iter().enumerate() {
        println!("{}. {} - {} [{:?}]", num + 1, contact.name, contact.phone, contact.contact_type);
    };
}

pub fn search_contact(contacts: &Vec<Contact>) {
    println!("Masukkan Nama: ");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("Cannot read input!");

    Contact::search(contacts, name);
}

pub fn delete_contact(contacts: &mut Vec<Contact>) {
    println!("Masukkan Nama: ");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("Cannot read input!");

    Contact::delete(contacts, name);
}
