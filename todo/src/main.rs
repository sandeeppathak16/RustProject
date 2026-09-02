use std::io;
use todo::todo_manager::TodoManager;
use todo::cli::{create_todo, edit_todo};


fn main() {
    let manager = TodoManager::new("todos.json");

    loop {
        println!();
        println!("====================");
        println!("TODO APPLICATION");
        println!("====================");
        println!("1. View Todos");
        println!("2. Add Todo");
        println!("3. Edit Todo");
        println!("4. Delete Todo");
        println!("5. Mark Todo Done");
        println!("6. Exit");
        println!();

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read input");

        let option = match option.trim().parse::<u32>() {
            Ok(option) => option,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match option {
            1 => {
                manager.view_all();
            }

            2 => {
                match create_todo() {
                    Some(todo) => {
                        manager.add(todo);
                        println!("Todo added successfully");
                    }
                    None => {
                        println!("Failed to create todo");
                    }
                }
            }

            3 => {
                manager.view_all();

                println!("Enter Todo ID:");

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let id = match input.trim().parse::<u64>() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Invalid ID");
                        continue;
                    }
                };

                match manager.get_by_id(id) {
                    Some(mut todo) => {
                        edit_todo(&mut todo);

                        if manager.update(todo) {
                            println!("Todo updated successfully");
                        } else {
                            println!("Failed to update todo");
                        }
                    }
                    None => {
                        println!("Todo not found");
                    }
                }
            }

            4 => {
                manager.view_all();

                println!("Enter Todo ID to delete:");

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let id = match input.trim().parse::<u64>() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Invalid ID");
                        continue;
                    }
                };

                if manager.delete(id) {
                    println!("Todo deleted successfully");
                } else {
                    println!("Todo not found");
                }
            }

            5 => {
                manager.view_all();

                println!("Enter Todo ID to mark done:");

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let id = match input.trim().parse::<u64>() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Invalid ID");
                        continue;
                    }
                };

                if manager.mark_done(id) {
                    println!("Todo marked as done");
                } else {
                    println!("Todo not found");
                }
            }

            6 => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid option");
            }
        }
    }
}