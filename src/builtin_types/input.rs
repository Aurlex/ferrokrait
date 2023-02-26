use pyo3::prelude::*;
use device_query::{Keycode, DeviceState, DeviceQuery};
use std::cell::LazyCell;
use crate::builtin_types::vec2::Vec2;

static mut QUERY_HANDLER: LazyCell<DeviceState> = LazyCell::new(DeviceState::new);

#[pyclass]
pub struct Input;

#[pymethods]
impl Input {
    #[staticmethod]
    pub unsafe fn get_key(key: Key) -> i8 {
        if key == Key::Any {
            return !QUERY_HANDLER.get_keys().is_empty() as i8;
        }
        QUERY_HANDLER
            .get_keys()
            .contains(&Keycode::try_from(key).expect("Invalid key")) as i8
    }
    #[staticmethod]
    pub unsafe fn all_keys() -> Vec<Key> {
        QUERY_HANDLER
            .get_keys()
            .iter()
            .map(Key::from)
            .collect()
    }
    #[staticmethod]
    pub unsafe fn mouse_pos() -> Vec2 {
        let t = QUERY_HANDLER.get_mouse().coords;
        Vec2 {
            x: t.0 as f64,
            y: t.1 as f64,
        }
    }
    #[staticmethod]
    pub unsafe fn relative_mouse_pos(origin: Vec2) -> Vec2 {
        Self::mouse_pos() - origin
    }
    #[staticmethod]
    pub unsafe fn get_vector(up: Key, down: Key, left: Key, right: Key) -> Vec2 {
        Vec2 {
            x: (Input::get_key(right) as i32 - Input::get_key(left) as i32) as f64,
            y: (Input::get_key(up) as i32 - Input::get_key(down) as i32) as f64,
        }.normalised()
    }
    #[staticmethod]
    pub unsafe fn get_action(action: Action) -> bool {
        let t: Vec<Key> = QUERY_HANDLER
            .get_keys()
            .iter()
            .map(Key::from)
            .collect();
        for item in action.0.iter() {
            if t.binary_search(item).is_err() {
            	return false
            }
        }
        true
    }
}
impl From<&Keycode> for Key {
    fn from(val: &Keycode) -> Self {
        match *val {
            Keycode::A => Self::A,
            Keycode::B => Self::B,
            Keycode::C => Self::C,
            Keycode::D => Self::D,
            Keycode::E => Self::E,
            Keycode::F => Self::F,
            Keycode::G => Self::G,
            Keycode::H => Self::H,
            Keycode::I => Self::I,
            Keycode::J => Self::J,
            Keycode::K => Self::K,
            Keycode::L => Self::L,
            Keycode::M => Self::M,
            Keycode::N => Self::N,
            Keycode::O => Self::O,
            Keycode::P => Self::P,
            Keycode::Q => Self::Q,
            Keycode::R => Self::R,
            Keycode::S => Self::S,
            Keycode::T => Self::T,
            Keycode::U => Self::U,
            Keycode::V => Self::V,
            Keycode::W => Self::W,
            Keycode::X => Self::X,
            Keycode::Y => Self::Y,
            Keycode::Z => Self::Z,
            Keycode::Space => Self::Space,
            Keycode::LControl => Self::LeftCtrl,
            Keycode::RControl => Self::RightCtrl,
            Keycode::LShift => Self::LeftShift,
            Keycode::RShift => Self::RightShift,
            Keycode::LAlt => Self::LeftAlt,
            Keycode::RAlt => Self::RightAlt,
            _ => Self::Null,
        }
    }
}
impl TryFrom<Key> for Keycode {
    type Error = ();
    fn try_from(val: Key) -> Result<Self, ()> {
        match val {
            Key::A => Ok(Self::A),
            Key::B => Ok(Self::B),
            Key::C => Ok(Self::C),
            Key::D => Ok(Self::D),
            Key::E => Ok(Self::E),
            Key::F => Ok(Self::F),
            Key::G => Ok(Self::G),
            Key::H => Ok(Self::H),
            Key::I => Ok(Self::I),
            Key::J => Ok(Self::J),
            Key::K => Ok(Self::K),
            Key::L => Ok(Self::L),
            Key::M => Ok(Self::M),
            Key::N => Ok(Self::N),
            Key::O => Ok(Self::O),
            Key::P => Ok(Self::P),
            Key::Q => Ok(Self::Q),
            Key::R => Ok(Self::R),
            Key::S => Ok(Self::S),
            Key::T => Ok(Self::T),
            Key::U => Ok(Self::U),
            Key::V => Ok(Self::V),
            Key::W => Ok(Self::W),
            Key::X => Ok(Self::X),
            Key::Y => Ok(Self::Y),
            Key::Z => Ok(Self::Z),
            Key::Space => Ok(Self::Space),
            Key::LeftCtrl => Ok(Self::LControl),
            Key::RightCtrl => Ok(Self::RControl),
            Key::LeftShift => Ok(Self::LShift),
            Key::RightShift => Ok(Self::RShift),
            Key::LeftAlt => Ok(Self::LAlt),
            Key::RightAlt => Ok(Self::RAlt),
            _ => Err(()),
        }
    }
}
#[pyclass]
#[derive(Copy, Clone, PartialEq, Debug, PartialOrd, Ord, Eq)]
pub enum Key {
    Null,
    Any,
    Space,
    LeftAlt,
    RightAlt, 
    LeftCtrl,
    RightCtrl, 
    LeftShift,
    RightShift,
    Tab,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}
#[pymethods]
impl Key {
    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

#[derive(Clone)]
#[pyclass]
pub struct Action(Vec<Key>);

#[pymethods]
impl Action {
    #[new]
    pub fn new(keys: Vec<Key>) -> Self {
        Self(keys)
    }
}
