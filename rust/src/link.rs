// Copyright 2024 Bewusstsein Labs

use std::fmt::Debug;
use std::marker::PhantomData;

use crate::constraint::Constraint;
use crate::joint::Joint;

#[derive( Clone, Default, Debug, PartialEq )]
pub struct Link<T, const DIM: usize>
where
    T: 'static + Default + Copy + Debug
{
    mass: T,
    constraint: Constraint<T, DIM>
}

impl<T, const DIM: usize> Link<T, DIM>
where
    T: 'static + Default + Copy + Debug
{
    pub fn new( mass: T, constraint: Constraint<T, DIM> ) -> Self {
        Self {
            mass,
            constraint
        }
    }

    /*
    pub fn constrain<const ORD: usize>( &self, joint1: &Joint<T, DIM, ORD>, joint2: &mut Joint<T, DIM, ORD>  )
    where
        T: 'static + Default + Copy + Debug + PartialEq + PartialOrd,
        [(); ORD + 1]:,
        [(); (ORD + 1) * 2]: Default
    {
        // Transform the position of joint2 into the coordinate system of joint1
        let relative_position = *joint2.position() - *joint1.position();
        let rotation_matrix = joint1.rotation().to_rotation_matrix(); // Assuming you have a method to get the rotation matrix
        let transformed_position = rotation_matrix * relative_position;

        // Create a mutable copy of the transformed position
        let mut constrained_position = transformed_position;

        // Apply the constraint to the transformed position
        self.constraint.constrain(&mut constrained_position);

        // If the position was clamped, adjust joint2's position
        if constrained_position != transformed_position {
            // Transform the constrained position back to the original coordinate system
            let corrected_position = joint1.position() + rotation_matrix.transpose() * constrained_position;
            *joint2.position_mut() = corrected_position;
        }
    }
    */
}

pub type Link1D<T> = Link<T, 1>;
pub type Link2D<T> = Link<T, 2>;
pub type Link3D<T> = Link<T, 3>;
pub type Link4D<T> = Link<T, 4>;
