#[cfg(feature = "python")]
use pyo3::pyclass;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "python", pyclass(eq, eq_int))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, PartialEq, Clone)]
pub enum TodoStatus {
    NotStarted,
    InProgress,
    Completed,
}
