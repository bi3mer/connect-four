use crate::scene::scene_id::SceneId;

pub trait Scene {
    fn update(&mut self, ai: &mut crate::AIType) -> SceneId;
}