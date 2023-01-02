use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Matrix {
    pub c0: Vector,
    pub c1: Vector,
    pub c2: Vector,
    pub c3: Vector,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn mag(v: Vector) -> f32 {
        f32::sqrt(v.x * v.x + v.y * v.y + v.z * v.z)
    }

    pub fn normalize(v: Vector) -> Self {
        let mag = Vector::mag(v);
        v / mag
    }

    pub fn dot(v1: Vector, v2: Vector) -> f32 {
        (v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z)
    }

    pub fn cross(v1: Vector, v2: Vector) -> Vector {
        Self {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
            w: 1.0,
        }
    }

    pub fn unit_x() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn unit_y() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn unit_z() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 0.0,
        }
    }

    pub fn zeroed() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn lerp(to: Vector, from: Vector, t: f32) -> Vector {
        to + (from - to) * t
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        let s = 1.0 / rhs;

        Vector::new(self.x * s, self.y * s, self.z * s, 1.0)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
        self.w = self.w + rhs.w;
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
        self.w = self.w - rhs.w;
    }
}

impl From<(f32, f32, f32)> for Vector {
    fn from(value: (f32, f32, f32)) -> Self {
        Vector::new(value.0, value.1, value.2, 1.0)
    }
}

impl From<(f32, f32, f32, f32)> for Vector {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Vector::new(value.0, value.1, value.2, value.3)
    }
}

pub mod float {
    pub fn lerp(to: f32, from: f32, t: f32) -> f32 {
        to + (from - to) * t
    }
}

impl Matrix {
    pub fn new(c0: Vector, c1: Vector, c2: Vector, c3: Vector) -> Self {
        Self { c0, c1, c2, c3 }
    }

