mod exp_director;
mod sce_commands;
mod sce_director;
mod sce_state;

use crate::scene::ScnScene;
use radiance::scene::{CoreScene, SceneManager};
pub use sce_director::SceDirector;

pub trait SceneManagerExtensions: SceneManager {
    fn scene_mut_or_fail(&mut self) -> &mut CoreScene<ScnScene> {
        self.scene_mut()
            .expect("No scene loaded. Probably a bug in Sce procedures.")
            .downcast_mut::<CoreScene<ScnScene>>()
            .unwrap()
    }
}

impl<T: SceneManager + ?Sized> SceneManagerExtensions for T {}
