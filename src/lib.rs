use bevy::prelude::*;

pub use lyon::{
    math::{point, Box2D, Point},
    path::{builder::BorderRadii, Winding},
    tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers},
};

pub struct BevyonPlugin;

impl Plugin for BevyonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup() {}
