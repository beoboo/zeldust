use std::ops::{Add, AddAssign, Sub, SubAssign};

use bevy::prelude::{FromReflect, Reflect};

#[derive(Clone, Copy, Debug, Reflect, FromReflect)]
pub struct Stat {
    value: u32,
    limit: u32,
    max: u32,
    cost: u32,
}

impl Stat {
    pub fn new(value: u32, limit: u32, max: u32, cost: u32) -> Self {
        Self {
            value,
            limit,
            max,
            cost,
        }
    }

    pub fn ratio_by_limit(&self) -> f32 {
        self.value as f32 / self.limit as f32
    }

    pub fn set(&mut self, val: u32) {
        self.value = val;
    }

    pub fn limit(&self) -> u32 {
        self.limit
    }

    pub fn max(&self) -> u32 {
        self.max
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn cost(&self) -> u32 {
        self.cost
    }

    pub fn upgrade(&mut self) {
        if self.limit < self.max {
            self.limit = (self.limit as f32 * 1.2).round() as u32;
            self.cost = (self.cost as f32 * 1.4).round() as u32;
        }

        if self.limit > self.max {
            self.limit = self.max;
        }
    }
}

impl Sub<u32> for Stat {
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

impl Add<u32> for Stat {
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

impl AddAssign<u32> for Stat {
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs;
    }
}

impl SubAssign<u32> for Stat {
    fn sub_assign(&mut self, rhs: u32) {
        *self = *self - rhs;
    }
}
