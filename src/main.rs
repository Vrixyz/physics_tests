use bevy::prelude::*;
use utils::PluginUtils;

pub mod physics_test;
pub mod utils;

use physics_test::*;

fn main() {
    let mut app = App::build();
    app.add_plugin(PluginPhysics);
    app.add_plugin(PluginUtils);
    app.run();
}
