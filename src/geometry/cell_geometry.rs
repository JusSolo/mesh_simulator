use std::sync::Arc;

use super::{
    shape::Shape,
    transform::Transform,
};

#[derive(Clone)]
pub struct CellGeometry {
    pub shape: Arc<dyn Shape>,
    pub transform: Transform,
}