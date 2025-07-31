#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_patterns)]

use crate::triangle::Triangle;
use bitflags::bitflags;
use nalgebra::{Matrix4, Vector3, Vector4};
use std::collections::HashMap;

bitflags! {
    pub struct Buffers: u32 {
        const Color = 0x01;
        const Depth = 0x10;
    }
}

impl Buffers {}

pub enum Primitive {
    Triangle,
}

pub struct PosBuf {
    pos_id: usize,
}

impl PosBuf {
    pub fn new(id: usize) -> Self {
        Self { pos_id: id }
    }
    pub fn pos_id(&self) -> &usize {
        &self.pos_id
    }
    pub fn set(&mut self, v: usize) {
        self.pos_id = v;
    }
}

impl Clone for PosBuf {
    fn clone(&self) -> Self {
        Self {
            pos_id: self.pos_id,
        }
    }
}
pub struct IndBuf {
    ind_id: usize,
}

impl IndBuf {
    pub fn new(id: usize) -> Self {
        Self { ind_id: id }
    }
    pub fn ind_id(&self) -> &usize {
        &self.ind_id
    }
    pub fn set(&mut self, v: usize) {
        self.ind_id = v;
    }
}

impl Clone for IndBuf {
    fn clone(&self) -> Self {
        Self {
            ind_id: self.ind_id,
        }
    }
}

pub struct Rasterizer {
    width: usize,
    height: usize,

    pos_buf: HashMap<usize, Vec<Vector3<f32>>>,
    ind_buf: HashMap<usize, Vec<Vector3<i32>>>,
    frame_buf: Vec<Vector3<f32>>,
    depth_buf: Vec<f32>,

    model: Matrix4<f32>,
    view: Matrix4<f32>,
    projection: Matrix4<f32>,

    next_id: usize,
}

