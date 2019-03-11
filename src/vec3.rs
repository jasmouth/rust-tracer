use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn make_unit_vector(&mut self) {
        let k = 1.0_f64
            / (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2])
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn r(&self) -> f64 {
        self.e[0]
    }

    pub fn g(&self) -> f64 {
        self.e[1]
    }

    pub fn b(&self) -> f64 {
        self.e[2]
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    (v1.x() * v2.x()) + (v1.y() * v2.y()) + (v1.z() * v2.z())
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(
        v1.y() * v2.z() - v1.z() * v2.y(),
        -(v1.x() * v2.z() - v1.z() * v2.x()),
        v1.x() * v2.y() - v1.y() * v2.x(),
    )
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        };
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self {
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        };
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, c: f64) -> Self {
        Vec3::new(c * self.e[0], c * self.e[1], c * self.e[2])
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, c: f64) {
        *self = Vec3 {
            e: [self.e[0] * c, self.e[1] * c, self.e[2] * c],
        };
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3::new(self * vec.x(), self * vec.y(), self * vec.z())
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Vec3) -> Self {
        Vec3::new(
            self.e[0] / other.e[0],
            self.e[1] / other.e[1],
            self.e[2] / other.e[2],
        )
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e: [
                self.e[0] / other.e[0],
                self.e[1] / other.e[1],
                self.e[2] / other.e[2],
            ],
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, c: f64) -> Self {
        let k = 1.0_f64 / c;
        Vec3::new(self.e[0] * k, self.e[1] * k, self.e[2] * k)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, c: f64) {
        let k = 1.0_f64 / c;
        *self = Vec3 {
            e: [self.e[0] * k, self.e[1] * k, self.e[2] * k],
        };
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        &self.e[idx]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut f64 {
        &mut self.e[idx]
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}
