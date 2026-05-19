#[path = "../src/network/mod.rs"]
mod network;

#[path = "../src/bevy_layer.rs"]
mod bevy_layer;

use bevy::prelude::*;
use bevy_layer::NetworkingPlugin;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(NetworkingPlugin)
        .run();
}
