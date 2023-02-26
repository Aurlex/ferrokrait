use pyo3::prelude::*;
use pyo3::types::PyType;

use crate::nodetree::NodeTree;

pub mod builtin_nodes;

#[pyclass(subclass)]
#[derive(Clone, Default, Debug)]
pub struct Node {
    #[pyo3(set, get)]
    nodetree_ref: Option<Py<NodeTree>>,
    #[pyo3(set, get)]
    mro: Vec<Py<PyType>>,
}

#[pymethods]
impl Node {
    #[new]
    pub fn new() -> Self {
        Self {
            nodetree_ref: None,
            mro: Vec::new(),
        }
    }
    pub fn _ready(&self) {}
    pub const fn _process(&self, _delta: f64) {}
    pub fn get_nodetree(&self, py: Python) -> Option<Py<NodeTree>> {
        self.nodetree_ref
            .as_ref()
            .map(|nodetree_ref| nodetree_ref.clone_ref(py))
    }
    pub fn _ready_recursive(slf: &PyCell<Self>, py: Python) -> PyResult<()> {
        for pytype in &slf.borrow().mro {
            pytype.call_method1(py, "_ready", (slf,))?;
        }
        Ok(())
    }
    pub fn _process_recursive(slf: &PyCell<Self>, py: Python, delta: f64) -> PyResult<()> {
        for pytype in &slf.borrow().mro {
            pytype.call_method1(py, "_process", (slf, delta))?;
        }
        Ok(())
    }
}
