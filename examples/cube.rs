//! 3D-Cube animation.
//!
//! <div class="warning">
//!
//! This is quite an irresponsible way of doing 3D rendering. A slightly
//! better (still very crude) version is available [here in JavaScript],
//! with some added explanation.
//!
//! The goal of this example is to render a quick cube, with real 3D
//! projection and a half-functioning camera, just to show we can.
//!
//! [here in JavaScript]: https://github.com/qrichert/painter/blob/main/demo/3d_engine.js
//!
//! </div>
//!
//! ```text
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⡠⢤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣤⠒⠒⠒⠉⠉⠁⠀⠀⢀⠇⠀⠉⠢⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⡎⠀⠑⠢⣀⠀⠀⠀⠀⠀⡎⠀⠀⠀⠀⠀⠈⠒⠤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⠀⠀⠀⠑⠢⢄⠀⡸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠑⠢⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⡜⠀⠀⠀⠀⠀⠀⠀⠀⢹⠣⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠢⢄⡀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⢀⠇⠀⠀⠀⠀⠀⠀⠀⢀⠇⠀⠀⠉⠢⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡞⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⡎⠀⠀⠀⠀⠀⠀⠉⠢⢄⠀⠀⠀⠀⠀⠀⠀⡠⡻⠁⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⣠⠜⠤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠒⢄⡀⠀⠀⡔⢡⠃⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⢰⠁⠀⠀⠀⢀⠴⠊⠀⠀⠀⠈⠑⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢲⢎⣀⠏⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⡎⠀⢀⡠⠚⠁⠀⠀⠀⠀⠀⠀⠀⠀⠈⠒⢄⡀⠀⠀⠀⠀⠀⡰⠁⢀⠎⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⢠⣣⠔⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠢⢄⠀⢀⠜⠀⡰⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠈⠢⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢩⠊⢀⠜⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠑⠢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⡠⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠒⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⢃⠜⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠢⣀⠀⠀⠀⠀⠀⠀⠀⢠⡣⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⢄⡀⠀⠀⠀⢠⡗⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠢⢄⢠⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//! ```

#![allow(clippy::cast_possible_truncation)]

use textcanvas::utils::GameLoop;
use textcanvas::TextCanvas;

type Vec2D = [f64; 2];
type Vec3D = [f64; 3];
type Vec4D = [f64; 4];
type Matrix3D = [Vec3D; 3];
type Matrix4D = [Vec4D; 4];
type CubeMesh = [[Vec3D; 4]; 6];

const CANVAS_WIDTH: f64 = 80.0;
const CANVAS_HEIGHT: f64 = 24.0;

fn main() {
    let mut canvas = TextCanvas::new(CANVAS_WIDTH as i32, CANVAS_HEIGHT as i32);

    let camera = Camera::new();
    let mut cube = Cube::new();

    GameLoop::loop_variable(&mut |delta_time| {
        canvas.clear();

        let rotate = 1.0 * delta_time;
        cube.rotate(rotate, rotate, rotate);

        let mut model = camera.project_cube(&cube);
        for face in &mut model {
            let mut first: Option<Vec2D> = None;
            let mut previous: Option<Vec2D> = None;
            for vertex in face {
                let [x, y, z] = *vertex;

                let [x, y] = world_to_screen([x, y, z]);
                *vertex = [x, y, 0.0]; // `z` gets ditched.

                if let Some(previous) = previous {
                    canvas.stroke_line(
                        previous[0].trunc() as i32,
                        previous[1].trunc() as i32,
                        x.trunc() as i32,
                        y.trunc() as i32,
                    );
                }

                if first.is_none() {
                    first = Some([x, y]);
                }

                previous = Some([x, y]);
            }

            // Close.
            if let (Some(first), Some(previous)) = (first, previous) {
                canvas.stroke_line(
                    previous[0].trunc() as i32,
                    previous[1].trunc() as i32,
                    first[0].trunc() as i32,
                    first[1].trunc() as i32,
                );
            }
        }

        // Don't eat up the whole CPU.
        std::thread::sleep(std::time::Duration::from_millis(7));

        Some(canvas.to_string())
    });
}

fn world_to_screen(point: Vec3D) -> Vec2D {
    // [-1, 1] + 1 -> [0, 2] / 2 -> [0, 1] * screen
    // Flip Y because screen coordinates are inverted.
    let [x, y, _] = point;
    let w = CANVAS_WIDTH * 2.0;
    let h = CANVAS_HEIGHT * 4.0;
    let x = ((x + 1.0) / 2.0) * w;
    let y = h - ((y + 1.0) / 2.0) * h;
    [x, y]
}

#[allow(clippy::similar_names)]
fn make_rotation_matrix(rotation: Vec3D) -> Matrix3D {
    let [gamma, beta, alpha] = rotation;

    let cosa = alpha.cos();
    let sina = alpha.sin();
    let cosb = beta.cos();
    let sinb = beta.sin();
    let cosg = gamma.cos();
    let sing = gamma.sin();

    [
        [
            cosa * cosb,
            cosa * sinb * sing - sina * cosg,
            cosa * sinb * cosg + sina * sing,
        ],
        [
            sina * cosb,
            sina * sinb * sing + cosa * cosg,
            sina * sinb * cosg - cosa * sing,
        ],
        [-sinb, cosb * sing, cosb * cosg],
    ]
}

