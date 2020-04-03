use std::ops::{Add, Sub, Mul, Div};


/*------------------------------------------------------------------------------
                    Scalar Primitives
------------------------------------------------------------------------------*/
#[derive(Debug)]
pub struct Scalar<T> {value: T}
impl<T: Copy> Copy for Scalar<T>{}
impl<T: Clone> Clone for Scalar<T>{
    fn clone(&self) -> Self {
        Scalar{value: self.value.clone()}
    }
}
impl<T> Scalar<T> {
    pub fn new (value: T) -> Scalar<T> {
        Scalar{value}
    }
}


/*------------------------------------------------------------------------------
                    2D Vector (With Methods)
------------------------------------------------------------------------------*/
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Vec2<T> {
    pub u: T,
    pub v: T,
}

impl<T: Copy> Copy for Vec2<T>{}
impl<T: Clone> Clone for Vec2<T> {
    fn clone(&self) -> Self {
        Vec2 {u: self.u.clone(), v: self.v.clone()}
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self{u: self.u + other.u, v: self.v + other.v}
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self{u: self.u - other.u, v: self.v - other.v}
    }
}

impl<T: Mul<Output = T>> Mul for Vec2<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self{u: self.u * other.u, v: self.v * other.v}
    }
}

//Currently does not work. Unable to use scalar math with overloaded operators
impl<T: std::clone::Clone + Mul<Output = T>> Mul<Scalar<T>> for Vec2<T>{
    type Output = Self;
    fn mul(self, rhs: Scalar<T>) -> Self::Output {
        Self{u: self.u * rhs.value.clone(), v: self.v * rhs.value}
    }
}

impl<T: Div<Output = T>> Div for Vec2<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self{u: self.u / other.u, v: self.v / other.v}
    }
}

impl<T> Vec2<T> {
    pub fn new(u: T,
               v: T) -> Vec2<T> {
        Vec2{u, v}
    }
}

/*------------------------------------------------------------------------------
                    3D Vector (With Methods)
------------------------------------------------------------------------------*/
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Copy> Copy for Vec2<T>{}
impl<T: Clone> Clone for Vec2<T> {
    fn clone(&self) -> Self {
        Vec3 {x: self.x.clone(), y: self.y.clone(), z: self.z.clone()}
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z
    }
}

impl<T: Mul<Output = T>> Mul for Vec2<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self{x: self.x * other.x, y: self.y * other.y, z: self.z * other.z
    }
}

//Currently does not work. Unable to use scalar math with overloaded operators
impl<T: std::clone::Clone + Mul<Output = T>> Mul<Scalar<T>> for Vec2<T>{
    type Output = Self;
    fn mul(self, rhs: Scalar<T>) -> Self::Output {
        Self{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl<T: Div<Output = T>> Div for Vec2<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self{u: self.u / other.u, v: self.v / other.v}
    }
}

impl<T> Vec2<T> {
    pub fn new(u: T,
               v: T) -> Vec2<T> {
        Vec2{u, v}
    }
}
