use crate::database::create_data;
use crate::todo_list::TodoList;
use std::io;

mod database;
mod todo_item;
mod todo_list;

fn main() {
    create_data().expect("Look like something miss");

    let mut todo_list = TodoList::new();

    loop {
        println!("1. Add Item");
        println!("2. List Items");
        println!("3. Complete Item");
        println!("4. Remove Item");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed read line");
        let choice = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter title item");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed read line");
                let id = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                todo_list.add_item(id).expect("Something error");
            }

            2 => todo_list.list_items().expect("Kerusakan"),

            3 => {
                println!("Enter ID to complete item");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed read line");
                let id = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                todo_list.complete_items(id).expect("Error berat")
            }

            4 => {
                println!("Enter ID to remove item");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed read line");
                let id = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                todo_list.remove_items(id).expect("ERROR WHILE DELETING")
            }

            5 => {
                println!("Exit");
                break;
            }

            _ => println!("Invalid choice"),
        }
    }
}
