#![allow(dead_code)]
use pyo3::{exceptions::PySystemError, prelude::*, types::PyType, PyTraverseError, PyVisit};
use spin_sleep::LoopHelper;

pub mod node;

use crate::utils::*;
use node::*;

#[pyclass]
#[derive(Default, Clone, Debug)]
pub struct NodeTree {
    nodes: Vec<PyObject>,
    running: bool,
}

#[pymethods]
impl NodeTree {
    #[new]
    pub const fn new() -> Self {
        Self {
            nodes: Vec::new(),
            running: false,
        }
    }
    pub fn add_node(slf: Py<Self>, py: Python, class_object: PyObject) -> PyResult<Py<Self>> {
        let object: PyObject = class_object.call0(py)?;
        if object.is_subclass::<Node>(py)? {
            object.setattr(py, "nodetree_ref", Some(slf.clone_ref(py)))?;
            let mut bases = object
                .get_pytype(py)
                .getattr("__mro__")?
                .extract::<Vec<Py<PyType>>>()?;
            bases.remove(bases.len() - 1);
            bases.remove(bases.len() - 1);
            bases.reverse();
            object.setattr(py, "mro", bases)?;
            slf.borrow_mut(py).nodes.push(object)
        }
        Ok(slf)
    }
    pub fn run(slf: Py<Self>, py: Python, fps: Option<usize>) -> PyResult<()> {
        if slf.borrow(py).running {
            return Err(PySystemError::new_err("This NodeTree is already running!"));
        }
        slf.borrow_mut(py).running = true;
        let nodes: &Vec<PyObject> = &slf.borrow(py).nodes;

        for node in nodes {
            node.call_method0(py, "_ready_recursive")?;
        }

        let mut loop_helper: LoopHelper = if let Some(fps) = fps {
            LoopHelper::builder()
                .report_interval_s(0.5)
                .build_with_target_rate(fps as f64)
        } else {
            LoopHelper::builder()
                .report_interval_s(0.5)
                .build_without_target_rate()
        };

        loop {
            let delta: f64 = loop_helper.loop_start_s();
            py.check_signals()?;
            for node in nodes {
                node.call_method1(py, "_process_recursive", (delta,))?;
            }
            loop_helper.loop_sleep();
        }
    }
    pub fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
    pub fn __traverse__(&self, visit: PyVisit) -> Result<(), PyTraverseError> {
        self.nodes
            .iter()
            .try_for_each(|py_object| -> Result<(), PyTraverseError> { visit.call(py_object) })
    }
    pub fn __clear__(&mut self) {
        self.nodes.clear()
    }
}
