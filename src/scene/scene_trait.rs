use crate::scene::scene_id::SceneId;

pub trait Scene {
    fn update(&mut self, ai: &crate::AIType) -> SceneId;
}