fn make_projection_matrix(znear: f64, zfar: f64, fov: f64, ar: f64) -> Matrix4D {
    let theta = (fov / 180.0) * std::f64::consts::PI;
    let a = 1.0 / ar;
    let f = 1.0 / (theta / 2.0).tan();
    let q = zfar / (zfar - znear);

    [
        [a * f, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, q, 1.0],
        [0.0, 0.0, -znear * q, 0.0],
    ]
}

fn vector3d_x_matrix(vector: Vec3D, matrix: Matrix3D) -> Vec3D {
    let mut product = [0.0, 0.0, 0.0];
    for (col, p) in product.iter_mut().enumerate() {
        let mut sum = 0.0;
        for row in 0..3 {
            sum += vector[row] * matrix[row][col];
        }
        *p = sum;
    }
    product
}

fn vector4d_x_matrix(vector: Vec4D, matrix: Matrix4D) -> Vec4D {
    let mut product = [0.0, 0.0, 0.0, 0.0];
    for (col, p) in product.iter_mut().enumerate() {
        let mut sum = 0.0;
        for row in 0..4 {
            sum += vector[row] * matrix[row][col];
        }
        *p = sum;
    }
    product
}

struct Cube {
    mesh: CubeMesh,
    rotation: Vec3D,
    translation: Vec3D,
}

impl Cube {
    fn new() -> Self {
        Self {
            mesh: [
                // Top.
                [
                    [-0.5, 0.5, -0.5],
                    [-0.5, 0.5, 0.5],
                    [0.5, 0.5, 0.5],
                    [0.5, 0.5, -0.5],
                ],
                // Bottom.
                [
                    [0.5, -0.5, 0.5],
                    [-0.5, -0.5, 0.5],
                    [-0.5, -0.5, -0.5],
                    [0.5, -0.5, -0.5],
                ],
                // North.
                [
                    [0.5, 0.5, 0.5],
                    [-0.5, 0.5, 0.5],
                    [-0.5, -0.5, 0.5],
                    [0.5, -0.5, 0.5],
                ],
                // South.
                [
                    [-0.5, -0.5, -0.5],
                    [-0.5, 0.5, -0.5],
                    [0.5, 0.5, -0.5],
                    [0.5, -0.5, -0.5],
                ],
                // East.
                [
                    [0.5, -0.5, -0.5],
                    [0.5, 0.5, -0.5],
                    [0.5, 0.5, 0.5],
                    [0.5, -0.5, 0.5],
                ],
                // West.
                [
                    [-0.5, 0.5, 0.5],
                    [-0.5, 0.5, -0.5],
                    [-0.5, -0.5, -0.5],
                    [-0.5, -0.5, 0.5],
                ],
            ],
            rotation: [0.0, 0.0, 0.0],
            translation: [0.0, 0.0, 2.0],
        }
    }

    fn rotate(&mut self, x: f64, y: f64, z: f64) {
        self.rotation[0] += x;
        self.rotation[1] += y;
        self.rotation[2] += z;
    }

    fn transformed_model(&self) -> CubeMesh {
        let mut mesh = self.mesh;
        let rotmat = make_rotation_matrix(self.rotation);

        for face in &mut mesh {
            for vertex in face {
                let [x, y, z] = *vertex;
                let [tx, ty, tz] = self.translation;
                // Apply rotation.
                let [x, y, z] = vector3d_x_matrix([x, y, z], rotmat);
                // Apply translation.
                let [x, y, z] = [x + tx, y + ty, z + tz];
                *vertex = [x, y, z];
            }
        }

        mesh
    }
}

struct Camera {
    znear: f64,
    zfar: f64,
    fov: f64,
}

impl Camera {
    fn new() -> Self {
        Self {
            znear: 0.003, // 30mm
            zfar: 1000.0, // 1000m
            fov: 63.4,    //63.4deg ~ 35mm focal length
        }
    }

    fn project_cube(&self, cube: &Cube) -> CubeMesh {
        let mut mesh = cube.transformed_model();

        for face in &mut mesh {
            for vertex in face {
                let [x, y, z] = *vertex;
                // Apply projection.
                let [x, y, z] = self.project_vertex([x, y, z]);
                *vertex = [x, y, z];
            }
        }

        mesh
    }

    fn project_vertex(&self, vertex: Vec3D) -> Vec3D {
        let [x, y, z] = vertex;
        // [x, y, z, w]
        let vector = [x, y, z, 1.0];
        let matrix = self.get_projection_matrix();
        // [x', y', z', z]
        let [x, y, z, w] = vector4d_x_matrix(vector, matrix);
        if w == 0.0 {
            return [x, y, z];
        }
        [x / w, y / w, z / w]
    }

    fn get_projection_matrix(&self) -> Matrix4D {
        let ar = (CANVAS_WIDTH / 1.904_76) / CANVAS_HEIGHT;
        make_projection_matrix(self.znear, self.zfar, self.fov, ar)
    }
}
