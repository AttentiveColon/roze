#![allow(dead_code)]

use crate::math::*;

pub struct OrthoCam {
    pub position: Vector,
    look_dir: Vector,
    up: Vector,

    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,

    projection: Matrix,
    view: Matrix,
}

impl OrthoCam {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let projection = Matrix::create_ortho(left, right, bottom, top, near, far);
        let position = (0.0, 0.0, 1.0).into();
        let look_dir = (0.0, 0.0, 0.0).into();
        let up = (0.0, 1.0, 0.0).into();
        let view = Matrix::look_at(position, look_dir, up);

        Self {
            position,
            look_dir,
            up,

            left,
            right,
            bottom,
            top,

            projection,
            view,
        }
    }

    pub fn new_center(aspect: f32, width: f32, near: f32, far: f32) -> Self {
        let height = width / aspect;

        let left = -(width / 2.0);
        let right = width / 2.0;
        let bottom = -(height / 2.0);
        let top = height / 2.0;

        let projection = Matrix::create_ortho(left, right, bottom, top, near, far);

        let position = (0.0, 0.0, 2.0).into();
        let look_dir: Vector = (0.0, 0.0, 0.0).into();
        let up = (0.0, 1.0, 0.0).into();

        let view = Matrix::look_at(position, look_dir, up);

        Self {
            position,
            look_dir,
            up,

            left,
            right,
            bottom,
            top,

            projection,
            view,
        }
    }

    pub fn get_proj_view(&self) -> (Matrix, Matrix) {
        (self.projection, self.view)
    }

    pub fn translate(&mut self, direction: Vector, amount: f32) {
        self.position += direction * amount;
        self.look_dir += direction * amount;
        self.view = Matrix::look_at(self.position, self.look_dir, self.up);
    }

    pub fn zoom(&mut self, amount: f32) {
        let aspect = self.right / self.bottom;

        self.left -= amount * aspect;
        self.right += amount * aspect;
        self.bottom += amount;
        self.top -= amount;

        self.projection =
            Matrix::create_ortho(self.left, self.right, self.bottom, self.top, 0.1, 100.0);
    }
}

#[derive(Debug)]
pub struct PerspectiveCam {
    pub position: Vector,
    look_dir: Vector,
    up: Vector,

    projection: Matrix,
    view: Matrix,
}

impl PerspectiveCam {
    pub fn new(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
        let projection = Matrix::create_perspective(fovy, aspect, near, far);

        let position = (0.0, 0.0, 0.0).into();
        let look_dir = (0.0, 0.0, -1.0).into();
        let up = (0.0, 1.0, 0.0).into();

        let view = Matrix::look_at(position, look_dir, up);

        Self {
            position,
            look_dir,
            up,

            projection,
            view,
        }
    }

    pub fn get_proj_view(&self) -> (Matrix, Matrix) {
        (self.projection, self.view)
    }

    pub fn translate(&mut self, direction: Vector, amount: f32) {
        self.position += direction * amount;
        self.look_dir += direction * amount;
        self.view = Matrix::look_at(self.position, self.look_dir, self.up);
    }
}
