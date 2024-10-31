#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "python")]
use pyo3::{exceptions::PyValueError, prelude::*};

#[cfg(feature = "wasm")]
type MyResult<T> = Result<T, String>;

#[cfg(feature = "python")]
type MyResult<T> = PyResult<T>;

#[cfg(feature = "cpp")]
type MyResult<T> = Result<T, String>;

#[cfg_attr(feature = "python", pyclass(eq, eq_int))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "cpp", repr(C))]
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum TodoStatus {
    #[default]
    NotStarted,
    InProgress,
    Completed,
}

#[cfg_attr(feature = "wasm", wasm_bindgen(js_name = Todo, inspectable))]
#[cfg_attr(feature = "python", pyclass(name = "Todo"))]
#[cfg_attr(feature = "cpp", repr(C))]
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Todo {
    id: i32,
    title: String,
    status: TodoStatus,
}

#[cfg(feature = "cpp")]
#[no_mangle]
pub extern "C" fn new_todo(id: i32, title: String) -> Todo {
    Todo::new(id, title)
}

#[cfg_attr(feature = "wasm", wasm_bindgen(js_class = Todo))]
#[cfg_attr(feature = "python", pymethods)]
impl Todo {
    #[cfg(feature = "cpp")]
    pub fn new(id: i32, title: String) -> Self {
        Todo {
            id,
            title,
            status: TodoStatus::default(),
        }
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(constructor)]
    pub fn new(id: i32, title: String) -> Self {
        Todo {
            id,
            title,
            status: TodoStatus::default(),
        }
    }

    #[cfg(feature = "python")]
    #[new]
    pub fn new(id: i32, title: String) -> Self {
        Todo {
            id,
            title,
            status: TodoStatus::default(),
        }
    }

    #[no_mangle]
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub extern "C" fn status(&self) -> TodoStatus {
        self.status
    }

    #[no_mangle]
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub extern "C" fn completed(&self) -> bool {
        self.status == TodoStatus::Completed
    }

    #[no_mangle]
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub extern "C" fn title(&self) -> String {
        self.title.clone()
    }

    #[no_mangle]
    pub extern "C" fn change_status(&mut self, status: TodoStatus) {
        self.status = status;
    }

    #[no_mangle]
    pub extern "C" fn change_title(&mut self, title: String) {
        self.title = title;
    }

    #[cfg(feature = "python")]
    fn __repr__(&self) -> String {
        format!(
            "Todo {{ id: {}, title: \"{}\", status: {:?} }}",
            self.id, self.title, self.status
        )
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen(js_name = Todos, inspectable))]
#[cfg_attr(feature = "python", pyclass(name = "Todos"))]
#[cfg_attr(feature = "cpp", repr(C))]
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Todos(Vec<Todo>);

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_class = Todos)]
impl Todos {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen(js_class = Todos))]
#[cfg_attr(feature = "python", pymethods)]
impl Todos {
    #[cfg(feature = "python")]
    #[new]
    fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, id: i32, title: String) {
        self.0.push(Todo::new(id, title));
    }

    #[cfg(feature = "python")]
    pub fn complete(&mut self, id: i32) -> MyResult<()> {
        let todo = self
            .0
            .iter_mut()
            .find(|todo| todo.id == id)
            .ok_or_else(|| PyValueError::new_err("Todo not found"))?;
        todo.change_status(TodoStatus::Completed);

        Ok(())
    }

    #[cfg(feature = "wasm")]
    pub fn complete(&mut self, id: i32) -> MyResult<()> {
        let todo = self
            .0
            .iter_mut()
            .find(|todo| todo.id == id)
            .ok_or("Todo not found".to_string())?;
        todo.change_status(TodoStatus::Completed);
        Ok(())
    }

    pub fn remove(&mut self, id: usize) -> MyResult<()> {
        let _ = self.0.remove(id);

        Ok(())
    }

    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn list(&self) -> Vec<Todo> {
        self.0.clone()
    }

    pub fn get(&self, id: i32) -> Option<Todo> {
        self.0.iter().find(|todo| todo.id == id).cloned()
    }
}

#[cfg(feature = "python")]
#[pymodule]
fn core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Todo>()
}
