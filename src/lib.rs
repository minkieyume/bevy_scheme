mod runtimes;

use guile;
use bevy::prelude::*;
pub use runtimes::guile::*;

pub struct GuilePlugin;

impl Plugin for GuilePlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
    }
}
