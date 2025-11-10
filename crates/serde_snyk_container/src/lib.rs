#[allow(clippy::all)]
#[allow(dead_code)]
#[rustfmt::skip]
mod bindings {
    include!("model.rs");
}
pub use bindings::*;
