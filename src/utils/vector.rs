use std::{
  cmp::Ordering,
  ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

use serde::{Deserialize, Serialize};

use super::viewbox::ViewBox;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector(pub f32, pub f32);

impl Vector {
  /// zero vector
  /// 零向量
  pub const ORIGIN: Vector = Vector(0., 0.);
  /// invalid vector
  /// 無效向量
  pub const UNREACHABLE: Vector = Vector(f32::NAN, f32::NAN);
  /// create new vector
  /// 創建新向量
  pub fn new(x: f32, y: f32) -> Vector {
    Vector(x, y)
  }
  /// get vector distance
  /// 得到向量距離
  pub fn distance(self) -> f32 {
    self.distance_magnitude().sqrt()
  }
  /// get vector distance magnitude
  /// 得到向量距離^2
  pub fn distance_magnitude(self) -> f32 {
    self.0.powi(2) + self.1.powi(2)
  }
  /// get vector radian
  /// 得到向量弧度
  pub fn radian(self) -> f32 {
    let (x, y) = self.unpack();
    y.atan2(x)
  }
  /// scale vector by length
  /// 按長度縮放矢量
  pub fn by_length(self, length: f32) -> Vector {
    let distance = self.distance();
    if distance < f32::EPSILON {
      return Vector::ORIGIN;
    }
    let ratio = length / distance;
    self * ratio
  }
  /// get from self to target vector
  /// 得到從自身到目標向量
  pub fn to(self, target: Vector) -> Vector {
    target - self
  }
  /// unpack vector
  /// 打包向量
  pub fn unpack(self) -> (f32, f32) {
    (self.0, self.1)
  }
  /// anticlockwise rotation
  /// 逆時針旋轉
  pub fn rotate(self, radian: f32) -> Vector {
    let (x, y) = (self / 2.).unpack();

    let sin = radian.sin();
    let cos = radian.cos();

    let rx = x * cos - y * sin;
    let ry = x * sin + y * cos;

    Vector::new(rx, ry)
  }
  /// get left normal vector
  /// 得到左法向量
  pub fn normal(self) -> Vector {
    Vector(-self.1, self.0)
  }
  /// get right normal vector
  ///得到右法向量
  pub fn right_normal(self) -> Vector {
    Vector(self.1, -self.0)
  }
  /// get absolute vector
  /// 得到向量絕對值
  pub fn abs(self) -> Vector {
    Vector(self.0.abs(), self.1.abs())
  }
  /// get signum (1 or -1)
  /// 得到正負(1 or -1)
  pub fn signum(self) -> Vector {
    let (x, y) = self.unpack();
    Vector(x.signum(), y.signum())
  }
  /// get vector dot product
  /// 得到向量點積
  pub fn dot(self, target: Vector) -> f32 {
    let concat = target * self;
    concat.0 + concat.1
  }
  /// get vector cross product
  /// 得到向量叉積  
  /// Self X Target  
  /// value > 0, Anticlockwise rotation  
  /// value < 0, Clockwise rotation  
  /// value = 0, Parallel  
  pub fn cross(self, target: Vector) -> f32 {
    return self.0 * target.1 - self.1 * target.0;
  }
  /// get orthoprojection of target on self
  /// 得到目標對自身的正射投影
  pub fn orthoprojection(self, target: Vector) -> Vector {
    self * (self.dot(target) / self.distance().powi(2))
  }
  /// get orthoprojection length of target on self
  /// 得到目標對自身的正射投影長度
  pub fn orthoprojection_length(self, target: Vector) -> f32 {
    self.dot(target) / self.distance()
  }

  /// get max
  pub fn max(self, target: Vector) -> Vector {
    Vector(self.0.max(target.0), self.1.max(target.1))
  }

  /// get min
  pub fn min(self, target: Vector) -> Vector {
    Vector(self.0.min(target.0), self.1.min(target.1))
  }

  /// 無條件捨去
  pub fn floor(self) -> Vector {
    let (x, y) = self.unpack();
    Vector(x.floor(), y.floor())
  }

  /// 無條件進位
  pub fn ceil(self) -> Vector {
    let (x, y) = self.unpack();
    Vector(x.ceil(), y.ceil())
  }

  /// 四捨五入
  pub fn round(self) -> Vector {
    let (x, y) = self.unpack();
    Vector(x.round(), y.round())
  }
}

impl Default for Vector {
  fn default() -> Self {
    Vector::ORIGIN
  }
}


//
// calc vector with vector
// 用向量計算向量
impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
      Self (-self.0,-self.1)
    }
}

