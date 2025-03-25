mod contacts;

use std::io;
use contacts::Contact;

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    println!("Contact Manager");
    println!("1. Buat Contact Baru");
    println!("2. List Contact");
    println!("3. Cari Kontak");
    println!("4. Hapus Kontak");
    println!("0. Keluar");

    loop {
        println!("Pilih: ");
        let mut menu: String = String::new();
        io::stdin().read_line(&mut menu).expect("Menu tidak diketahui!");
        println!("");
        
        match menu.trim() {
            "1" => contacts::add_contact(&mut contacts),
            "2" => contacts::list_contact(&contacts),
            "3" => contacts::search_contact(&contacts),
            "4" => contacts::delete_contact(&mut contacts),
            "0" => {
                println!("Exiting...");
                break;
            },
            _ => {
                println!("Menu tidak diketahui!");
            }
        }
    }
}