use crate::todo_item::TodoItem;
use rusqlite::{params, Connection, Result};

pub struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }

    pub fn add_item(&mut self, title: String) -> Result<()> {
        let id = self.items.len() as u64 + 2;
        let conn = Connection::open("projek1.sqlite")?;
        let new_item = TodoItem {
            id,
            title: title.clone(),
            done: false,
        };

        self.items.push(new_item);
        conn.execute(
            "INSERT INTO items (id, title, done) VALUES (?1, ?2, ?3)",
            params![id, title, false],
        )?;
        println!("New item todo: {}", title);
        Ok(())
    }

    pub fn list_items(&self) -> Result<()> {
        let conn = Connection::open("projek1.sqlite")?;
        let mut stmt = conn.prepare("SELECT id, title, done FROM items")?;

        if !stmt.exists(())? {
            println!("No items found");
            return Ok(());
        }
        let item_iter = stmt.query_map([], |row| {
            Ok(TodoItem {
                id: row.get(0)?,
                title: row.get(1)?,
                done: row.get(2)?,
            })
        })?;

        println!("Your Todo-list");
        for item in item_iter.into_iter().filter_map(Result::ok) {
            let status = if item.done { "[X]" } else { "[ ]" };
            println!("{} {} - {}", status, item.title, item.id);
        }

        Ok(())

        // if self.items.is_empty() {
        //     println!("empty")
        // } else {
        //     println!("Your");
        //     for item in &self.items {
        //         let status = if item.done { "[X]" } else { "[]" };
        //         println!("{} {} - {}", status, item.id, item.title);
        //     }
        // }
    }

    pub fn complete_items(&mut self, id: u64) -> Result<()> {
        let conn = Connection::open("projek1.sqlite")?;

        conn.execute("UPDATE items SET done = true WHERE id = ?1", params![id])?;

        println!("Completed item {}", id);

        Ok(())

        // if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
        //     item.done = true;
        //     println!("Complete item done");
        // } else {
        //     println!("No item {} to complete", id);
        // }
    }

    pub fn remove_items(&mut self, id: u64) -> Result<()> {
        let conn = Connection::open("projek1.sqlite")?;

        conn.execute("DELETE FROM items WHERE id = ?1", params![id])?;

        println!("Removed item {}", id);

        Ok(())

        // if let Some(item) = self.items.iter().position(|item| item.id == id) {
        //     self.items.remove(item);
        //     println!("Item successfully removed");
        // } else {
        //     println!("No item {} to remove", id);
        // }
    }
}
