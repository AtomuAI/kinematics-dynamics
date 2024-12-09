// Copyright 2024 Bewusstsein Labs

use std::fmt::Debug;
use std::ops::{AddAssign, Mul};

use graphs::{
    graph::GraphTraits,
    undirected_graph::UnGraph
};

use crate::{
    Assert, IsTrue,
    joint::Joint,
    link::Link,
    constraint::Constraint
};

#[derive(Debug)]
pub enum Error {
    FailedToAddJoint,
    FailedToAddLink,
    FailedToRemoveJoint,
    FailedToRemoveLink
}

#[derive( Default, Debug )]
pub struct Linkage<I, T, const DIM: usize, const ORD: usize>( UnGraph<I, Joint<T, DIM, ORD>, Link<T>> )
where
    I: 'static + Default + Debug,
    T: 'static + Default + Copy + Debug,
    [Constraint<T, DIM>; (ORD + 1) * 2]: Default,
    [(); ORD + 1]:,
    [(); (ORD + 1) * 2]:;

impl <I, T, const DIM: usize, const ORD: usize> Linkage<I, T, DIM, ORD>
where
    I: 'static + Default + Copy + Debug + Ord,
    T: 'static + Default + Copy + Debug + PartialEq,
    [Constraint<T, DIM>; (ORD + 1) * 2]: Default,
    [(); ORD + 1]:,
    [(); (ORD + 1) * 2]:
{
    pub fn new() -> Self {
        Self( UnGraph::<I, Joint<T, DIM, ORD>, Link<T>>::new() )
    }

    pub fn add_joint( &mut self, id: I, joint: Joint<T, DIM, ORD> ) -> Result<(), Error> {
        self.0.add_node( id, joint ).map_err( |_| Error::FailedToAddJoint )
    }

    pub fn add_link( &mut self, nodeid1: I, nodeid2: I, link: Link<T> ) -> Result<(), Error> {
        self.0.add_edge( nodeid1, nodeid2, link ).map_err( |_| Error::FailedToAddLink )
    }

    pub fn get_joint( &self, id: I ) -> Option<&Joint<T, DIM, ORD>> {
        self.0.get_node( id )
    }

    pub fn get_joint_mut( &mut self, id: I ) -> Option<&mut Joint<T, DIM, ORD>> {
        self.0.get_node_mut( id )
    }

    pub fn get_link( &self, nodeid1: I, nodeid2: I ) -> Option<&Link<T>> {
        self.0.get_edge( nodeid1, nodeid2 )
    }

    pub fn get_link_mut( &mut self, nodeid1: I, nodeid2: I ) -> Option<&mut Link<T>> {
        self.0.get_edge_mut( nodeid1, nodeid2 )
    }

    pub fn remove_joint( &mut self, id: I ) -> Result<Joint<T, DIM, ORD>, Error> {
        self.0.remove_node( id ).map( |data| data.data().to_owned() ).map_err( |_| Error::FailedToRemoveJoint )
    }

    pub fn remove_link( &mut self, nodeid1: I, nodeid2: I ) -> Result<Link<T>, Error> {
        self.0.remove_edge( nodeid1, nodeid2 ).map_err( |_| Error::FailedToRemoveLink )
    }

    pub fn constrain_joint_positions( &mut self )
    where
        T: PartialOrd
    {
        for node in self.0.nodes_mut().iter_mut() {
            node.1.data_mut().constrain_position();
        }
    }

    pub fn constrain_joint_rotations( &mut self )
    where
        T: PartialOrd
    {
        for node in self.0.nodes_mut().iter_mut() {
            node.1.data_mut().constrain_rotation();
        }
    }

    pub fn constrain_joint_spatial_velocities( &mut self )
    where
        T: PartialOrd,
        Assert<{ ORD >= 1 }>: IsTrue
    {
        for node in self.0.nodes_mut().iter_mut() {
            node.1.data_mut().constrain_spatial_velocity();
        }
    }

    pub fn constrain_joint_angular_velocities( &mut self )
    where
        T: PartialOrd,
        Assert<{ ORD >= 1 }>: IsTrue
    {
        for node in self.0.nodes_mut().iter_mut() {
            node.1.data_mut().constrain_angular_velocity();
        }
    }

    pub fn constrain_joint_spatial_accelerations( &mut self )
    where
        T: PartialOrd,
        Assert<{ ORD >= 2 }>: IsTrue
    {
        for node in self.0.nodes_mut().iter_mut() {
            node.1.data_mut().constrain_spatial_acceleration();
        }
    }

    pub fn constrain_joint_angular_accelerations( &mut self )
    where
        T: PartialOrd,
        Assert<{ ORD >= 2 }>: IsTrue
    {
        for node in self.0.nodes_mut().iter_mut() {
            node.1.data_mut().constrain_angular_acceleration();
        }
    }

    pub fn constrain_joint_spatial_jerks( &mut self )
    where
        T: PartialOrd,
        Assert<{ ORD >= 3 }>: IsTrue
    {
        for node in self.0.nodes_mut().iter_mut() {
            node.1.data_mut().constrain_spatial_jerk();
        }
    }

    pub fn constrain_joint_angular_jerks( &mut self )
    where
        T: PartialOrd,
        Assert<{ ORD >= 3 }>: IsTrue
    {
        for node in self.0.nodes_mut().iter_mut() {
            node.1.data_mut().constrain_angular_jerk();
        }
    }

    pub fn constrain_joint_spatial_snaps( &mut self )
    where
        T: PartialOrd,
        Assert<{ ORD >= 4 }>: IsTrue
    {
        for node in self.0.nodes_mut().iter_mut() {
            node.1.data_mut().constrain_spatial_snap();
        }
    }

    pub fn constrain_joint_angular_snaps( &mut self )
    where
        T: PartialOrd,
        Assert<{ ORD >= 4 }>: IsTrue
    {
        for node in self.0.nodes_mut().iter_mut() {
            node.1.data_mut().constrain_angular_snap();
        }
    }

    pub fn constrain_joint_spatial_crackles( &mut self )
    where
        T: PartialOrd,
        Assert<{ ORD >= 5 }>: IsTrue
    {
        for node in self.0.nodes_mut().iter_mut() {
            node.1.data_mut().constrain_spatial_crackle();
        }
    }

    pub fn constrain_joint_angular_crackles( &mut self )
    where
        T: PartialOrd,
        Assert<{ ORD >= 5 }>: IsTrue
    {
        for node in self.0.nodes_mut().iter_mut() {
            node.1.data_mut().constrain_angular_crackle();
        }
    }

    pub fn constrain_joint_spatial_pops( &mut self )
    where
        T: PartialOrd,
        Assert<{ ORD >= 6 }>: IsTrue
    {
        for node in self.0.nodes_mut().iter_mut() {
            node.1.data_mut().constrain_spatial_pop();
        }
    }

    pub fn constrain_joint_angular_pops( &mut self )
    where
        T: PartialOrd,
        Assert<{ ORD >= 6 }>: IsTrue
    {
        for node in self.0.nodes_mut().iter_mut() {
            node.1.data_mut().constrain_angular_pop();
        }
    }

    pub fn constrain_joints( &mut self )
    where
        T: PartialOrd
    {
        for node in self.0.nodes_mut().iter_mut() {
            node.1.data_mut().constrain();
        }
    }

    pub fn update( &mut self, time_step: T )
    where
        T: 'static + Default + Copy + Debug + PartialEq + PartialOrd + Mul<Output = T> + AddAssign
    {
        for node in self.0.nodes_mut().iter_mut() {
            node.1.data_mut().update( time_step );
        }
    }
}

