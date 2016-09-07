use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign
};

//===============================================
// Vec3 class

#[derive(PartialEq, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    //===============================================
    pub fn length_squared(&self) -> f64 {
        return
            self.x * self.x +
            self.y * self.y +
            self.z * self.z;
    }

    //===============================================
    pub fn normalize(&self) -> Vec3 {
        return self / self.length_squared().sqrt();
    }

    //===============================================
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    //===============================================
    pub fn dot(&self, other: &Vec3) -> f64 {
        return
            self.x * other.x +
            self.y * other.y +
            self.z * other.z;
    }
}

//===============================================
impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

//===============================================
impl<'b> AddAssign<&'b Vec3> for Vec3 {
    fn add_assign(&mut self, other: &'b Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}


//===============================================
impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, other: &'b Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

//===============================================
impl<'b> SubAssign<&'b Vec3> for Vec3 {
    fn sub_assign(&mut self, other: &'b Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

//===============================================
impl<'a> Mul<f64> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, scaler: f64) -> Vec3 {
        Vec3 {
            x: self.x * scaler,
            y: self.y * scaler,
            z: self.z * scaler
        }
    }
}

//===============================================
impl<'a> Mul<&'a Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: &'a Vec3) -> Vec3 {
        Vec3 {
            x: v.x * self,
            y: v.y * self,
            z: v.z * self
        }
    }
}

//===============================================
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scaler: f64) {
        self.x *= scaler;
        self.y *= scaler;
        self.z *= scaler;
    }
}

//===============================================
impl<'a> Div<f64> for &'a Vec3 {
    type Output = Vec3;
    fn div(self, divisor: f64) -> Vec3 {
        let d = 1f64 / divisor;
        Vec3 {
            x : self.x * d,
            y : self.y * d,
            z : self.z * d
        }
    }
}

//===============================================
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, divisor: f64) {
        let d = 1f64 / divisor;
        self.x *= d;
        self.y *= d;
        self.z *= d;
    }
}

//===============================================
// Ray class

#[derive(PartialEq, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn point_at(&self, t: f64) -> Vec3 {
        return &self.origin + &(&self.dir * t);
    }
}

//===============================================
// Vec3 Unit Tests

//===============================================
#[cfg(test)]
mod vec3 {
    use super::Vec3;

    #[test]
    fn add() {
        let a = Vec3 {x: 1f64, y: 2f64, z: 3f64};
        let b = Vec3 {x: 1f64, y: 2f64, z: 3f64};
        let c = &a + &b;
        assert_eq!(c.x, 2f64);
        assert_eq!(c.y, 4f64);
        assert_eq!(c.z, 6f64);
    }

    #[test]
    fn add_assign() {
        let mut a = Vec3 {x: 1f64, y: 2f64, z: 3f64};
        let b = Vec3 {x: 1f64, y: 2f64, z: 3f64};
        a += &b;
        assert_eq!(a.x, 2f64);
        assert_eq!(a.y, 4f64);
        assert_eq!(a.z, 6f64);
    }

    #[test]
    fn sub() {
        let a = Vec3 {x: 1f64, y: 2f64, z: 3f64};
        let b = Vec3 {x: 1f64, y: 2f64, z: 3f64};
        let c = &a - &b;
        assert_eq!(c.x, 0f64);
        assert_eq!(c.y, 0f64);
        assert_eq!(c.z, 0f64);
    }

    #[test]
    fn sub_assign() {
        let mut a = Vec3 {x: 1f64, y: 2f64, z: 3f64};
        let b = Vec3 {x: 1f64, y: 2f64, z: 3f64};
        a -= &b;
        assert_eq!(a.x, 0f64);
        assert_eq!(a.y, 0f64);
        assert_eq!(a.z, 0f64);
    }

    #[test]
    fn scale() {
        let a = Vec3 {x: 1f64, y: 2f64, z: 3f64};

        let b = 3f64 * &a;
        assert_eq!(b.x, 3f64);
        assert_eq!(b.y, 6f64);
        assert_eq!(b.z, 9f64);

        let c = &a * 3f64;
        assert_eq!(c.x, 3f64);
        assert_eq!(c.y, 6f64);
        assert_eq!(c.z, 9f64);
    }

    #[test]
    fn scale_assign() {
        let mut a = Vec3 {x: 1f64, y: 2f64, z: 3f64};
        a *= 3f64;

        assert_eq!(a.x, 3f64);
        assert_eq!(a.y, 6f64);
        assert_eq!(a.z, 9f64);
    }
    #[test]
    fn dot() {
        let a = Vec3 {x: 1f64, y: 2f64, z: 3f64};
        let b = Vec3 {x: 1f64, y: 2f64, z: 3f64};
        let c = a.dot(&b);
        assert_eq!(c, 14f64);
    }

    #[test]
    fn div() {
        let a = Vec3 {x: 1f64, y: 2f64, z: 3f64};
        let b = &a / 1f64;
        assert_eq!(b.x, 1f64);
        assert_eq!(b.y, 2f64);
        assert_eq!(b.z, 3f64);
    }

    #[test]
    fn div_assign() {
        let mut a = Vec3 {x: 1f64, y: 2f64, z: 3f64};
        a /= 1f64;
        assert_eq!(a.x, 1f64);
        assert_eq!(a.y, 2f64);
        assert_eq!(a.z, 3f64);
    }

    #[test]
    fn length_squared() {
        let a = Vec3 {x: 1f64, y: 2f64, z: 3f64};
        assert_eq!(a.length_squared(), 14f64);
    }

    #[test]
    fn normalize() {
        let a = Vec3 {x: 1f64, y: 2f64, z: 3f64};
        assert_eq!(a.normalize().length_squared(), 1f64);
    }

    #[test]
    fn cross() {
        let a = Vec3 {x: 1f64, y: 10f64, z: 0f64};
        let b = Vec3 {x: 0f64, y: 23f64, z: -12f64};
        let c = a.cross(&b);
        assert_eq!(a.dot(&c), 0f64);
        assert_eq!(b.dot(&c), 0f64);
    }

}

//===============================================
// Ray Unit Tests

//===============================================
#[cfg(test)]
mod ray {
    use super::Ray;
    use super::Vec3;

    #[test]
    fn point_at() {
        let a = Vec3 {x: 1f64, y: 1f64, z: 0f64};
        let b = Vec3 {x: 0f64, y: 3f64, z: 0f64};

        let ray = Ray {
            origin: a,
            dir: b
        };

        let point = ray.point_at(2f64);

        let expected_point = Vec3 {
            x: 1f64,
            y: 7f64,
            z: 0f64
        };
        assert_eq!(point, expected_point);
    }
}
