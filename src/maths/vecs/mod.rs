#![allow(dead_code)]

type Real = f32;

#[derive(Debug, PartialEq, PartialOrd)]
struct Vec3 {
    x: Real,
    y: Real,
    z: Real,
}

// impl<T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T> + Clone> Vec3<T> {
impl Vec3 {
    pub fn new(x: Real, y: Real, z: Real) -> Self {
        Vec3 { x, y, z }
    }

    pub fn add(&self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + vec.x,
            y: self.y + vec.y,
            z: self.z + vec.z,
        }
    }

    pub fn sub(&self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - vec.x,
            y: self.y - vec.y,
            z: self.z - vec.z,
        }
    }

    pub fn magnitud(&self) -> Real {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn scalar_mul(&self, n: Real) -> Vec3 {
        Vec3 {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }

    pub fn dot_product(&self, vec: Vec3) -> Real {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    // | i  j  k  |
    // | x  y  z  | = (y*z' - y'*z)i - (x*z' - x'*z)j + (x*y' - x'y)k
    // | x' y' z' |
    //
    pub fn cross_product(&self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * vec.z - vec.y * self.z,
            y: -(self.x * vec.z - vec.x * self.z),
            z: self.x * vec.y - vec.x * self.y,
        }
    }

    pub fn invert(&self) -> Vec3 {
        self.scalar_mul(-1.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::maths::vecs::Vec3;

    #[test]
    fn add() {
        let a = Vec3::new(3.0, 4.0, 5.0);
        let b = Vec3::new(7.0, 8.0, 9.0);

        assert_eq!(a.add(b), Vec3::new(10.0, 12.0, 14.0));
    }

    #[test]
    fn sub() {
        let a = Vec3::new(3.0, 4.0, 5.0);
        let b = Vec3::new(7.0, 8.0, 9.0);

        assert_eq!(a.sub(b), Vec3::new(-4.0, -4.0, -4.0));
    }

    #[test]
    fn scalar_prod() {
        let a = Vec3::new(3.0, 4.0, 5.0);

        assert_eq!(a.scalar_mul(2.0), Vec3::new(6.0, 8.0, 10.0))
    }

    #[test]
    fn dot_prod() {
        let a = Vec3::new(3.0, 4.0, 5.0);
        let b = Vec3::new(7.0, 8.0, 9.0);

        let res = a.dot_product(b);
        assert_eq!(res, (21 + 32 + 45) as f32);
    }

    #[test]
    fn cross_prod() {
        let a = Vec3::new(3.0, 4.0, 5.0);
        let b = Vec3::new(7.0, 8.0, 9.0);

        assert_eq!(a.cross_product(b), Vec3::new(-4.0, 8.0, -4.0));
    }

    #[test]
    fn size() {
        let a = Vec3::new(2.0, 2.0, 2.0);

        assert_eq!(a.magnitud(), 3.4641016)
    }
}
