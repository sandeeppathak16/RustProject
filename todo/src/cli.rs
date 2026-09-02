use std::io;
use crate::todo::ToDo;

pub fn create_todo() -> Option<ToDo> {
    let mut name = String::new();

    println!("Enter Name:");
    io::stdin().read_line(&mut name).unwrap();

    let name = name.trim().to_string();

    if name.is_empty() {
        println!("Name cannot be empty");
        return None;
    }

    let mut description = String::new();

    println!("Enter Description (optional):");
    io::stdin().read_line(&mut description).unwrap();

    let description = if description.trim().is_empty() {
        None
    } else {
        Some(description.trim().to_string())
    };

    let mut days = String::new();

    println!("Enter Due Days (optional):");
    io::stdin().read_line(&mut days).unwrap();

    let due_in_days = if days.trim().is_empty() {
        None
    } else {
        match days.trim().parse::<i64>() {
            Ok(days) => Some(days),
            Err(_) => {
                println!("Invalid number");
                return None;
            }
        }
    };

    Some(ToDo::new(
        name,
        description,
        due_in_days,
    ))
}


pub fn edit_todo(todo: &mut ToDo) {
    let mut description = String::new();

    println!("New Description (leave empty to keep current):");
    io::stdin().read_line(&mut description).unwrap();

    let description = if description.trim().is_empty() {
        None
    } else {
        Some(description.trim().to_string())
    };

    let mut days = String::new();

    println!("New Due Days (leave empty to keep current):");
    io::stdin().read_line(&mut days).unwrap();

    let due_in_days = if days.trim().is_empty() {
        None
    } else {
        match days.trim().parse::<i64>() {
            Ok(days) => Some(days),
            Err(_) => {
                println!("Invalid number");
                return;
            }
        }
    };

    let mut done = String::new();

    println!("Mark Done? (T/F, leave empty to keep current):");
    io::stdin().read_line(&mut done).unwrap();

    let done = match done.trim() {
        "" => None,
        "T" | "t" => Some(true),
        "F" | "f" => Some(false),
        _ => {
            println!("Please enter T or F");
            return;
        }
    };

    todo.edit(
        description,
        done,
        due_in_days,
    );
}