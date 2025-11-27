#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub const ZERO: Vector2D = Vector2D { x: 0.0, y: 0.0 };
    pub const UNIT_X: Vector2D = Vector2D { x: 1.0, y: 0.0 };
    pub const UNIT_Y: Vector2D = Vector2D { x: 0.0, y: 1.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Vector2D { x, y }
    }

    pub fn length(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn scale(&self, scale: f32) -> Self {
        Vector2D {
            x: self.x * scale,
            y: self.y * scale,
        }
    }

    // self passed by value
    // pub fn scale(self, value: f32) -> Self {
    //     Vector2D {
    //         x: self.x * value,
    //         y: self.y * value,
    //     }
    // }
}

impl std::ops::Neg for Vector2D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vector2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod tests_vector2d {
    use super::*;

    #[test]
    fn explicit_construction() {
        let v = Vector2D { x: 1.0, y: 2.0 };
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn new_method() {
        let v = Vector2D::new(1.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn default_trait() {
        let v = Vector2D::default();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
    }

    #[test]
    fn is_copyable() {
        let v = Vector2D::new(1.0, 2.0);
        let v2 = v;
        assert_eq!(v, v2);
    }

    #[test]
    fn is_clonable() {
        let v = Vector2D::new(1.0, 2.0);
        let v2 = v.clone();
        assert_eq!(v, v2);
    }

    #[test]
    fn debug_format() {
        let v = Vector2D::new(1.0, 2.0);
        assert_eq!(format!("{:?}", v), "Vector2D { x: 1.0, y: 2.0 }");
    }

    #[test]
    fn length() {
        let v = Vector2D::new(3.0, 4.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn scale() {
        let v = Vector2D::new(1.0, 2.0);
        let scaled_v = v.scale(2.0);
        assert_eq!(v, Vector2D::new(1.0, 2.0));
        assert_eq!(scaled_v, Vector2D::new(2.0, 4.0));
    }

    #[test]
    fn partial_eq() {
        let v1 = Vector2D::new(1.0, 2.0);
        let v2 = Vector2D::new(1.0, 2.0);
        assert_eq!(v1, v2);
    }

    #[test]
    fn zero_associated_constant() {
        let z = Vector2D::ZERO;
        assert_eq!(z, Vector2D::new(0.0, 0.0));
    }

    #[test]
    fn unit_vectors() {
        let unit_x: Vector2D = Vector2D::UNIT_X;
        assert!(unit_x.x == 1.0 && unit_x.y == 0.0);

        let unit_y: Vector2D = Vector2D::UNIT_Y;
        assert!(unit_y.x == 0.0 && unit_y.y == 1.0);
    }

    /*****************************************************/
    /* Operator overloads                                */
    /*****************************************************/

    #[test]
    fn negation() {
        let v = Vector2D::new(1.0, 2.0);
        let neg_v = -v;
        assert_eq!(neg_v, Vector2D::new(-1.0, -2.0));
    }

    #[test]
    fn addition() {
        let v1 = Vector2D::new(1.0, 2.0);
        let v2 = Vector2D::new(3.0, 4.0);
        let v3 = v1 + v2;
        assert_eq!(v3, Vector2D::new(4.0, 6.0));
    }

    //     #[test]
    //     fn subtraction() {
    //         let v1 = Vector2D::new(1.0, 2.0);
    //         let v2 = Vector2D::new(3.0, 4.5);
    //         let v3 = v1 - v2;

    //         assert_eq!(v3, Vector2D::new(-2.0, -2.5));
    //     }

    //     #[test]
    //     fn scalar_product() {
    //         let v1 = Vector2D::new(1.0, 2.0);
    //         let v2: Vector2D = Vector2D::new(2.0, 3.0);
    //         let product = v1 * v2;
    //         assert_eq!(product, 8.0);
    //     }

    //     #[test]
    //     fn multiplication_with_scalar() {
    //         let v = Vector2D::new(1.0, 2.0);
    //         let v_scaled = v * 2.0;
    //         assert_eq!(v_scaled, Vector2D::new(2.0, 4.0));
    //     }

    //     #[test]
    //     #[should_panic]
    //     fn index() {
    //         let v = Vector2D::new(1.0, 2.0);
    //         assert_eq!(v[0], 1.0);
    //         assert_eq!(v[1], 2.0);
    //         let _ = v[2];
    //     }

    //     #[test]
    //     fn index_mut() {
    //         let mut v = Vector2D::new(1.0, 2.0);
    //         v[0] = 3.0;
    //         v[1] = 4.0;
    //         assert_eq!(v, Vector2D::new(3.0, 4.0));
    //     }

    //     #[test]
    //     fn convert_from_tuple() {
    //         let v_tuple = (1.0, 2.0);
    //         let v = Vector2D::from(v_tuple);
    //         assert_eq!(v, Vector2D::new(1.0, 2.0));
    //     }
}
