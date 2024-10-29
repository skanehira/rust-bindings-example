use crate::{
    core::{Todo, Todos},
    enumurate::TodoStatus,
};
use pyo3::{exceptions::PyValueError, prelude::*};

#[derive(Clone)]
#[pyclass(name = "Todo")]
struct TodoClass(Todo);

impl From<&Todo> for TodoClass {
    fn from(todo: &Todo) -> Self {
        TodoClass(todo.clone())
    }
}

#[pymethods]
impl TodoClass {
    #[new]
    fn new(id: i32, title: String) -> Self {
        TodoClass(Todo::new(id, title))
    }

    fn complete(&mut self) {
        self.0.change_status(TodoStatus::Completed);
    }

    fn __repr__(&self) -> String {
        format!(
            "Todo {{ id: {}, title: \"{}\", status: {:?} }}",
            self.0.id, self.0.title, self.0.status
        )
    }
}

#[pyclass(name = "Todos")]
#[derive(Clone)]
struct TodosClass(Todos);

#[pymethods]
impl TodosClass {
    #[new]
    fn new() -> Self {
        Self(Todos::default())
    }

    fn add(&mut self, id: i32, title: String) {
        self.0.add(id, title);
    }

    fn complete(&mut self, id: i32) -> PyResult<()> {
        self.0
            .complete(id)
            .map_err(|_| PyValueError::new_err("Todo not found"))
    }

    fn remove(&mut self, id: i32) -> PyResult<()> {
        self.0
            .remove(id)
            .map_err(|_| PyValueError::new_err("Todo not found"))
    }

    fn list(&self) -> Vec<TodoClass> {
        self.0.list().iter().map(Into::into).collect()
    }

    fn get(&self, id: i32) -> Option<TodoClass> {
        self.0.get(id).map(Into::into)
    }
}

#[pymodule]
fn core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<TodosClass>()?;
    m.add_class::<TodoClass>()
}
