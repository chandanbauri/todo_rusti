
struct Todo {
    title: String,
    status: bool,
}

fn main() {
    // Declare a mutable global vector to store todos
    let mut todo_list: Vec<Todo> = Vec::new();

    // Create a todo instance and return it
    let create_todo = |title: String| -> Todo {
        Todo {
            title,
            status: false,
        }
    };

    // Add a todo instance to the todo list
    fn add_todo (todo_list: &mut Vec<Todo>, todo: Todo) { 
        todo_list.push(todo);
    }

    fn update_todo(todo_list: &mut Vec<Todo>, index: usize, status: bool) {
        match todo_list.get_mut(index) {
            Some(todo) => { todo.status = status;}
            None =>  {println!("The todo does not exit")}
        }
    }

    fn remove_todo(todo_list: &mut Vec<Todo>, index: usize) {
        match todo_list.get(index) {
            Some(_) => {todo_list.remove(index); } 
            None =>  {println!("The todo does not exist");}
        }
    }

    let t1 = create_todo(format!("create todo list"));
    add_todo(&mut todo_list , t1);
    let t2 = create_todo(format!("create todo list"));
    add_todo(&mut todo_list,t2);

    update_todo(&mut todo_list, 1, true);
    remove_todo(&mut todo_list, 2);

    // Print the todos in the todo list
    for todo in &todo_list {
        println!("Todo: {} ({})", todo.title, todo.status);
    }

    println!("Number of todos: {}", todo_list.len());
}

