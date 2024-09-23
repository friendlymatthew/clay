use std::arch::wasm::{
    f32x4, f32x4_abs, f32x4_add, f32x4_div, f32x4_extract_lane, f32x4_ge, f32x4_gt, f32x4_le,
    f32x4_mul, f32x4_pmax, f32x4_pmin, f32x4_splat, f32x4_sqrt, f32x4_sub, i32x4_extract_lane,
    v128,
};
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct CanvasPoint(pub v128);

impl CanvasPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self(f32x4(x, y, 0.0, 0.0))
    }

    pub fn coord(self) -> (f32, f32) {
        (
            f32x4_extract_lane::<0>(self.0),
            f32x4_extract_lane::<1>(self.0),
        )
    }
}

macro_rules! impl_math {
    ($ty:ty) => {
        impl Add for $ty {
            type Output = $ty;

            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                <$ty>::from_v128(f32x4_add(self.0, rhs.0))
            }
        }

        impl Sub for $ty {
            type Output = $ty;

            fn sub(self, rhs: Self) -> Self::Output {
                <$ty>::from_v128(f32x4_sub(self.0, rhs.0))
            }
        }

        impl Mul for $ty {
            type Output = $ty;

            fn mul(self, rhs: Self) -> Self::Output {
                <$ty>::from_v128(f32x4_mul(self.0, rhs.0))
            }
        }

        impl Div for $ty {
            type Output = $ty;

            fn div(self, rhs: Self) -> Self::Output {
                <$ty>::from_v128(f32x4_div(self.0, rhs.0))
            }
        }

        impl $ty {
            #[inline(always)]
            pub fn from_v128(vector: v128) -> Self {
                Self(vector)
            }

            #[inline(always)]
            pub fn abs(self: $ty) -> $ty {
                <$ty>::from_v128(f32x4_abs(self.0))
            }

            #[inline(always)]
            fn approx_eq(self, other: Self, epsilon: f32) -> bool {
                let diff = (self - other).abs();

                diff < <$ty>::from_v128(f32x4_splat(epsilon))
            }

            #[inline(always)]
            pub fn sqrt(self) -> Self {
                <$ty>::from_v128(f32x4_sqrt(self.0))
            }

            #[inline(always)]
            pub fn min(self, other: $ty) -> Self {
                <$ty>::from_v128(f32x4_pmin(self.0, other.0))
            }

            #[inline(always)]
            pub fn max(self, other: $ty) -> Self {
                <$ty>::from_v128(f32x4_pmax(self.0, other.0))
            }

            #[inline(always)]
            pub fn le_or(self, other: $ty) -> bool {
                let res = f32x4_le(self.0, other.0);

                i32x4_extract_lane::<0>(res) | i32x4_extract_lane::<1>(res) == -1
            }

            #[inline(always)]
            pub fn one(self) -> f32 {
                f32x4_extract_lane::<0>(self.0)
            }

            #[inline(always)]
            pub fn two(self) -> f32 {
                f32x4_extract_lane::<1>(self.0)
            }

            #[inline(always)]
            pub fn sum(&self) -> f32 {
                f32x4_extract_lane::<0>(self.0) + f32x4_extract_lane::<1>(self.0)
            }

            #[inline(always)]
            pub fn euclid_dist(self, other: $ty) -> f32 {
                let diff = self - other;
                let sq_diff = diff * diff;

                sq_diff.sum().sqrt()
            }

            #[inline(always)]
            pub fn clamp(&self, min_box: $ty, max_box: $ty) -> $ty {
                <$ty>::from_v128(f32x4_pmin(f32x4_pmax(self.0, min_box.0), max_box.0))
            }

            #[inline(always)]
            pub fn midpoint(self, other: $ty) -> $ty {
                (self + other) / <$ty>::new(2.0, 2.0)
            }
        }

        impl PartialEq for $ty {
            fn eq(&self, other: &Self) -> bool {
                self.approx_eq(*other, 1e-6)
            }
        }

        impl PartialOrd for $ty {
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
    };
}

impl_math!(CanvasPoint);

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[wasm_bindgen_test]
    fn eq() {
        let p1 = CanvasPoint::new(2.4, 3.5);
        let p2 = CanvasPoint::new(2.4, 3.5);
        assert_eq!(p1, p2);

        let p3 = CanvasPoint::new(3.3, 81.0);
        assert_ne!(p1, p3);
    }

    #[wasm_bindgen_test]
    fn add_basic() {
        let p1 = CanvasPoint::new(0.0, 2.0);
        let p2 = CanvasPoint::new(3.2, 4.3);

        assert_eq!(p1 + p2, CanvasPoint::new(3.2, 6.3));
    }

    #[wasm_bindgen_test]
    fn clamp() {
        assert_eq!(
            CanvasPoint::new(34.5, 70.40),
            CanvasPoint::new(34.5, 68.9)
                .clamp(CanvasPoint::new(20.30, 70.40), CanvasPoint::new(45.0, 78.9))
        )
    }
}
