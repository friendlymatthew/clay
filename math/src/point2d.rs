use std::arch::wasm::{
    f32x4, f32x4_abs, f32x4_add, f32x4_div, f32x4_extract_lane, f32x4_ge, f32x4_gt, f32x4_le,
    f32x4_mul, f32x4_splat, f32x4_sqrt, f32x4_sub, i32x4_extract_lane, v128,
};
use std::arch::wasm32::f32x4_min;
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Point2D(pub v128);

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self(f32x4(x, y, 0.0, 0.0))
    }

    #[inline]
    pub fn from_v128(vector: v128) -> Self {
        Self(vector)
    }

    pub fn sqrt(self) -> Self {
        Point2D::from_v128(f32x4_sqrt(self.0))
    }

    pub fn coord(self) -> (f32, f32) {
        (
            f32x4_extract_lane::<0>(self.0),
            f32x4_extract_lane::<1>(self.0),
        )
    }

    pub fn euclid_dist(self, other: Self) -> f32 {
        let diff = self - other;
        let diff_squared = diff * diff;
        let (x, y) = diff_squared.coord();

        x + y
    }

    pub fn abs(self) -> Self {
        Self(f32x4_abs(self.0))
    }

    fn approx_eq(self, other: Self, epsilon: f32) -> bool {
        let diff = (self - other).abs();

        diff < Point2D(f32x4_splat(epsilon))
    }

    pub fn one(self) -> f32 {
        f32x4_extract_lane::<0>(self.0)
    }

    pub fn two(self) -> f32 {
        f32x4_extract_lane::<1>(self.0)
    }

    pub fn min(self, other: Point2D) -> Point2D {
        Point2D(f32x4_min(self.0, other.0))
    }
}

impl Add for Point2D {
    type Output = Point2D;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point2D::from_v128(f32x4_add(self.0, rhs.0))
    }
}

impl Sub for Point2D {
    type Output = Point2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2D::from_v128(f32x4_sub(self.0, rhs.0))
    }
}

impl Mul for Point2D {
    type Output = Point2D;

    fn mul(self, rhs: Self) -> Self::Output {
        Point2D::from_v128(f32x4_mul(self.0, rhs.0))
    }
}

impl Div for Point2D {
    type Output = Point2D;

    fn div(self, rhs: Self) -> Self::Output {
        Point2D::from_v128(f32x4_div(self.0, rhs.0))
    }
}

impl PartialEq for Point2D {
    fn eq(&self, other: &Self) -> bool {
        self.approx_eq(*other, 1e-6)
    }
}

impl PartialOrd for Point2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.lt(other) {
            Some(Ordering::Less)
        } else if self.gt(other) {
            Some(Ordering::Greater)
        } else if self.eq(other) {
            Some(Ordering::Equal)
        } else {
            None
        }
    }

    fn lt(&self, other: &Self) -> bool {
        let res = f32x4_le(self.0, other.0);
        i32x4_extract_lane::<0>(res) & i32x4_extract_lane::<1>(res) == -1
    }

    fn le(&self, other: &Self) -> bool {
        let res = f32x4_le(self.0, other.0);
        i32x4_extract_lane::<0>(res) & i32x4_extract_lane::<1>(res) == -1
    }

    fn gt(&self, other: &Self) -> bool {
        let res = f32x4_gt(self.0, other.0);
        i32x4_extract_lane::<0>(res) & i32x4_extract_lane::<1>(res) == -1
    }

    fn ge(&self, other: &Self) -> bool {
        let res = f32x4_ge(self.0, other.0);
        i32x4_extract_lane::<0>(res) & i32x4_extract_lane::<1>(res) == -1
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[wasm_bindgen_test]
    fn eq() {
        let p1 = Point2D::new(2.4, 3.5);
        let p2 = Point2D::new(2.4, 3.5);
        assert_eq!(p1, p2);

        let p3 = Point2D::new(3.3, 81.0);
        assert_ne!(p1, p3);
    }

    #[wasm_bindgen_test]
    fn add_basic() {
        let p1 = Point2D::new(0.0, 2.0);
        let p2 = Point2D::new(3.2, 4.3);

        assert_eq!(p1 + p2, Point2D::new(3.2, 6.3));
    }
}