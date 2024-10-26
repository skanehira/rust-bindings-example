#[derive(Clone)]
pub(crate) struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: i32, title: String) -> Self {
        Todo {
            id,
            title,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

#[derive(Default, Clone)]
pub(crate) struct Todos(Vec<Todo>);

impl Todos {
    pub fn get(&self, id: i32) -> Option<&Todo> {
        self.0.iter().find(|todo| todo.id == id)
    }

    pub fn add(&mut self, id: i32, title: String) {
        self.0.push(Todo::new(id, title));
    }

    pub fn complete(&mut self, id: i32) -> Result<(), &'static str> {
        self.0
            .iter_mut()
            .find(|todo| todo.id == id)
            .ok_or("todo not found")?
            .complete();
        Ok(())
    }

    pub fn remove(&mut self, id: i32) -> Result<(), &'static str> {
        let Some(pos) = self.0.iter().position(|todo| todo.id == id) else {
            return Err("todo not found");
        };
        self.0.remove(pos);
        Ok(())
    }

    pub fn list(&self) -> &[Todo] {
        self.0.as_slice()
    }
}
