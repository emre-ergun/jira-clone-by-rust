mod models;

use models::{Epic, Status};

fn main() {
    println!("Welcome To My-Jira!");

    let mut epic = Epic::new("Epic-1".to_owned(), "New Epic created for test".to_owned());

    epic.status = Status::InProgress;
    epic.status = Status::Closed;
    epic.status = Status::Resolved;
    
    println!("Title: '{}'\nDescription: '{}'", epic.name, epic.description);
}