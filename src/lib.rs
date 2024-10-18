// Find all our documentation at https://docs.near.org
use near_sdk::{log, near};

type TodoId = u64;

// Define the contract structure
#[derive(near_sdk::PanicOnDefault)]
#[near(contract_state)]
pub struct Contract {
    todos: std::collections::HashMap<TodoId, Todo>,
    next_todo_id: TodoId,
}

#[derive(Clone)]
#[near(serializers = ["json", "borsh"])]
pub enum TodoStatus {
    Pending,
    Completed,
}

#[derive(Clone)]
#[near(serializers = ["json", "borsh"])]
pub struct Todo {
    text: String,
    status: TodoStatus,
}

#[near(serializers = ["json"])]
pub struct TodoWithId {
    todo_id: TodoId,
    #[serde(flatten)]
    todo: Todo,
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            todos: std::collections::HashMap::new(),
            next_todo_id: 0,
        }
    }

    /// Add new todo
    pub fn add_todo(&mut self, text: String) -> TodoId {
        let todo = Todo {
            text,
            status: TodoStatus::Pending,
        };
        let todo_id = self.next_todo_id;
        self.todos.insert(todo_id, todo);
        self.next_todo_id += 1;
        todo_id
    }

    /// List all todos
    pub fn list_todos(&self) -> Vec<TodoWithId> {
        self.todos
            .iter()
            .map(|(todo_id, todo)| TodoWithId {
                todo_id: *todo_id,
                todo: todo.clone(),
            })
            .collect()
    }

    /// Change todo status
    pub fn complete_todo_status(&mut self, todo_id: TodoId, status: TodoStatus) {
        if let Some(todo) = self.todos.get_mut(&todo_id) {
            todo.status = status;
        }
    }

    /// Delete todo
    pub fn delete_todo(&mut self, todo_id: TodoId) {
        self.todos.remove(&todo_id);
    }

    /// Edit todo
    pub fn edit_todo_text(&mut self, todo_id: TodoId, text: String) {
        let Some(todo) = self.todos.get_mut(&todo_id) else {
            near_sdk::env::panic_str("Todo not found");
        };

        todo.text = text;
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_new_todo() {
        let mut contract = Contract::new();
        let new_todo_id = contract.add_todo("Write todo app".to_string());
        insta::assert_json_snapshot!(contract.list_todos());
    }
}
