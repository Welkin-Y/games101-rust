use nalgebra::Vector3;
use opencv::core::MatTraitConst;
use opencv::core::{Mat, Vec3b};
use opencv::imgcodecs::imread;
use opencv::imgcodecs::ImreadModes;

#[derive(Debug)]
pub struct Texture {
    image_data: Mat,
    pub width: i32,
    pub height: i32,
}

impl Texture {
    fn new(name: &str) -> Self {
        let image = &imread(name, ImreadModes::IMREAD_COLOR_BGR.into())
            .ok()
            .unwrap();
        Self {
            image_data: image.clone(),
            width: image.cols(),
            height: image.rows(),
        }
    }

    fn get_color(&self, u: f32, v: f32) -> Vector3<f32> {
        let u_img = (u * self.width as f32) as i32;
        let v_img = ((1. - v) * self.height as f32) as i32;
        let color = self.image_data.at_2d::<Vec3b>(v_img, u_img).unwrap();
        Vector3::new(color[0] as f32, color[1] as f32, color[2] as f32)
    }
}
