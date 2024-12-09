// Copyright 2024 Bewusstsein Labs

use std::{
    fmt::Debug,
    ops::{ AddAssign, Deref, DerefMut, Mul }
};

use linear_algebra::vector::Vector;

use crate::{
    Assert, IsTrue,
    body::Body,
    constraint::Constraint
};

#[derive( Clone, Default, Debug, PartialEq )]
pub struct Joint<T, const DIM: usize, const ORD: usize>
where
    T: 'static + Default + Copy + Debug,
    [Constraint<T, DIM>; (ORD + 1) * 2]: Default,
    [(); ORD + 1]:,
    [(); (ORD + 1) * 2]:
{
    body: Body<T, DIM, ORD>,
    constraints: [Constraint<T, DIM>; (ORD + 1) * 2],
}

impl<T, const DIM: usize, const ORD: usize> Joint<T, DIM, ORD>
where
    T: 'static + Default + Copy + Debug + PartialOrd,
    [Constraint<T, DIM>; (ORD + 1) * 2]: Default,
    [(); ORD + 1]:,
    [(); (ORD + 1) * 2]:
{
    pub fn new( body: Body<T, DIM, ORD>, constraints: [Constraint<T, DIM>; (ORD + 1) * 2] ) -> Self {
        Self {
            body,
            constraints,
        }
    }

    pub fn constrain_position( &mut self ) {
        self.constraints[0].constrain( self.body.position_mut() );
    }

    pub fn constrain_rotation( &mut self ) {
        self.constraints[0].constrain( self.body.rotation_mut() );
    }

    pub fn constrain_spatial_velocity( &mut self )
    where
        Assert<{ ORD >= 1 }>: IsTrue
    {
        self.constraints[1].constrain( self.body.spatial_velocity_mut() );
    }

    pub fn constrain_angular_velocity( &mut self )
    where
        Assert<{ ORD >= 1 }>: IsTrue
    {
        self.constraints[1].constrain( self.body.angular_velocity_mut() );
    }

    pub fn constrain_spatial_acceleration( &mut self )
    where
        Assert<{ ORD >= 2 }>: IsTrue
    {
        self.constraints[1].constrain( self.body.spatial_acceleration_mut() );
    }

    pub fn constrain_angular_acceleration( &mut self )
    where
        Assert<{ ORD >= 2 }>: IsTrue
    {
        self.constraints[1].constrain( self.body.angular_acceleration_mut() );
    }

    pub fn constrain_spatial_jerk( &mut self )
    where
        Assert<{ ORD >= 3 }>: IsTrue
    {
        self.constraints[1].constrain( self.body.spatial_jerk_mut() );
    }

    pub fn constrain_angular_jerk( &mut self )
    where
        Assert<{ ORD >= 3 }>: IsTrue
    {
        self.constraints[1].constrain( self.body.angular_jerk_mut() );
    }

    pub fn constrain_spatial_snap( &mut self )
    where
        Assert<{ ORD >= 4 }>: IsTrue
    {
        self.constraints[1].constrain( self.body.spatial_snap_mut() );
    }

    pub fn constrain_angular_snap( &mut self )
    where
        Assert<{ ORD >= 4 }>: IsTrue
    {
        self.constraints[1].constrain( self.body.angular_snap_mut() );
    }

    pub fn constrain_spatial_crackle( &mut self )
    where
        Assert<{ ORD >= 5 }>: IsTrue
    {
        self.constraints[1].constrain( self.body.spatial_crackle_mut() );
    }

    pub fn constrain_angular_crackle( &mut self )
    where
        Assert<{ ORD >= 5 }>: IsTrue
    {
        self.constraints[1].constrain( self.body.angular_crackle_mut() );
    }

    pub fn constrain_spatial_pop( &mut self )
    where
        Assert<{ ORD >= 6 }>: IsTrue
    {
        self.constraints[1].constrain( self.body.spatial_pop_mut() );
    }

    pub fn constrain_angular_pop( &mut self )
    where
        Assert<{ ORD >= 6 }>: IsTrue
    {
        self.constraints[1].constrain( self.body.angular_pop_mut() );
    }

    pub fn constrain( &mut self )
    where
    T: PartialOrd,
    {
        for i in 0..=ORD {
            self.constraints[ i ].constrain( &mut self.body.position[ i ] );
        }
    }

    pub fn update( &mut self, time_step: T )
    where
        T: Mul<Output = T> + AddAssign
    {
        self.body.update( time_step );
    }
}

impl<T, const DIM: usize, const ORD: usize> Deref for Joint<T, DIM, ORD>
where
    T: 'static + Default + Copy + Debug,
    [Constraint<T, DIM>; (ORD + 1) * 2]: Default,
    [(); ORD + 1]:,
    [(); (ORD + 1) * 2]:

{
    type Target = Body<T, DIM, ORD>;

    fn deref( &self ) -> &Self::Target {
        &self.body
    }
}

impl<T, const DIM: usize, const ORD: usize> DerefMut for Joint<T, DIM, ORD>
where
    T: 'static + Default + Copy + Debug,
    [Constraint<T, DIM>; (ORD + 1) * 2]: Default,
    [(); ORD + 1]:,
    [(); (ORD + 1) * 2]:
{
    fn deref_mut( &mut self ) -> &mut Self::Target {
        &mut self.body
    }
}

pub type Joint1D<T, const ORD: usize> = Joint<T, 1, ORD>;
pub type Joint2D<T, const ORD: usize> = Joint<T, 2, ORD>;
pub type Joint3D<T, const ORD: usize> = Joint<T, 3, ORD>;
pub type Joint4D<T, const ORD: usize> = Joint<T, 4, ORD>;
