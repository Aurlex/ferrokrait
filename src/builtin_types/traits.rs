#![allow(dead_code)]
use pyo3::prelude::*;

pub trait Lerp<T=Self, A=f64> {
    type Output;
    fn lerp(&self, rhs: &T, lerp: A) -> Self::Output;
}

pub fn lerp<T: Lerp<T, A>, A>(slf: &T, rhs: &T, lerp: A) -> T::Output {
    slf.lerp(rhs, lerp)
}

pub fn clamp<T: Clamp>(slf: &T, min: &T, max: &T) -> T {
    slf.clamp(min, max)
}

#[pyfunction]
#[pyo3(name="lerp")]
pub fn lerp_py(slf: PyObject, py: Python, rhs: PyObject, lerp: f64) -> PyResult<Py<PyAny>> {
    
    if let Ok(slf) = slf.extract::<f64>(py) {
        Ok(slf.lerp(&rhs.extract::<f64>(py)?, lerp).into_py(py))
    } else if let Ok(slf) = slf.extract::<i64>(py) {
        Ok(slf.lerp(&rhs.extract::<i64>(py)?, lerp as i64).into_py(py))
    } else if let Ok(slf) = slf.extract::<u64>(py) {
        Ok(slf.lerp(&rhs.extract::<u64>(py)?, lerp as u64).into_py(py)) 
    } else {
        slf.getattr(py, "lerp")?.call1(py, (rhs, lerp,))
    }
}

#[pyfunction]
#[pyo3(name="clamp")]
pub fn clamp_py(slf: PyObject, py: Python, min: PyObject, max: PyObject) -> PyResult<Py<PyAny>> {
    
    if let Ok(slf) = slf.extract::<f64>(py) {
        Ok(slf.clamp(min.extract::<f64>(py)?, max.extract::<f64>(py)?).into_py(py))
    } else if let Ok(slf) = slf.extract::<i64>(py) {
        Ok(slf.clamp(min.extract::<i64>(py)?, max.extract::<i64>(py)?).into_py(py))
    } else if let Ok(slf) = slf.extract::<u64>(py) {
        Ok(slf.clamp(min.extract::<u64>(py)?, max.extract::<u64>(py)?).into_py(py))
    } else {
        slf.getattr(py, "clamp")?.call1(py, (min, max, ))
    }
}

pub trait Clamp: Min + Max + Sized {
    fn clamp(&self, min: &Self, max: &Self) -> Self {
        self.max(min).min(max)
    }
}

pub trait Min {
    fn min(&self, rhs: &Self) -> Self;
}

pub trait Max {
    fn max(&self, rhs: &Self) -> Self;
}

impl Lerp<Self, Self> for f64 {
    type Output = Self;
    fn lerp(&self, rhs: &Self, lerp: Self) -> Self::Output {
        self + lerp * (rhs - self)
    }
}
impl Lerp<Self, Self> for i64 {
    type Output = Self;
    fn lerp(&self, rhs: &Self, lerp: Self) -> Self::Output {
        self + lerp * (rhs - self)
    }
}
impl Lerp<Self, Self> for u64 {
    type Output = Self;
    fn lerp(&self, rhs: &Self, lerp: Self) -> Self::Output {
        self + lerp * (rhs - self)
    }
}
impl Lerp<Self, Self> for f32 {
    type Output = Self;
    fn lerp(&self, rhs: &Self, lerp: Self) -> Self::Output {
        self + lerp * (rhs - self)
    }
}
impl Lerp<Self, Self> for i32 {
    type Output = Self;
    fn lerp(&self, rhs: &Self, lerp: Self) -> Self::Output {
        self + lerp * (rhs - self)
    }
}
impl Lerp<Self, Self> for u32 {
    type Output = Self;
    fn lerp(&self, rhs: &Self, lerp: Self) -> Self::Output {
        self + lerp * (rhs - self)
    }
}
impl Lerp<Self, Self> for i16 {
    type Output = Self;
    fn lerp(&self, rhs: &Self, lerp: Self) -> Self::Output {
        self + lerp * (rhs - self)
    }
}
impl Lerp<Self, Self> for u16 {
    type Output = Self;
    fn lerp(&self, rhs: &Self, lerp: Self) -> Self::Output {
        self + lerp * (rhs - self)
    }
}
impl Lerp<Self, Self> for i8 {
    type Output = Self;
    fn lerp(&self, rhs: &Self, lerp: Self) -> Self::Output {
        self + lerp * (rhs - self)
    }
}
impl Lerp<Self, Self> for u8 {
    type Output = Self;
    fn lerp(&self, rhs: &Self, lerp: Self) -> Self::Output {
        self + lerp * (rhs - self)
    }
}
impl Lerp<Self, Self> for usize {
    type Output = Self;
    fn lerp(&self, rhs: &Self, lerp: Self) -> Self::Output {
        self + lerp * (rhs - self)
    }
}
impl Lerp<Self, Self> for isize {
    type Output = Self;
    fn lerp(&self, rhs: &Self, lerp: Self) -> Self::Output {
        self + lerp * (rhs - self)
    }
}