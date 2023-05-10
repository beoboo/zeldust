use std::ops::{Add, AddAssign, Sub, SubAssign};

use bevy::prelude::Reflect;

#[derive(Clone, Copy, Reflect)]
pub struct ClampedU32 {
    value: u32,
    max: u32,
}

impl ClampedU32 {
    pub fn new(value: u32, max: u32) -> Self {
        Self { value, max }
    }

    pub fn ratio(&self) -> f32 {
        self.value as f32 / self.max as f32
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

impl Sub<u32> for ClampedU32 {
    type Output = Self;

    fn sub(mut self, rhs: u32) -> Self::Output {
        if self.value > rhs {
            self.value -= rhs;
        } else {
            self.value = 0;
        }

        self
    }
}

impl Add<u32> for ClampedU32 {
    type Output = Self;

    fn add(mut self, rhs: u32) -> Self::Output {
        if self.value + rhs <= self.max {
            self.value += rhs;
        } else {
            self.value = self.max;
        }

        self
    }
}

impl AddAssign<u32> for ClampedU32 {
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs;
    }
}

impl SubAssign<u32> for ClampedU32 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = *self - rhs;
    }
}