pub type Linkage1D<I, T, const ORD: usize> = Linkage<I, T, 1, ORD>;
pub type Linkage2D<I, T, const ORD: usize> = Linkage<I, T, 2, ORD>;
pub type Linkage3D<I, T, const ORD: usize> = Linkage<I, T, 3, ORD>;
pub type Linkage4D<I, T, const ORD: usize> = Linkage<I, T, 4, ORD>;

#[cfg(test)]
mod tests {
    use linear_algebra::vector::Vector3;
    use super::*;
    use crate::{
        body::Body3D,
        joint::Joint3D,
        constraint::{ Range, Constraint3D }
    };

    #[test]
    fn new_test() {
        let mut linkage = Linkage3D::<usize, f32, 6>::new();
        linkage.add_joint( 0,
            Joint3D::new(
                Body3D::new( 1.0, [ Vector3::default(); 7 ], [ Vector3::default(); 7 ] ),
                [ Constraint3D::new( [ None, None, None ] ); 14 ]
            )
        ).unwrap();
        linkage.add_joint( 1,
            Joint3D::new(
                Body3D::new( 1.0, [ Vector3::default(); 7 ], [ Vector3::default(); 7 ] ),
                [ Constraint3D::new( [ None, None, None ] ); 14 ]
            )
        ).unwrap();
        linkage.add_joint( 2,
            Joint3D::new(
                Body3D::new( 1.0, [ Vector3::default(); 7 ], [ Vector3::default(); 7 ] ),
                [ Constraint3D::new( [ None, None, None ] ); 14 ]
            )
        ).unwrap();
        linkage.add_link( 0, 1, Link::default() ).unwrap();
        linkage.add_link( 1, 2, Link::default() ).unwrap();
        linkage.constrain_joints();
        dbg!( &linkage );
    }
}
