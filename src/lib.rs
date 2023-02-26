#![feature(once_cell)]
use pyo3::prelude::*;

mod nodetree;
mod utils;
mod builtin_types;

use nodetree::{NodeTree, node::builtin_nodes::all::*};
use builtin_types::all::*;

#[pymodule]
fn ferrokrait(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_class::<NodeTree>()?;
	m.add_class::<Node>()?;
	m.add_class::<Vec2>()?;
	m.add_class::<Vec3>()?;
	m.add_class::<Vec4>()?;
	m.add_class::<Input>()?;
	m.add_class::<Color>()?;
	m.add_function(wrap_pyfunction!(lerp_py, m)?)?;
	m.add_function(wrap_pyfunction!(clamp_py, m)?)?;
    Ok(())
}
