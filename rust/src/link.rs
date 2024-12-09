// Copyright 2024 Bewusstsein Labs

use std::fmt::Debug;
use std::marker::PhantomData;

#[derive( Clone, Default, Debug, PartialEq )]
pub struct Link<T>
where
    T: 'static + Default + Copy + Debug
{
    mass: T,
    length: T
}

impl<T> Link<T>
where
    T: 'static + Default + Copy + Debug
{
    pub fn new( mass: T, length: T ) -> Self {
        Self {
            mass,
            length
        }
    }
}
