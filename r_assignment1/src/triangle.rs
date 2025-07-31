#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use nalgebra::{Vector2, Vector3, Vector4};
use std::ops::RangeInclusive;

pub struct Triangle {
    v: [Vector3<f32>; 3],
    color: [Vector3<f32>; 3],
    tex_coords: [Vector2<f32>; 3],
    normal: [Vector3<f32>; 3],
}

impl Triangle {
    pub fn default() -> Self {
        Self {
            v: [Vector3::<f32>::default(); 3],
            color: [Vector3::<f32>::default(); 3],
            tex_coords: [Vector2::<f32>::default(); 3],
            normal: [Vector3::<f32>::default(); 3],
        }
    }

    pub fn a(&self) -> &Vector3<f32> {
        &self.v[0]
    }
    pub fn b(&self) -> &Vector3<f32> {
        &self.v[1]
    }
    pub fn c(&self) -> &Vector3<f32> {
        &self.v[2]
    }
    pub fn set_vertex(&mut self, ind: usize, ver: Vector3<f32>) -> Result<(), String> {
        self.check_ind(ind)?;
        self.v[ind] = ver;
        Ok(())
    }
    pub fn set_normal(&mut self, ind: usize, n: Vector3<f32>) -> Result<(), String> {
        self.check_ind(ind)?;
        self.normal[ind] = n;
        Ok(())
    }
    pub fn set_color(&mut self, ind: usize, r: f32, g: f32, b: f32) -> Result<(), String> {
        let range = RangeInclusive::new(0., 255.);
        if !range.contains(&r) || !range.contains(&g) || !range.contains(&b) {
            return Err("Invalid color values".to_string());
        }
        self.color[ind] = Vector3::new(r, g, b);
        Ok(())
    }

    pub fn set_tex_coord(&mut self, ind: usize, s: f32, t: f32) -> Result<(), String> {
        self.check_ind(ind)?;
        self.tex_coords[ind] = Vector2::new(s, t);
        Ok(())
    }

    pub fn to_vector4(&self) -> Vector3<Vector4<f32>> {
        Vector3::default()
    }

    fn check_ind(&self, ind: usize) -> Result<(), String> {
        let range = RangeInclusive::new(0, 2);
        if !range.contains(&ind) {
            return Err("Invalid ind".to_string());
        }
        Ok(())
    }
}
