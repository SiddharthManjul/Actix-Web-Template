use actix_cors::Cors;

use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};

use serde::{Deserialize, Serialize};

use reqwest::Client as HttpClient;

use async_trait::async_trait;

use core::task;
use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>
}

impl Database {
    fn new() -> Self {
        Self { 
            tasks: HashMap::new(), 
            users: HashMap::new() 
        }
    }

    // CRUD DATA RELATED FUNCTIONS
    fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn get(&self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }

    fn get_all_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn delete(&mut self, id: &u64) {
        self.tasks.remove(id);
    }

    fn update(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    // USER DATA RELATED FUNCTIONS
    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    // DATABASE SAVING
    fn save_to_file(&self) -> std::io::Result<()> {
        let data: String = serde_json::to_string(&self)?;
        let mut file: fs::File = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load_from_data() -> std::io::Result<Self> {
        let file_content = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }

}

fn main() {
    println!("Hello, world!");
}
