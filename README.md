# Task Manager CLI

## 📌 Overview
Task Manager CLI is a simple command-line task management application built using **Rust Workspaces**. It follows a modular structure with multiple crates for **task logic, storage handling, and CLI interactions**.

## 📂 Project Structure
```
task_manager/
│── Cargo.toml  # Root workspace definition
├── cli/        # Binary crate (Main CLI app)
│   ├── Cargo.toml
│   ├── src/main.rs
├── core/       # Library crate (Task management logic)
│   ├── Cargo.toml
│   ├── src/lib.rs
├── storage/    # Library crate (Handles file/database storage)
│   ├── Cargo.toml
│   ├── src/lib.rs
```

## 🚀 Features
- **Add tasks** with unique IDs and titles.
- **List all tasks** (completed & pending).
- **Mark tasks as done**.
- **Persistent storage** using JSON.

## 🛠 Setup & Installation
### **1. Clone the repository**
```sh
git clone https://github.com/your-username/task-manager-cli.git
cd task-manager-cli
```

### **2. Build the project**
```sh
cargo build
```

### **3. Run the CLI**
```sh
cargo run -p cli
```

## 🖥 Usage
Upon running the CLI, you will see the following menu:
```sh
Task Manager CLI
1. Add Task
2. List Tasks
3. Mark Task Done
4. Exit
Choose an option:
```

### **Adding a Task**
```sh
Enter task title: Buy groceries
```

### **Listing Tasks**
```sh
[1] Buy groceries - ❌ Pending
```

### **Marking a Task as Done**
```sh
Enter task ID to mark done: 1
```
Output:
```sh
[1] Buy groceries - ✅ Done
```

## 📌 Crate Responsibilities
### **cli/** (Binary crate)
- Handles **user input & output**.
- Calls functions from `core` and `storage` crates.

### **core/** (Library crate)
- Defines **`Task` struct** and its logic.
- Implements methods for creating and modifying tasks.

### **storage/** (Library crate)
- Saves & loads tasks to/from **tasks.json**.
- Uses `serde_json` for serialization.

## 🏆 Future Enhancements
- ✅ Add support for command-line arguments.
- ✅ Replace JSON with an SQLite/PostgreSQL database.
- ✅ Convert CLI into a web service with Actix-Web.

## 🛠 Dependencies
This project uses the following Rust crates:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---
🔥 Built with Rust! 🚀

