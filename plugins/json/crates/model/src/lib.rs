pub use component::load_json::*;
pub use component::save_json::*;
pub use entity::load_json::*;
pub use entity::save_json::*;

pub mod component;
pub mod entity;

pub const NAMESPACE_JSON: &str = "json";
