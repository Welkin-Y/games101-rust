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

pub struct ColBuf {
    col_id: usize,
}

impl ColBuf {
    pub fn new(id: usize) -> Self {
        Self { col_id: id }
    }
    pub fn col_id(&self) -> &usize {
        &self.col_id
    }
    pub fn set(&mut self, v: usize) {
        self.col_id = v;
    }
}

impl Clone for ColBuf {
    fn clone(&self) -> Self {
        Self {
            col_id: self.col_id,
        }
    }
}

fn compute_barycentric_2d(x: f32, y: f32, v: &[Vector3<f32>; 3]) -> (f32, f32, f32) {
    let c1 = (x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * y + v[1].x * v[2].y - v[2].x * v[1].y)
        / (v[0].x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * v[0].y + v[1].x * v[2].y
            - v[2].x * v[1].y);
    let c2 = (x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * y + v[2].x * v[0].y - v[0].x * v[2].y)
        / (v[1].x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * v[1].y + v[2].x * v[0].y
            - v[0].x * v[2].y);
    let c3 = (x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * y + v[0].x * v[1].y - v[1].x * v[0].y)
        / (v[2].x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * v[2].y + v[0].x * v[1].y
            - v[1].x * v[0].y);
    (c1, c2, c3)
}

pub struct Rasterizer {
    width: usize,
    height: usize,

    pos_buf: HashMap<usize, Vec<Vector3<f32>>>,
    ind_buf: HashMap<usize, Vec<Vector3<i32>>>,
    col_buf: HashMap<usize, Vec<Vector3<f32>>>,
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
            col_buf: HashMap::default(),
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

    pub fn set_pixel(&mut self, point: &Vector3<f32>, color: &Vector3<f32>) {
        // old index: auto ind = point.y() + point.x() * width;
        if point.x >= self.width as f32 || point.y >= self.height as f32 {
            return;
        }

        let ind = (self.height - point.y as usize - 1) * self.width + point.x as usize;
        if self.depth_buf[ind] > point.z {
            self.frame_buf[ind] = color.clone();
            self.depth_buf[ind] = point.z;
        }
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

    pub fn load_colors(&mut self, colors: Vec<Vector3<f32>>) -> ColBuf {
        let id = self.get_next_id();
        self.col_buf.insert(id, colors.clone());
        ColBuf::new(id)
    }

    pub fn draw(
        &mut self,
        pos_buffer: &PosBuf,
        ind_buffer: &IndBuf,
        col_buffer: &ColBuf,
        primitive: Primitive,
    ) -> Result<(), String> {
        // Implement triangle rasterization here
        match primitive {
            Primitive::Triangle => {
                let buf = self
                    .pos_buf
                    .get(pos_buffer.pos_id())
                    .ok_or("Invalid pos buffer id")?
                    .clone();

                let ind = self
                    .ind_buf
                    .get(ind_buffer.ind_id())
                    .ok_or("Invalid ind buffer id")?
                    .clone();

                let col = self
                    .col_buf
                    .get(col_buffer.col_id())
                    .ok_or("Invlid color buffer id")?
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

                    let col_x = col[i[0] as usize];
                    let col_y = col[i[1] as usize];
                    let col_z = col[i[2] as usize];

                    t.set_color(0, col_x[0], col_x[1], col_x[2]).ok();
                    t.set_color(1, col_y[0], col_y[1], col_y[2]).ok();
                    t.set_color(2, col_z[0], col_z[1], col_z[2]).ok();
                    self.rasterize_wireframe(&t);
                    self.rasterize_triangle(&t);
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
            let point = Vector3::new(x as f32, y as f32, 1.);
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
                let point = Vector3::new(x as f32, y as f32, 1.);
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
            let point = Vector3::new(x as f32, y as f32, 1.);
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
                let point = Vector3::new(x as f32, y as f32, 1.);
                self.set_pixel(&point, &line_color);
            }
        }
    }

    fn rasterize_wireframe(&mut self, t: &Triangle) {
        self.draw_line(t.c(), t.a());
        self.draw_line(t.c(), t.b());
        self.draw_line(t.b(), t.a());
    }

    fn rasterize_triangle(&mut self, t: &Triangle) {
        //TODO: get bound box
        let v = t.to_vector4();
        let right = t.a()[0].max(t.b()[0]).max(t.c()[0]);
        let left = t.a()[0].min(t.b()[0]).min(t.c()[0]);
        let top = t.a()[1].max(t.b()[1]).max(t.c()[1]);
        let bottom = t.a()[1].min(t.b()[1]).min(t.c()[1]);
        for x in left as i32..right as i32 {
            for y in bottom as i32..top as i32 {
                if t.contains(x as f32 + 0.5, y as f32 + 0.5) {
                    let (alpha, beta, gamma) =
                        compute_barycentric_2d(x as f32 + 0.5, y as f32 + 0.5, t.v());
                    let w_reciprocal = 1.0 / (alpha / v[0].w + beta / v[1].w + gamma / v[2].w);
                    let mut z_interpolated =
                        alpha * v[0].z / v[0].w + beta * v[1].z / v[1].w + gamma * v[2].z / v[2].w;
                    z_interpolated *= w_reciprocal;
                    //TODO: only one color per triangle
                    self.set_pixel(
                        &Vector3::new(x as f32, y as f32, z_interpolated),
                        &t.get_color(),
                    );
                }
            }
        }
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