impl Add for Vector {
  type Output = Vector;
  fn add(self, rhs: Self) -> Self::Output {
    let x = self.0 + rhs.0;
    let y = self.1 + rhs.1;
    Vector(x, y)
  }
}

impl AddAssign for Vector {
  fn add_assign(&mut self, other: Self) {
    *self = *self + other
  }
}

impl Sub for Vector {
  type Output = Vector;
  fn sub(self, rhs: Self) -> Self::Output {
    let x = self.0 - rhs.0;
    let y = self.1 - rhs.1;
    Vector(x, y)
  }
}

impl SubAssign for Vector {
  fn sub_assign(&mut self, other: Self) {
    *self = *self - other
  }
}

impl Mul for Vector {
  type Output = Vector;
  fn mul(self, rhs: Vector) -> Self::Output {
    Vector(self.0 * rhs.0, self.1 * rhs.1)
  }
}

impl MulAssign for Vector {
  fn mul_assign(&mut self, other: Self) {
    *self = *self * other
  }
}

impl Div for Vector {
  type Output = Vector;
  fn div(self, rhs: Vector) -> Self::Output {
    Vector(self.0 / rhs.0, self.1 / rhs.1)
  }
}

impl DivAssign for Vector {
  fn div_assign(&mut self, other: Self) {
    *self = *self / other
  }
}

impl Rem for Vector {
  type Output = Vector;
  fn rem(self, rhs: Vector) -> Self::Output {
    Vector(self.0 % rhs.0, self.1 % rhs.1)
  }
}

impl RemAssign for Vector {
  fn rem_assign(&mut self, rhs: Vector) {
    *self = *self % rhs
  }
}

impl PartialEq for Vector {
  fn eq(&self, other: &Vector) -> bool {
    self.0 == other.0 && self.1 == other.1
  }
}

impl PartialOrd for Vector {
  fn partial_cmp(&self, other: &Vector) -> Option<Ordering> {
    let (x, y) = self.unpack();
    let (ox, oy) = other.unpack();
    let x = x.partial_cmp(&ox);
    let y = y.partial_cmp(&oy);
    if x == y {
      x
    } else {
      None
    }
  }
}

//
// Support f32
// 支持 f32

impl Add<f32> for Vector {
  type Output = Vector;
  fn add(self, rhs: f32) -> Self::Output {
    Vector(self.0 + rhs, self.1 + rhs)
  }
}

impl AddAssign<f32> for Vector {
  fn add_assign(&mut self, rhs: f32) {
    *self = *self + rhs
  }
}

impl Sub<f32> for Vector {
  type Output = Vector;
  fn sub(self, rhs: f32) -> Self::Output {
    Vector(self.0 - rhs, self.1 - rhs)
  }
}

impl SubAssign<f32> for Vector {
  fn sub_assign(&mut self, rhs: f32) {
    *self = *self - rhs
  }
}

impl Mul<f32> for Vector {
  type Output = Vector;
  fn mul(self, rhs: f32) -> Self::Output {
    Vector(self.0 * rhs, self.1 * rhs)
  }
}

impl MulAssign<f32> for Vector {
  fn mul_assign(&mut self, rhs: f32) {
    *self = *self * rhs
  }
}

impl Div<f32> for Vector {
  type Output = Vector;
  fn div(self, rhs: f32) -> Self::Output {
    Vector(self.0 / rhs, self.1 / rhs)
  }
}

impl DivAssign<f32> for Vector {
  fn div_assign(&mut self, rhs: f32) {
    *self = *self / rhs
  }
}

impl Rem<f32> for Vector {
  type Output = Vector;
  fn rem(self, rhs: f32) -> Self::Output {
    Vector(self.0 % rhs, self.1 % rhs)
  }
}

impl RemAssign<f32> for Vector {
  fn rem_assign(&mut self, rhs: f32) {
    *self = *self % rhs
  }
}

//
//
//
impl ViewBox for Vector {
  fn angle(&self) -> f32 {
    0.0
  }

  fn size(&self) -> Vector {
    Vector::ORIGIN
  }

  fn position(&self) -> Vector {
    self.clone()
  }
}
