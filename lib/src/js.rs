#![allow(unused)]

use crate::{
    core::{Todo, Todos},
    enumurate::TodoStatus,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Todo, inspectable)]
struct TodoClass(Todo);

impl From<&Todo> for TodoClass {
    fn from(val: &Todo) -> Self {
        TodoClass(val.clone())
    }
}

#[wasm_bindgen(js_class = Todo, inspectable)]
impl TodoClass {
    #[wasm_bindgen(constructor)]
    pub fn new(id: i32, title: String) -> Self {
        TodoClass(Todo::new(id, title))
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> i32 {
        self.0.id
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.0.title.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn status(&self) -> TodoStatus {
        self.0.status.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn completed(&self) -> bool {
        self.0.status == TodoStatus::Completed
    }

    pub fn change_status(&mut self, status: TodoStatus) {
        self.0.change_status(status);
    }
}

#[wasm_bindgen(js_name = Todos)]
struct TodosClass(Todos);

#[wasm_bindgen(js_class = Todos)]
impl TodosClass {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        TodosClass(Todos::default())
    }

    pub fn get(&self, id: i32) -> Option<TodoClass> {
        self.0.get(id).map(Into::into)
    }

    pub fn add(&mut self, id: i32, title: String) {
        self.0.add(id, title);
    }

    pub fn complete(&mut self, id: i32) -> Result<(), String> {
        self.0.complete(id).map_err(String::from)
    }

    pub fn remove(&mut self, id: i32) -> Result<(), String> {
        self.0.remove(id).map_err(String::from)
    }

    #[wasm_bindgen(getter)]
    pub fn list(&self) -> Vec<TodoClass> {
        self.0.list().iter().map(Into::into).collect()
    }
}
