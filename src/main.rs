use guile;
use bevy::prelude::*;


fn main() {
    println!("Hello, world!");
    guile::init(|vm| {
        let args = vec!["Test".to_string()];
        vm.shell(args);
    });
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup (mut commands: Commands) {
    println!("Bevy setup system running.");
}
