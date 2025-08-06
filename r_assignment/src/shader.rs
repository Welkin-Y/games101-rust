use crate::texture::Texture;
use nalgebra::{Vector2, Vector3};

#[derive(Default, Debug)]
pub struct FragmentShaderPayload {
    pub view_pos: Vector3<f32>,
    pub color: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coords: Vector2<f32>,
    pub texture: Option<Texture>,
}

impl FragmentShaderPayload {
    fn new(col: &Vector3<f32>, nor: &Vector3<f32>, tc: &Vector2<f32>, tex: Texture) -> Self {
        Self {
            color: *col,
            normal: *nor,
            tex_coords: *tc,
            texture: Some(tex),
            view_pos: Vector3::default(),
        }
    }
}
