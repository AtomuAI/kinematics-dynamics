// Copyright 2024 Bewusstsein Labs

use std::{
    fmt::Debug,
    ops::Deref
};
use num::traits::clamp;

use linear_algebra::vector::Vector;

#[derive( Clone, Copy, Default, Debug, PartialEq )]
pub struct Range<T> {
    min: T,
    max: T
}

impl<T> Range<T>
where
    T: Copy
{
    pub fn new( min: T, max: T ) -> Self {
        Self { min, max }
    }

    pub fn min( &self ) -> &T {
        &self.min
    }

    pub fn max( &self ) -> &T {
        &self.max
    }

    pub fn clamp( &self, value: &mut T )
    where
        T: PartialOrd
    {
        *value = clamp( *value, *self.min(), *self.max() )
    }
}

#[derive( Clone, Copy, Default, Debug, PartialEq )]
pub struct Constraint<T, const DIM: usize>( Vector<Option<Range<T>>, DIM> )
where
    T: 'static + Default + Copy + Debug;

impl<T, const DIM: usize> Constraint<T, DIM>
where
    T: 'static + Default + Copy + Debug + PartialOrd
{
    pub fn new( options: [ Option<Range<T>>; DIM ] ) -> Self {
        Self( Vector::from( options ) )
    }

    pub fn constrain( &self, vec: &mut Vector<T, DIM>)
    {
        for i in 0..DIM {
            if let Some( range ) = &self[i] {
                range.clamp( &mut vec[i] );
            }
        }
    }
}

impl<T, const DIM: usize> Deref for Constraint<T, DIM>
where
    T: 'static + Default + Copy + Debug
{
    type Target = Vector<Option<Range<T>>, DIM>;

    fn deref( &self ) -> &Self::Target {
        &self.0
    }
}

pub type Constraint1D<T> = Constraint<T, 1>;
pub type Constraint2D<T> = Constraint<T, 2>;
pub type Constraint3D<T> = Constraint<T, 3>;
pub type Constraint4D<T> = Constraint<T, 4>;
