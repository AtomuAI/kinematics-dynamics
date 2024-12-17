// Copyright 2024 Bewusstsein Labs

use std::{
    fmt::Debug,
    ops::{ Deref, DerefMut, Sub, Mul, Div, AddAssign }
};

use linear_algebra::vector::Vector;

use crate::{
    Assert, IsTrue,
    particle::Particle,
};

#[derive( Clone, Debug, PartialEq )]
pub struct Body<T, const DIM: usize, const ORD: usize>
where
    T: 'static + Default + Copy + Debug,
    [(); ORD + 1]:
{
    mass: T,
    particle: Particle<T, DIM, ORD>,
}

#[allow(clippy::needless_lifetimes)]
impl<T, const DIM: usize, const ORD: usize> Body<T, DIM, ORD>
where
    T: 'static + Default + Copy + Debug,
    [(); ORD + 1]:
{
    pub fn new( mass: T, spatial: [Vector<T, DIM>; ORD + 1], angular: [Vector<T, DIM>; ORD + 1]  ) -> Self {
        Self {
            mass,
            particle: Particle::new( spatial, angular )
        }
    }

    pub fn mass<'a>( &'a self ) -> &'a T { &self.mass }
    pub fn mass_mut<'b>( &'b mut self ) -> &'b mut T { &mut self.mass }
}

impl<T, const DIM: usize, const ORD: usize> Default for Body<T, DIM, ORD>
where
    T: 'static + Default + Copy + Debug,
    [(); ORD + 1]:
{
    fn default() -> Self {
        Self {
            mass: T::default(),
            particle: Particle::default()
        }
    }
}

impl<T, const DIM: usize, const ORD: usize> Deref for Body<T, DIM, ORD>
where
    T: 'static + Default + Copy + Debug,
    [(); ORD + 1]:
{
    type Target = Particle<T, DIM, ORD>;

    fn deref( &self ) -> &Self::Target {
        &self.particle
    }
}

impl<T, const DIM: usize, const ORD: usize> DerefMut for Body<T, DIM, ORD>
where
    T: 'static + Default + Copy + Debug,
    [(); ORD + 1]:
{
    fn deref_mut( &mut self ) -> &mut Self::Target {
        &mut self.particle
    }
}

pub type Body1D<T, const ORD: usize> = Body<T, 1, ORD>;
pub type Body2D<T, const ORD: usize> = Body<T, 2, ORD>;
pub type Body3D<T, const ORD: usize> = Body<T, 3, ORD>;
pub type Body4D<T, const ORD: usize> = Body<T, 4, ORD>;

#[cfg(test)]
mod tests {
    use linear_algebra::vector::Vector3;
    use super::*;

    #[test]
    fn new_test() {
        let position = Vector3::from([ 1.0, 2.0, 3.0 ]);
        let rotation = Vector3::from([ 3.0, 2.0, 1.0 ]);
        let spatial_velocity = Vector3::from([ 0.0, 0.0, 0.0 ]);
        let angular_velocity = Vector3::from([ 0.0, 0.0, 0.0 ]);
        let body = Body3D::<f32, 1>::new( 1.0, [ position, Vector3::default() ], [ rotation, Vector3::default() ] );
        assert_eq!( *body.position(), position );
        assert_eq!( *body.rotation(), rotation );
        assert_eq!( *body.spatial_velocity(), spatial_velocity );
        assert_eq!( *body.angular_velocity(), angular_velocity );
    }
}
