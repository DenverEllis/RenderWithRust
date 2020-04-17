use std::ops::{Add, Sub, Mul, Div};

/*------------------------------------------------------------------------------
                    Scalar Primitives
------------------------------------------------------------------------------*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Scalar<T: Clone> {
    pub value: T,
}

impl <T: Clone + Add<Output = T>> Add for Scalar<T> {
    type Output = Self;
    fn add(self, rhs: Self)->Self {
        Scalar{value: self.value + rhs.value}
    }
}

impl <T: Clone + Sub<Output = T>> Sub for Scalar<T> {
    type Output = Self;
    fn sub(self, rhs: Self)->Self {
        Scalar{value: self.value - rhs.value}
    }
}

impl <T: Clone + Mul<Output = T>> Mul for Scalar<T> {
    type Output = Self;
    fn mul(self, rhs: Self)->Self {
        Scalar{value: self.value * rhs.value}
    }
}

impl <T: Clone + Div<Output = T>> Div for Scalar<T> {
    type Output = Self;
    fn div(self, rhs: Self)->Self {
        Scalar{value: self.value / rhs.value}
    }
}

impl<T: Clone> Scalar<T> {
    pub fn new(value: T) -> Scalar<T> {
        Scalar{value}
    }
}


/*------------------------------------------------------------------------------
                    2D Vector (With Methods)
------------------------------------------------------------------------------*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2<T: Clone> {
    pub x: Scalar<T>,
    pub y: Scalar<T>,
}

impl<T: Clone + Add<Output = T>> Add for Vec2<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self{x: self.x + other.x, y: self.y + other.y}
    }
}

impl<T: Clone + Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self{x: self.x - other.x, y: self.y - other.y}
    }
}

impl<T: Clone + Mul<Output = T>> Mul for Vec2<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self{x: self.x * other.x, y: self.y * other.y}
    }
}

impl<T: Clone + Mul<Output = T>> Mul<Scalar<T>> for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: Scalar<T>) -> Self::Output {
        Self{x: self.x * rhs.clone(), y: self.y * rhs}
    }
}

impl<T: Clone + Div<Output = T>> Div for Vec2<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self{x: self.x / other.x, y: self.y / other.y}
    }
}

impl<T: Clone + Div<Output = T>> Div<Scalar<T>> for Vec2<T> {
    type Output = Self;
    fn div(self, rhs: Scalar<T>) -> Self::Output {
        Self{x: self.x / rhs.clone(), y: self.y / rhs}
    }
}

impl<T: Clone> Vec2<T> {
    pub fn new(x: Scalar<T>,
               y: Scalar<T>) -> Vec2<T> {
        Vec2{x, y}
    }
}

/*------------------------------------------------------------------------------
                    3D Vector (With Methods)
------------------------------------------------------------------------------*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T: Clone> {
    pub x: Scalar<T>,
    pub y: Scalar<T>,
    pub z: Scalar<T>,
}

impl<T: Clone + Add<Output = T>> Add for Vec3<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl<T: Clone + Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl<T: Clone + Mul<Output = T>> Mul for Vec3<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self{x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }
}

impl<T: Clone + Mul<Output = T>> Mul<Scalar<T>> for Vec3<T>{
    type Output = Self;
    fn mul(self, rhs: Scalar<T>) -> Self::Output {
        Self{x: self.x * rhs.clone(), y: self.y * rhs.clone(), z: self.z * rhs}
    }
}

impl<T: Clone + Div<Output = T>> Div for Vec3<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self{x: self.x / other.x, y: self.y / other.y, z: self.z / other.z}
    }
}

impl<T: Clone + Div<Output = T>> Div<Scalar<T>> for Vec3<T>{
    type Output = Self;
    fn div(self, rhs: Scalar<T>) -> Self::Output {
        Self{x: self.x / rhs.clone(), y: self.y / rhs.clone(), z: self.z / rhs}
    }
}

impl<T: Clone> Vec3<T> {
    pub fn new(x: Scalar<T>,
               y: Scalar<T>,
               z: Scalar<T>) -> Vec3<T> {
        Vec3{x, y, z}
    }
}



/*------------------------------------------------------------------------------
                    4D Vector (With Methods)
------------------------------------------------------------------------------*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec4<T: Clone> {
    pub x: Scalar<T>,
    pub y: Scalar<T>,
    pub z: Scalar<T>,
    pub w: Scalar<T>,
}

impl<T: Clone + Add<Output = T>> Add for Vec4<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self{x: self.x + other.x,
             y: self.y + other.y,
             z: self.z + other.z,
             w: self.w + other.w}
    }
}

impl<T: Clone + Sub<Output = T>> Sub for Vec4<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self{x: self.x - other.x,
             y: self.y - other.y,
             z: self.z - other.z,
             w: self.w - other.w}
    }
}

impl<T: Clone + Mul<Output = T>> Mul for Vec4<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self{x: self.x * other.x,
             y: self.y * other.y,
             z: self.z * other.z,
             w: self.w * other.w}
    }
}

impl<T: Clone + Mul<Output = T>> Mul<Scalar<T>> for Vec4<T>{
    type Output = Self;
    fn mul(self, rhs: Scalar<T>) -> Self::Output {
        Self{x: self.x * rhs.clone(),
             y: self.y * rhs.clone(),
             z: self.z * rhs.clone(),
             w: self.w}
    }
}

impl<T: Clone + Div<Output = T>> Div for Vec4<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self{x: self.x / other.x,
             y: self.y / other.y,
             z: self.z / other.z,
             w: self.w / other.w}
    }
}


//Currently does not work. Unable to use scalar math with overloaded operators
impl<T: Clone + Div<Output = T>> Div<Scalar<T>> for Vec4<T>{
    type Output = Self;
    fn div(self, rhs: Scalar<T>) -> Self::Output {
        Self{x: self.x / rhs.clone(),
             y: self.y / rhs.clone(),
             z: self.z / rhs.clone(),
             w: self.w}
    }
}

impl Vec4<f32> {
    fn norm(&self) -> f32 {
        (self.x * self.x +
         self.y * self.y +
         self.z * self.z +
         self.w * self.w ).value.sqrt()
    }

    fn normalize(&self) -> Vec4<f32> {
        let length: Scalar::<f32> = Scalar::new(Vec4::norm(self));
        Self{x: self.x / length,
             y: self.y / length,
             z: self.z / length,
             w: self.w / length}
    }
}

impl<T: Clone> Vec4<T> {
    pub fn new(x: Scalar<T>,
               y: Scalar<T>,
               z: Scalar<T>,
               w: Scalar<T>) -> Vec4<T> {
        Vec4{x, y, z, w}
    }
}

pub type Vec2i = Vec2<i32>;
pub type Vec2f = Vec2<f32>;
pub type Vec3i = Vec3<i32>;
pub type Vec3f = Vec3<f32>;