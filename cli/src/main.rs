use core::Task;
use storage::{save_tasks, load_tasks};
use std::io::{self, Write};

fn main() {
    let mut tasks = load_tasks();

    println!("Task Manager CLI");
    println!("1. Add Task");
    println!("2. List Tasks");
    println!("3. Mark Task Done");
    println!("4. Exit");

    print!("Choose an option: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => {
            print!("Enter task title: ");
            io::stdout().flush().unwrap();
            let mut title = String::new();
            io::stdin().read_line(&mut title).unwrap();

            let new_task = Task::new(tasks.len() as u32 + 1, title.trim());
            tasks.push(new_task);
            save_tasks(&tasks);
        }
        "2" => {
            for task in &tasks {
                println!(
                    "[{}] {} - {}",
                    task.id,
                    task.title,
                    if task.completed { "✅ Done" } else { "❌ Pending" }
                );
            }
        }
        "3" => {
            print!("Enter task ID to mark done: ");
            io::stdout().flush().unwrap();
            let mut id_input = String::new();
            io::stdin().read_line(&mut id_input).unwrap();
            let id: u32 = id_input.trim().parse().unwrap_or(0);

            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.mark_done();
                save_tasks(&tasks);
            }
        }
        "4" => return,
        _ => println!("Invalid choice!"),
    }
}
