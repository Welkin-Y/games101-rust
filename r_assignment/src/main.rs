#![allow(unused_imports)]
#![allow(unused_variables)]
use nalgebra::{Matrix4, Vector3};
use opencv::{core::Mat, highgui, prelude::*};
mod rst;
mod triangle;

fn get_model_matrix(angle: f32) -> Matrix4<f32> {
    let sina = (angle / 180.0 * std::f32::consts::PI).sin();
    let cosa = (angle / 180.0 * std::f32::consts::PI).cos();
    Matrix4::new(
        cosa, -sina, 0.0, 0.0, sina, cosa, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    )
}

fn get_view_matrix(eye_pos: Vector3<f32>) -> Matrix4<f32> {
    Matrix4::new(
        1.0,
        0.0,
        0.0,
        -eye_pos[0],
        0.0,
        1.0,
        0.0,
        -eye_pos[1],
        0.0,
        0.0,
        1.0,
        -eye_pos[2],
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

fn get_projection_matrx(eye_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Matrix4<f32> {
    let mut projection: Matrix4<f32> = Matrix4::identity();
    let eye_fov_rad = eye_fov * std::f32::consts::PI / 180.;

    // Calculate the top, bottom, right, and left values
    let t = (eye_fov_rad / 2.).tan() * z_near;
    let r = t * aspect_ratio;
    let l = -r;
    let b = -t;

    projection[(0, 0)] = 2. * z_near / (r - l);
    projection[(1, 1)] = 2. * z_near / (t - b);
    projection[(2, 2)] = -(z_far + z_near) / (z_far - z_near);
    projection[(2, 3)] = -2. * z_far * z_near / (z_far - z_near);
    projection[(3, 2)] = -1.;
    projection[(3, 3)] = 0.;

    projection
}

fn main() {
    // Init rasterizer size
    let mut r = rst::Rasterizer::new(700, 700);

    let mut angle = 0.0;

    // camera position
    let eye_pos = Vector3::new(0., 0., 5.);

    let points = [
        (2., 0., -2.),
        (0., 2., -2.),
        (-2., 0., -2.),
        (3.5, -1., -5.),
        (2.5, 1.5, -5.),
        (-1., 0.5, -5.),
    ]
    .iter()
    .map(|&(x, y, z)| Vector3::new(x, y, z))
    .collect();

    let ind = vec![Vector3::new(0, 1, 2), Vector3::new(3, 4, 5)];

    let colors = [
        (217.0, 238.0, 185.0),
        (217.0, 238.0, 185.0),
        (217.0, 238.0, 185.0),
        (185.0, 217.0, 238.0),
        (185.0, 217.0, 238.0),
        (185.0, 217.0, 238.0),
    ]
    .iter()
    .map(|&(r, g, b)| Vector3::new(r, g, b))
    .collect();

    let pos_id = r.load_positions(points);
    let ind_id = r.load_indices(ind);
    let col_id = r.load_colors(colors);

    // keyboard input
    let mut key = 0;
    let mut frame_count = 0;

    // while running
    while key != 27 {
        // clear depth buffer and color buffer
        r.clear(rst::Buffers::Color | rst::Buffers::Depth);
        r.set_model(get_model_matrix(angle));
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrx(45., 1., 0.1, 50.));
        r.draw(&pos_id, &ind_id, &col_id, rst::Primitive::Triangle)
            .ok();

        // Assume frame_buffer is Vec<Vector3f> with RGB floats in [0,
        let mut img_data = Vec::with_capacity(700 * 700 * 3);

        for pixel in r.framebuffer() {
            img_data.push((pixel.z) as u8); // B
            img_data.push((pixel.y) as u8); // G
            img_data.push((pixel.x) as u8); // R
        }

        let mat = Mat::from_slice(&img_data).expect("Failed to create Mat from slice");
        let newsz = vec![700, 700];
        let image = mat.reshape_nd(3, &newsz).expect("Failed to reshape Mat");
        highgui::imshow("image", &image).expect("Failed to show image");
        key = highgui::wait_key(10).expect("Failed to read key");

        println!("frame count: {frame_count}");
        frame_count += 1;
        if key == ('a' as i8).into() {
            angle += 10.0;
        } else if key == ('d' as i8).into() {
            angle -= 10.0;
        }
    }
}
