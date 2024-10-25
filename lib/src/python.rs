use pyo3::{exceptions::PyValueError, prelude::*};

#[derive(Clone)]
#[pyclass]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[pymethods]
impl Todo {
    #[new]
    fn new(id: i32, title: String) -> Self {
        Todo {
            id,
            title,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }

    fn __repr__(&self) -> String {
        format!(
            "Todo {{ id: {}, title: \"{}\", completed: {} }}",
            self.id, self.title, self.completed
        )
    }
}

#[pyclass]
struct Todos {
    todos: Vec<Todo>,
}

#[pymethods]
impl Todos {
    #[new]
    fn new() -> Self {
        Self { todos: Vec::new() }
    }

    fn add(&mut self, id: i32, title: String) {
        self.todos.push(Todo::new(id, title));
    }

    fn complete(&mut self, id: i32) -> PyResult<()> {
        self.todos
            .iter_mut()
            .find(|todo| todo.id == id)
            .ok_or_else(|| PyValueError::new_err("Todo not found"))?
            .complete();
        Ok(())
    }

    fn remove(&mut self, id: i32) -> PyResult<()> {
        let Some(pos) = self.todos.iter().position(|todo| todo.id == id) else {
            return Err(PyValueError::new_err("Todo not found"));
        };
        self.todos.remove(pos);
        Ok(())
    }

    fn list(&self) -> PyResult<Vec<Todo>> {
        Ok(self.todos.clone())
    }

    fn get(&self, id: i32) -> PyResult<Option<Todo>> {
        if let Some(todo) = self.todos.iter().find(|todo| todo.id == id) {
            Ok(Some(todo.clone()))
        } else {
            Ok(None)
        }
    }
}

#[pymodule]
fn core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Todos>()?;
    m.add_class::<Todo>()
}
