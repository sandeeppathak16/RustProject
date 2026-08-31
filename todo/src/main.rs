use std::io;
use todo::todo::ToDo;


fn create_todo() -> ToDo {
    let mut name = String::new();
    println!("Enter Name");
    io::stdin().read_line(&mut name).unwrap();

    let mut description = String::new();
    println!("Enter description (optional)");
    io::stdin().read_line(&mut description).unwrap();

    let mut day_remaining = String::new();
    println!("Enter days remaining (optional)");
    io::stdin().read_line(&mut day_remaining).unwrap();

    let description = if description.trim().is_empty() {
        None
    } else {
        Some(description.trim().to_string())
    };

    let due_in_days = if day_remaining.trim().is_empty() {
        None
    } else {
        match day_remaining.trim().parse::<i64>() {
            Ok(days) => Some(days),
            Err(_) => {
                println!("Please enter a valid number");
                return;
            }
        }
    };

    ToDo::new(
        name.trim().to_string(),
        description,
        due_in_days,
    )
}


fn add_todo(todo: Option<Todo>) {
    let todo = match todo {
        Some(todo) => todo,
        None => create_todo()
    }

    todo.view_todo();

    println!("let us know if we could add this file or not 1/0: \n");
    let mut can_add = String::new();

    match can_add.trim().parse::<u32>() {
        Ok(0) => add_todo(todo),
        Ok(1) => todo.add(),
        Ok(_) => {
            println!("Invalid option");
            add_todo(todo);
        }
        Err(_) => {
            println!("Please enter a number");
            add_todo(todo);
        }
    }
}


fn main() {
    println!("Enter Option\n");
    println!("1. View todos");
    println!("2. Add todos\n");

    let mut ops = String::new();
    io::stdin().read_line(&mut ops).unwrap();

    match ops.trim().parse::<u32>() {
        Ok(1) => add_todo(),
        Ok(2) => println!("Adding todo"),
        Ok(_) => println!("Invalid option"),
        Err(_) => println!("Please enter a number"),
    }
}
