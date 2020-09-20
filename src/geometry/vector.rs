use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub dx: f32,
    pub dy: f32,
    pub dz: f32
}

impl Vector {

    pub fn default() -> Vector {
        Vector {
            dx: 0.,
            dy: 0.,
            dz: 0.
        }
    }

    pub fn norm(&self) -> f32 {
        return (self.dx * self.dx + self.dy * self.dy + self.dz * self.dz)
                .sqrt();
    }

    pub fn dxNorm(&self) -> f32 {
        return self.dx / self.norm();
    }

    pub fn dyNorm(&self) -> f32 {
        return self.dy / self.norm();
    }

    pub fn dzNorm(&self) -> f32 {
        return self.dz / self.norm();
    }

    pub fn normalized(&self) -> Vector {
        let norm = self.norm();

        return Vector {
            dx: self.dx / norm,
            dy: self.dy / norm,
            dz: self.dz / norm,
        }
    }
}

impl ops::Add<Vector> for Vector {

    type Output = Vector;

    fn add(self, v: Vector) -> Vector {
        return Vector { 
            dx: self.dx + v.dx, 
            dy: self.dy + v.dy, 
            dz: self.dz + v.dz
        };
    }
}

impl ops::Sub<Vector> for Vector {

    type Output = Vector;

    fn sub(self, v: Vector) -> Vector {
        return Vector { 
            dx: self.dx - v.dx, 
            dy: self.dy - v.dy, 
            dz: self.dz - v.dz
        };
    }
}

impl ops::Mul<Vector> for Vector {

    type Output = f32;

    fn mul(self, v: Vector) -> f32 {
        return self.dx * v.dx + self.dy * v.dy +self.dz * v.dz;

    }
}

impl ops::Mul<f32> for Vector {

    type Output = Vector;

    fn mul(self, k: f32) -> Vector {
        return Vector { 
            dx: self.dx * k, 
            dy: self.dy * k, 
            dz: self.dz * k
        };
    }
}