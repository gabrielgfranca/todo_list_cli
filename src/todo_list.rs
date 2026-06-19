pub mod todo_list {
    use std::fs;

    use serde::{
        Deserialize,
        Serialize
    };

    use crate::{
        task::task::{
            Task,
            Status,
        },

    };

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TodoList {
        tasks: Vec<Task>,
    }

    const TASKS_FILE_PATH: &str = "tasks.json";

    impl TodoList {

        pub fn new() -> Self {
            Self {
                tasks: Vec::new()
            }
        }

        pub fn load() -> Self {
            match fs::read_to_string(self::TASKS_FILE_PATH) {
                Ok(content) => {
                    serde_json::from_str(&content)
                        .unwrap_or_else(|_| Self::new())
                }

                Err(_) => Self::new(),
            }
        }

        pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
            let json = serde_json::to_string_pretty(self)?;

            fs::write(TASKS_FILE_PATH, json)?;

            Ok(())
        }

        fn generate_id(&self) -> u32 {
            self.tasks
                .iter()
                .map(|task| task.id)
                .max()
                .unwrap_or(0)
                + 1
        }

        pub fn create_task(&mut self, description: String) {
            let task = Task {
                id: self.generate_id(),
                description,
                status: Status::Pending,
            };

            self.tasks.push(task)
        }

        pub fn list_all_tasks(&self) {
            if self.tasks.is_empty() {
                println!("No task found.");
                return;
            }

            for task in &self.tasks {
                println!("{task}");
            }
        }

        pub fn done_task(&mut self, task_id: u32) -> Result<(), &'static str> {
            match self.find_task_mut(task_id) {
                Some(task) => {
                    task.status = Status::Completed;
                    Ok(())
                }
                None => return Err("Task not found"),
            }
        }

        pub fn undone_task(&mut self, task_id: u32) -> Result<(), &'static str> {
            match self.find_task_mut(task_id) {
                Some(task) => {
                    task.status = Status::Pending;
                    Ok(())
                }
                None => return Err("Task not found"),
            }
        }

        pub fn remove_task(&mut self, task_id: u32) -> Result<(), &'static str>{
            match self.find_task_index(task_id) {
                Some(index) => {
                    self.tasks.remove(index);
                    Ok(())
                }
                None => return Err("Task not found"),
            }
        }

        pub fn find_task(&self, task_id: u32) -> Option<&Task> {
            self.tasks
                .iter()
                .find(|task| task.id == task_id)
        }

        pub fn find_task_mut(&mut self, task_id: u32) -> Option<&mut Task> {
            self.tasks
                .iter_mut()
                .find(|task| task.id == task_id)
        }

        pub fn find_task_index(&self, task_id: u32) -> Option<usize> {
            self.tasks
                .iter()
                .position(|task| task.id == task_id)
        }
    }
}