impl Rasterizer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pos_buf: HashMap::default(),
            ind_buf: HashMap::default(),
            frame_buf: vec![Vector3::default(); width * height],
            depth_buf: vec![f32::INFINITY; width * height],
            model: Matrix4::identity(),
            view: Matrix4::identity(),
            projection: Matrix4::identity(),
            next_id: 0,
        }
    }

    pub fn clear(&mut self, buffers: Buffers) {
        if buffers.contains(Buffers::Color) {
            for pixel in &mut self.frame_buf {
                *pixel = Vector3::default();
            }
        }
        if buffers.contains(Buffers::Depth) {
            for depth in &mut self.depth_buf {
                *depth = f32::INFINITY;
            }
        }
    }

    pub fn set_model(&mut self, model: Matrix4<f32>) {
        self.model = model;
    }

    pub fn set_view(&mut self, view: Matrix4<f32>) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: Matrix4<f32>) {
        self.projection = projection;
    }

    pub fn set_pixel(&mut self, point: &Vector3<usize>, color: &Vector3<f32>) {
        // old index: auto ind = point.y() + point.x() * width;
        if point.x >= self.width || point.y >= self.height {
            return;
        }

        let ind = (self.height - point.y - 1) * self.width + point.x;
        self.frame_buf[ind] = color.clone();
    }

    pub fn load_positions(&mut self, positions: Vec<Vector3<f32>>) -> PosBuf {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions.clone());
        PosBuf::new(id)
    }

    pub fn load_indices(&mut self, indices: Vec<Vector3<i32>>) -> IndBuf {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices.clone());
        IndBuf::new(id)
    }
    pub fn draw(
        &mut self,
        pos_buffer: &PosBuf,
        ind_buffer: &IndBuf,
        primitive: Primitive,
    ) -> Result<(), String> {
        // Implement triangle rasterization here
        match primitive {
            Primitive::Triangle => {
                let buf = self
                    .pos_buf
                    .get(&pos_buffer.pos_id())
                    .ok_or("Invalid pos buffer id")?
                    .clone();

                let ind = self
                    .ind_buf
                    .get(&ind_buffer.ind_id())
                    .ok_or("Invalid ind buffer id")?
                    .clone();

                let f1 = (100. - 0.1) / 2.0;
                let f2 = (100. + 0.1) / 2.0;

                let mvp = self.projection * self.view * self.model;
                for i in ind {
                    let mut v = vec![
                        mvp * Vector4::new(
                            buf[i[0] as usize][0],
                            buf[i[0] as usize][1],
                            buf[i[0] as usize][2],
                            1.,
                        ),
                        mvp * Vector4::new(
                            buf[i[1] as usize][0],
                            buf[i[1] as usize][1],
                            buf[i[1] as usize][2],
                            1.,
                        ),
                        mvp * Vector4::new(
                            buf[i[2] as usize][0],
                            buf[i[2] as usize][1],
                            buf[i[2] as usize][2],
                            1.,
                        ),
                    ];

                    for vec in &mut v {
                        *vec /= vec[3];
                    }

                    for vert in &mut v {
                        vert.x = 0.5 * self.width as f32 * (vert.x + 1.);

                        vert.y = 0.5 * self.height as f32 * (vert.y + 1.);
                        vert.z = vert.z * f1 + f2;
                    }

                    let mut t = Triangle::default();
                    for (j, vert) in v.iter().enumerate() {
                        t.set_vertex(j, Vector3::new(vert.x, vert.y, vert.z)).ok();
                    }

                    t.set_color(0, 255.0, 0.0, 0.0).ok();
                    t.set_color(1, 0.0, 255.0, 0.0).ok();
                    t.set_color(2, 0.0, 0.0, 255.0).ok();
                    self.rasterize_wireframe(&t)
                }
                Ok(())
            }
            _ => Err("Not supported primitive".to_string()),
        }
    }

    pub fn framebuffer(&self) -> &Vec<Vector3<f32>> {
        &self.frame_buf
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    fn draw_line(&mut self, begin: &Vector3<f32>, end: &Vector3<f32>) {
        let x1 = begin.x;
        let y1 = begin.y;
        let x2 = end.x;
        let y2 = end.y;

        let line_color = Vector3::new(255., 255., 255.);

        let dx = x2 - x1;
        let dy = y2 - y1;
        let dx1 = dx.abs();
        let dy1 = dy.abs();
        let mut px = 2. * dy1 - dx1;
        let mut py = 2. * dx1 - dy1;

        let mut x: i32;
        let mut y: i32;
        let xe: i32;
        let ye: i32;

        if dy1 <= dx1 {
            if dx >= 0. {
                x = x1 as i32;
                y = y1 as i32;
                xe = x2 as i32;
            } else {
                x = x2 as i32;
                y = y2 as i32;
                xe = x1 as i32;
            }
            let point = Vector3::new(x as usize, y as usize, 1);
            self.set_pixel(&point, &line_color);
            let xs = x + 1;
            for x in xs..=xe {
                if px < 0. {
                    px += 2. * dy1;
                } else {
                    if (dx < 0. && dy < 0.) || (dx > 0. && dy > 0.) {
                        y += 1;
                    } else {
                        y -= 1;
                    }
                    px += 2. * (dy1 - dx1);
                }
                let point = Vector3::new(x as usize, y as usize, 1);
                self.set_pixel(&point, &line_color);
            }
        } else {
            if dy >= 0. {
                x = x1 as i32;
                y = y1 as i32;
                ye = y2 as i32;
            } else {
                x = x2 as i32;
                y = y2 as i32;
                ye = y1 as i32;
            }
            let point = Vector3::new(x as usize, y as usize, 1);
            self.set_pixel(&point, &line_color);
            let ys = y + 1;
            for y in ys..=ye {
                if py <= 0. {
                    py += 2. * dx1;
                } else {
                    if (dx < 0. && dy < 0.) || (dx > 0. && dy > 0.) {
                        x += 1;
                    } else {
                        x -= 1;
                    }
                    py += 2. * (dx1 - dy1);
                }
                let point = Vector3::new(x as usize, y as usize, 1);
                self.set_pixel(&point, &line_color);
            }
        }
    }

    fn rasterize_wireframe(&mut self, t: &Triangle) {
        self.draw_line(t.c(), t.a());
        self.draw_line(t.c(), t.b());
        self.draw_line(t.b(), t.a());
    }

    fn get_next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn get_index(&self, x: i32, y: i32) -> i32 {
        (self.height as i32 - y) * self.width as i32 + x
    }
}
