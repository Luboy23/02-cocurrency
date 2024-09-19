mod matrix;
mod vecotr;
mod metrics;

pub use matrix::{multiply, Matrix};
pub use vecotr::{Vector, dot_product};
pub use metrics::{AmapMetrics,CmapMetrics};