    pub fn new_identity() -> Self {
        Self {
            c0: Vector::new(1.0, 0.0, 0.0, 0.0),
            c1: Vector::new(0.0, 1.0, 0.0, 0.0),
            c2: Vector::new(0.0, 0.0, 1.0, 0.0),
            c3: Vector::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn transpose(&self) -> Matrix {
        let mut m = *self;
        std::mem::swap(&mut m.c0.y, &mut m.c1.x);
        std::mem::swap(&mut m.c0.z, &mut m.c2.x);
        std::mem::swap(&mut m.c0.w, &mut m.c3.x);

        std::mem::swap(&mut m.c1.z, &mut m.c2.y);
        std::mem::swap(&mut m.c1.w, &mut m.c3.y);

        std::mem::swap(&mut m.c2.w, &mut m.c3.z);
        m
    }

    pub fn determinant(&self) -> f32 {
        let d = self.c3.x
            * (self.c0.y * (self.c1.z * self.c2.w - self.c2.z * self.c1.w)
                - self.c1.y * (self.c0.z * self.c2.w - self.c2.z * self.c0.w)
                + self.c2.y * (self.c0.z * self.c1.w - self.c1.z * self.c0.w));

        let c = self.c2.x
            * (self.c0.y * (self.c1.z * self.c3.w - self.c3.z * self.c1.w)
                - self.c1.y * (self.c0.z * self.c3.w - self.c3.z * self.c0.w)
                + self.c3.y * (self.c0.z * self.c1.w - self.c1.z * self.c0.w));

        let b = self.c1.x
            * (self.c0.y * (self.c2.z * self.c3.w - self.c3.z * self.c2.w)
                - self.c2.y * (self.c0.z * self.c3.w - self.c3.z * self.c0.w)
                + self.c3.y * (self.c0.z * self.c2.w - self.c2.z * self.c0.w));

        let a = self.c0.x
            * (self.c1.y * (self.c2.z * self.c3.w - self.c3.z * self.c2.w)
                - self.c2.y * (self.c1.z * self.c3.w - self.c3.z * self.c1.w)
                + self.c3.y * (self.c1.z * self.c2.w - self.c2.z * self.c1.w));

        a - b + c - d
    }

    #[rustfmt::skip]
    pub fn invert(&self) -> Option<Matrix> {
        let mut inv = [0.0f32; 16];

        inv[0] =   self.c1.y * self.c2.z * self.c3.w -
                   self.c1.y * self.c3.z * self.c2.w -
                   self.c1.z * self.c2.y * self.c3.w +
                   self.c1.z * self.c3.y * self.c2.w +
                   self.c1.w * self.c2.y * self.c3.z -
                   self.c1.w * self.c3.y * self.c2.z;

        inv[4] =  -self.c0.y * self.c2.z * self.c3.w +
                   self.c0.y * self.c3.z * self.c2.w +
                   self.c0.z * self.c2.y * self.c3.w -
                   self.c0.z * self.c3.y * self.c2.w -
                   self.c0.w * self.c2.y * self.c3.z +
                   self.c0.w * self.c3.y * self.c2.z;

        inv[8] =   self.c0.y * self.c1.z * self.c3.w -
                   self.c0.y * self.c3.z * self.c1.w -
                   self.c0.z * self.c1.y * self.c3.w +
                   self.c0.z * self.c3.y * self.c1.w +
                   self.c0.w * self.c1.y * self.c3.z -
                   self.c0.w * self.c3.y * self.c1.z;

        inv[12] = -self.c0.y * self.c1.z * self.c2.w +
                   self.c0.y * self.c2.z * self.c1.w +
                   self.c0.z * self.c1.y * self.c2.w -
                   self.c0.z * self.c2.y * self.c1.w -
                   self.c0.w * self.c1.y * self.c2.z +
                   self.c0.w * self.c2.y * self.c1.z;

        inv[1] =  -self.c1.x * self.c2.z * self.c3.w +
                   self.c1.x * self.c3.z * self.c2.w +
                   self.c1.z * self.c2.x * self.c3.w -
                   self.c1.z * self.c3.x * self.c2.w -
                   self.c1.w * self.c2.x * self.c3.z +
                   self.c1.w * self.c3.x * self.c2.z;

        inv[5] =   self.c0.x * self.c2.z * self.c3.w -
                   self.c0.x * self.c3.z * self.c2.w -
                   self.c0.z * self.c2.x * self.c3.w +
                   self.c0.z * self.c3.w * self.c2.w +
                   self.c0.w * self.c2.x * self.c3.z -
                   self.c0.w * self.c3.w * self.c2.z;

        inv[9] =  -self.c0.x * self.c1.z * self.c3.w +
                   self.c0.x * self.c3.z * self.c1.w +
                   self.c0.z * self.c1.x * self.c3.w -
                   self.c0.z * self.c3.x * self.c1.w -
                   self.c0.w * self.c1.x * self.c3.z +
                   self.c0.w * self.c3.x * self.c1.z;

        inv[13] =  self.c0.x * self.c1.z * self.c2.w -
                   self.c0.x * self.c2.z * self.c1.w -
                   self.c0.z * self.c1.x * self.c2.w +
                   self.c0.z * self.c2.x * self.c1.w +
                   self.c0.w * self.c1.x * self.c2.z -
                   self.c0.w * self.c2.x * self.c1.z;

        inv[2] =   self.c1.x * self.c2.y * self.c3.w -
                   self.c1.x * self.c3.y * self.c2.w -
                   self.c1.y * self.c2.x * self.c3.w +
                   self.c1.y * self.c3.x * self.c2.w +
                   self.c1.w * self.c2.x * self.c3.y -
                   self.c1.w * self.c3.x * self.c2.y;

        inv[6] =  -self.c0.x * self.c2.y * self.c3.w +
                   self.c0.x * self.c3.y * self.c2.w +
                   self.c0.y * self.c2.x * self.c3.w -
                   self.c0.y * self.c3.x * self.c2.w -
                   self.c0.w * self.c2.x * self.c3.y +
                   self.c0.w * self.c3.x * self.c2.y;

        inv[10] =  self.c0.x * self.c1.y * self.c3.w -
                   self.c0.x * self.c3.y * self.c1.w -
                   self.c0.y * self.c1.x * self.c3.w +
                   self.c0.y * self.c3.x * self.c1.w +
                   self.c0.w * self.c1.x * self.c3.y -
                   self.c0.w * self.c3.x * self.c1.y;

        inv[14] = -self.c0.x * self.c1.y * self.c2.w +
                   self.c0.x * self.c2.y * self.c1.w +
                   self.c0.y * self.c1.x * self.c2.w -
                   self.c0.y * self.c2.x * self.c1.w -
                   self.c0.w * self.c1.x * self.c2.y +
                   self.c0.w * self.c2.x * self.c1.y;

        inv[3] =  -self.c1.x * self.c2.y * self.c3.z +
                   self.c1.x * self.c3.y * self.c2.z +
                   self.c1.y * self.c2.x * self.c3.z -
                   self.c1.y * self.c3.x * self.c2.z -
                   self.c1.z * self.c2.x * self.c3.y +
                   self.c1.z * self.c3.x * self.c2.y;

        inv[7] =   self.c0.x * self.c2.y * self.c3.z -
                   self.c0.x * self.c3.y * self.c2.z -
                   self.c0.y * self.c2.x * self.c3.z +
                   self.c0.y * self.c3.x * self.c2.z +
                   self.c0.z * self.c2.x * self.c3.y -
                   self.c0.z * self.c3.x * self.c2.y;

        inv[11] = -self.c0.x * self.c1.y * self.c3.z +
                   self.c0.x * self.c3.y * self.c1.z +
                   self.c0.y * self.c1.x * self.c3.z -
                   self.c0.y * self.c3.x * self.c1.z -
                   self.c0.z * self.c1.x * self.c3.y +
                   self.c0.z * self.c3.x * self.c1.y;

        inv[15] =  self.c0.x * self.c1.y * self.c2.z -
                   self.c0.x * self.c2.y * self.c1.z -
                   self.c0.y * self.c1.x * self.c2.z +
                   self.c0.y * self.c2.x * self.c1.z +
                   self.c0.z * self.c1.x * self.c2.y -
                   self.c0.z * self.c2.x * self.c1.y;

        let mut det = self.c0.x * inv[0] + self.c1.x * inv[4] + self.c2.x * inv[8] + self.c3.x * inv[12];

        if det == 0.0 {
            None
        } else {
            det = 1.0 / det;
            let mut res: [f32; 16] = [0.0; 16];
            for i in 0..16 {
                res[i] = inv[i] * det;
            }
            let c0 = Vector::new(res[0], res[4], res[8], res[12]);
            let c1 = Vector::new(res[1], res[5], res[9], res[13]);
            let c2 = Vector::new(res[2], res[6], res[10], res[14]);
            let c3 = Vector::new(res[3], res[7], res[11], res[15]);

            Some(Matrix {c0, c1, c2, c3})
        }

    }

    pub fn negate(&self) -> Matrix {
        let mut m = *self;
        m.c0.x = -m.c0.x;
        m.c1.x = -m.c1.x;
        m.c2.x = -m.c2.x;
        m.c3.x = -m.c3.x;

        m.c0.y = -m.c0.y;
        m.c1.y = -m.c1.y;
        m.c2.y = -m.c2.y;
        m.c3.y = -m.c3.y;

        m.c0.z = -m.c0.z;
        m.c1.z = -m.c1.z;
        m.c2.z = -m.c2.z;
        m.c3.z = -m.c3.z;

        m.c0.w = -m.c0.w;
        m.c1.w = -m.c1.w;
        m.c2.w = -m.c2.w;
        m.c3.w = -m.c3.w;
        m
    }

    pub fn create_trs(t: Matrix, r: Matrix, s: Matrix) -> Matrix {
        Matrix::mxm(Matrix::mxm(t, r), s)
    }

    pub fn create_mvp(proj: Matrix, view: Matrix, model: Matrix) -> Matrix {
        Matrix::mxm(proj, Matrix::mxm(view, model))
    }

    pub fn mxv(m: Matrix, v: Vector) -> Vector {
        Vector {
            x: m.c0.x * v.x + m.c1.x * v.y + m.c2.x * v.z + m.c3.x * v.w,
            y: m.c0.y * v.x + m.c1.y * v.y + m.c2.y * v.z + m.c3.y * v.w,
            z: m.c0.z * v.x + m.c1.z * v.y + m.c2.z * v.z + m.c3.z * v.w,
            w: m.c0.w * v.x + m.c1.w * v.y + m.c2.w * v.z + m.c3.w * v.w,
        }
    }

    pub fn mxm(m1: Matrix, m2: Matrix) -> Matrix {
        let c0 = Vector::new(
            m1.c0.x * m2.c0.x + m1.c1.x * m2.c0.y + m1.c2.x * m2.c0.z + m1.c3.x * m2.c0.w,
            m1.c0.y * m2.c0.x + m1.c1.y * m2.c0.y + m1.c2.y * m2.c0.z + m1.c3.y * m2.c0.w,
            m1.c0.z * m2.c0.x + m1.c1.z * m2.c0.y + m1.c2.z * m2.c0.z + m1.c3.z * m2.c0.w,
            m1.c0.w * m2.c0.x + m1.c1.w * m2.c0.y + m1.c2.w * m2.c0.z + m1.c3.w * m2.c0.w,
        );

        let c1 = Vector::new(
            m1.c0.x * m2.c1.x + m1.c1.x * m2.c1.y + m1.c2.x * m2.c1.z + m1.c3.x * m2.c1.w,
            m1.c0.y * m2.c1.x + m1.c1.y * m2.c1.y + m1.c2.y * m2.c1.z + m1.c3.y * m2.c1.w,
            m1.c0.z * m2.c1.x + m1.c1.z * m2.c1.y + m1.c2.z * m2.c1.z + m1.c3.z * m2.c1.w,
            m1.c0.w * m2.c1.x + m1.c1.w * m2.c1.y + m1.c2.w * m2.c1.z + m1.c3.w * m2.c1.w,
        );

        let c2 = Vector::new(
            m1.c0.x * m2.c2.x + m1.c1.x * m2.c2.y + m1.c2.x * m2.c2.z + m1.c3.x * m2.c2.w,
            m1.c0.y * m2.c2.x + m1.c1.y * m2.c2.y + m1.c2.y * m2.c2.z + m1.c3.y * m2.c2.w,
            m1.c0.z * m2.c2.x + m1.c1.z * m2.c2.y + m1.c2.z * m2.c2.z + m1.c3.z * m2.c2.w,
            m1.c0.w * m2.c2.x + m1.c1.w * m2.c2.y + m1.c2.w * m2.c2.z + m1.c3.w * m2.c2.w,
        );

        let c3 = Vector::new(
            m1.c0.x * m2.c3.x + m1.c1.x * m2.c3.y + m1.c2.x * m2.c3.z + m1.c3.x * m2.c3.w,
            m1.c0.y * m2.c3.x + m1.c1.y * m2.c3.y + m1.c2.y * m2.c3.z + m1.c3.y * m2.c3.w,
            m1.c0.z * m2.c3.x + m1.c1.z * m2.c3.y + m1.c2.z * m2.c3.z + m1.c3.z * m2.c3.w,
            m1.c0.w * m2.c3.x + m1.c1.w * m2.c3.y + m1.c2.w * m2.c3.z + m1.c3.w * m2.c3.w,
        );

        Matrix::new(c0, c1, c2, c3)
    }

    pub fn create_translation_matrix(v: Vector) -> Matrix {
        let mut mat = Matrix::new_identity();
        mat.c3.x = v.x;
        mat.c3.y = v.y;
        mat.c3.z = v.z;
        mat
    }

    pub fn create_scaling_matrix(v: Vector) -> Matrix {
        let mut mat = Matrix::new_identity();
        mat.c0.x = v.x;
        mat.c1.y = v.y;
        mat.c2.z = v.z;
        mat
    }

    pub fn create_rotation_matrix(degrees: f32) -> Matrix {
        let angle = degrees.to_radians();
        let mut mat = Matrix::new_identity();
        mat.c0.x = f32::cos(angle);
        mat.c1.x = f32::sin(angle);
        mat.c0.y = -f32::sin(angle);
        mat.c1.y = f32::cos(angle);
        mat
    }

    pub fn look_at(eye: Vector, dir: Vector, up: Vector) -> Matrix {
        let f = Vector::normalize(dir - eye);
        let s = Vector::normalize(Vector::cross(f, up));
        let u = Vector::cross(s, f);

        Matrix {
            c0: (s.x, u.x, -f.x, 0.0).into(),
            c1: (s.y, u.y, -f.y, 0.0).into(),
            c2: (s.z, u.z, -f.z, 0.0).into(),
            c3: (
                -Vector::dot(eye, s),
                -Vector::dot(eye, u),
                Vector::dot(eye, f),
                1.0,
            )
                .into(),
        }
    }

    pub fn create_perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Matrix {
        let rfovy = fovy.to_radians();
        let f = 1.0 / f32::tan(rfovy / 2.0);

        let c0 = Vector::new(f / aspect, 0.0, 0.0, 0.0);
        let c1 = Vector::new(0.0, f, 0.0, 0.0);
        let c2 = Vector::new(0.0, 0.0, (far + near) / (near - far), -1.0);
        let c3 = Vector::new(0.0, 0.0, (2.0 * far * near) / (near - far), 0.0);

        Matrix { c0, c1, c2, c3 }
    }

    pub fn create_ortho(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Matrix {
        let c0 = Vector::new(2.0 / (right - left), 0.0, 0.0, 0.0);
        let c1 = Vector::new(0.0, 2.0 / (top - bottom), 0.0, 0.0);
        let c2 = Vector::new(0.0, 0.0, -2.0 / (far - near), 0.0);
        let c3 = Vector::new(
            -(right + left) / (right - left),
            -(top + bottom) / (top - bottom),
            -(far + near) / (far - near),
            1.0,
        );

        Matrix { c0, c1, c2, c3 }
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n|{:11}, {:11}, {:11}, {:11}|\n|{:11}, {:11}, {:11}, {:11}|\n|{:11}, {:11}, {:11}, {:11}|\n|{:11}, {:11}, {:11}, {:11}|",
            self.c0.x,
            self.c1.x,
            self.c2.x,
            self.c3.x,
            self.c0.y,
            self.c1.y,
            self.c2.y,
            self.c3.y,
            self.c0.z,
            self.c1.z,
            self.c2.z,
            self.c3.z,
            self.c0.w,
            self.c1.w,
            self.c2.w,
            self.c3.w
        )
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Matrix::mxv(self, rhs)
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix::mxm(self, rhs)
    }
}

impl Neg for Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        Matrix::negate(&self)
    }
}
