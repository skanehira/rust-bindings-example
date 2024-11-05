#[cfg(feature = "cpp")]
use std::ffi::{c_char, CString};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg_attr(feature = "python", pyclass(eq, eq_int))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, PartialEq, Clone, Copy, Default)]
#[repr(C)]
pub enum TodoStatus {
    #[default]
    NotStarted,
    InProgress,
    Completed,
}

#[cfg(not(feature = "cpp"))]
#[cfg_attr(feature = "wasm", wasm_bindgen(inspectable))]
#[cfg_attr(feature = "python", pyclass)]
#[derive(Default, Debug, PartialEq, Clone)]
/// cbindgen:ignore
pub struct Todo {
    id: i32,
    title: String,
    status: TodoStatus,
}

#[cfg(feature = "cpp")]
#[derive(Debug, PartialEq, Clone)]
#[repr(C)]
pub struct Todo {
    id: i32,
    title: *const c_char,
    status: TodoStatus,
}

/// # Safety
#[cfg(feature = "cpp")]
#[no_mangle]
pub unsafe extern "C" fn new_todo(id: i32, title: *const c_char) -> *mut Todo {
    Box::into_raw(Box::new(Todo {
        id,
        title,
        status: TodoStatus::default(),
    }))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl Todo {
    #[wasm_bindgen(constructor)]
    pub fn new(id: i32, title: String) -> Self {
        Todo {
            id,
            title,
            status: TodoStatus::default(),
        }
    }
}

/// # Safety
#[cfg(feature = "cpp")]
#[no_mangle]
pub unsafe extern "C" fn todo_free(o: *mut Todo) {
    if !o.is_null() {
        unsafe {
            let _ = Box::from_raw(o);
        }
    }
}

/// # Safety
#[cfg(feature = "cpp")]
#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen(js_class = Todo))]
#[cfg_attr(feature = "python", pymethods)]
impl Todo {
    #[cfg(feature = "python")]
    #[new]
    pub fn new(id: i32, title: String) -> Self {
        Todo {
            id,
            title,
            status: TodoStatus::default(),
        }
    }

    #[cfg(not(feature = "cpp"))]
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    /// cbindgen:ignore
    pub fn status(&self) -> TodoStatus {
        self.status
    }

    #[cfg(feature = "cpp")]
    #[no_mangle]
    pub extern "C" fn status_str(&self) -> *mut c_char {
        let status = format!("{:?}", self.status);
        CString::new(status).unwrap().into_raw()
    }

    #[no_mangle]
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub extern "C" fn completed(&self) -> bool {
        self.status == TodoStatus::Completed
    }

    #[no_mangle]
    pub extern "C" fn change_status(&mut self, status: TodoStatus) {
        self.status = status;
    }

    #[cfg(feature = "python")]
    fn __repr__(&self) -> String {
        format!(
            "Todo {{ id: {}, title: \"{}\", status: {:?} }}",
            self.id, self.title, self.status
        )
    }
}

#[cfg(feature = "python")]
#[pymodule]
fn core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Todo>().unwrap();
    m.add_class::<TodoStatus>()
}
