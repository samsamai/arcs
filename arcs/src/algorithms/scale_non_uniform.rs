use crate::{
    primitives::{Point, Line},
    Vector,
};
use kurbo::Affine;

/// Something which can be scaled **non-uniform** in x and y directions in *Drawing Space*
pub trait ScaleNonUniform {
    fn scale_nu(&mut self, factor_x: f64, factor_y: f64, base: Vector);

    fn scaled_nu(&self, factor_x: f64, factor_y: f64, base: Vector) -> Self
    where 
        Self: Sized + Clone,
        {
            let mut clone = self.clone();
            clone.scale_nu(factor_x, factor_y, base);

            clone
        }

}

impl<'t, T: ScaleNonUniform + ?Sized> ScaleNonUniform for &'t mut T {
    fn scale_nu(&mut self, factor_x: f64, factor_y: f64, base: Vector) {
        (*self).scale_nu(factor_x, factor_y, base);
    }
}

impl ScaleNonUniform for Vector {
    fn scale_nu(&mut self, factor_x: f64, factor_y: f64, base: Vector) {
        let translate_to_base = Affine::translate(base * -1.0);
        let scale = kurbo_scale_nu(factor_x, factor_y);
        let translate_back = Affine::translate(base);
        let combined_transform = translate_back * scale * translate_to_base;
        let new_pos = combined_transform * *self;
        self.x = new_pos.x;
        self.y = new_pos.y;
    }
}

impl ScaleNonUniform for Point {
    fn scale_nu(&mut self, factor_x: f64, factor_y: f64, base: Vector) {
        self.location.scale_nu(factor_x, factor_y, base);
    }
}

impl ScaleNonUniform for Line {
    fn scale_nu(&mut self, factor_x: f64, factor_y: f64, base: Vector) {
        self.start.scale_nu(factor_x, factor_y, base);
        self.end.scale_nu(factor_x, factor_y, base);
    }
}

#[inline]
const fn kurbo_scale_nu(factor_x: f64, factor_y: f64) -> Affine {
    Affine::new([factor_x, 0.0, 0.0, factor_y, 0.0, 0.0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector() {
        let original = Vector::new(-1.0, 5.0);
        let factor_x = 2.0;
        let factor_y = 2.5;

        let actual = original.scaled_nu(factor_x, factor_y, Vector::zero());
        // known value
        let expected = Vector::new(-2.0, 12.5);

        assert_eq!(actual, expected);

        let base = Vector::new(2.0, 0.0);
        let actual = original.scaled_nu(factor_x, factor_y, base);
        let expected = Vector::new(-4.0, 12.5);

        assert_eq!(actual, expected);
    }

    #[test]
    fn line() {
        let start = Vector::new(2.0, 4.0);
        let end = Vector::new(3.0, -5.0);
        let original = Line::new(start, end);
        let factor_x = 1.5;
        let factor_y = -2.0;

        let actual = original.scaled_nu(factor_x, factor_y, Vector::zero());
        let expected = Line::new(Vector::new(3.0, -8.0), Vector::new(4.5, 10.0));

        assert_eq!(actual, expected);

        // scale by line mid-point as reference
        let actual = original.scaled_nu(factor_x, factor_y, start + original.displacement() * 0.5);
        let expected = Line::new(Vector::new(1.75, -9.5), Vector::new(3.25, 8.5));

        assert_eq!(actual, expected);
    }
}