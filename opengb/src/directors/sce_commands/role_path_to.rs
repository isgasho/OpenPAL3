use super::{map_role_id, nav_coord_to_scene_coord};
use crate::directors::sce_director::SceCommand;
use crate::directors::sce_state::SceState;
use crate::directors::SceneManagerExtensions;
use crate::scene::RoleAnimationRepeatMode;
use imgui::Ui;
use radiance::scene::Entity;
use radiance::{math::Vec3, scene::SceneManager};

#[derive(Clone)]
pub struct SceCommandRolePathTo {
    role_id: String,
    nav_x: f32,
    nav_y: f32,
    unknown: i32,
}

impl SceCommand for SceCommandRolePathTo {
    fn initialize(&mut self, scene_manager: &mut dyn SceneManager, state: &mut SceState) {
        scene_manager
            .scene_mut_or_fail()
            .get_role_entity(&self.role_id)
            .play_anim("C02", RoleAnimationRepeatMode::Repeat);
    }

    fn update(
        &mut self,
        scene_manager: &mut dyn SceneManager,
        ui: &mut Ui,
        state: &mut SceState,
        delta_sec: f32,
    ) -> bool {
        const SPEED: f32 = 100.;

        let scene = scene_manager.scene_mut_or_fail();
        let to = nav_coord_to_scene_coord(scene, self.nav_x, self.nav_y);
        let position = scene.get_role_entity(&self.role_id).transform().position();
        let step = SPEED * delta_sec;
        let remain = Vec3::sub(&to, &position);
        let completed = remain.norm() < step;
        let new_position = if completed {
            to
        } else {
            Vec3::add(&position, &Vec3::dot(step, &Vec3::normalized(&remain)))
        };

        let entity = scene.get_role_entity(&self.role_id);
        entity
            .transform_mut()
            .look_at(&to)
            .set_position(&new_position);

        if completed {
            scene
                .get_role_entity(&self.role_id)
                .play_anim("C01", RoleAnimationRepeatMode::Repeat);
        }
        completed
    }
}

impl SceCommandRolePathTo {
    pub fn new(role_id: i32, nav_x: i32, nav_y: i32, unknown: i32) -> Self {
        Self {
            role_id: map_role_id(role_id).to_string(),
            nav_x: nav_x as f32,
            nav_y: nav_y as f32,
            unknown,
        }
    }
}
