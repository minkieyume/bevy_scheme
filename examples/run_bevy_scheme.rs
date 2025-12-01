use bevy::prelude::*;
use bevy_scheme::*;

fn main() {
    println!("Hello, world!") ;
    GuileRuntime::init(|vm| {
        App::new()
	    .add_plugins(DefaultPlugins)
	    .add_systems(Startup, setup)
	    .run();
    })
}

fn setup (mut commands: Commands) {
    println!("Bevy setup system running.");
}
