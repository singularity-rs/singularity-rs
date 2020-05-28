use amethyst::ecs::{Component, DenseVecStorage};

// NEXT: Ask on discord: How do I draw a (non-debug) Line between to points? in #help

#[derive(Debug, Default)]
pub struct Road();

impl Component for Road {
    type Storage = DenseVecStorage<Self>;
}
