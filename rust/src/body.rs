// Copyright 2024 Bewusstsein Labs

use std::{
    fmt::Debug,
    ops::{ Mul, AddAssign }
};

use linear_algebra::vector::Vector;

use crate::{ Assert, IsTrue };

#[derive( Clone, Debug, PartialEq )]
pub struct Body<T, const DIM: usize, const ORD: usize>
where
    T: 'static + Default + Copy + Debug,
    [(); ORD + 1]:
{
    mass: T,
    pub(crate) position: [Vector<T, DIM>; ORD + 1],
    pub(crate) rotation: [Vector<T, DIM>; ORD + 1],
}

#[allow(clippy::needless_lifetimes)]
impl<T, const DIM: usize, const ORD: usize> Body<T, DIM, ORD>
where
    T: 'static + Default + Copy + Debug,
    [(); ORD + 1]:
{
    pub fn new( mass: T, position: [Vector<T, DIM>; ORD + 1], rotation: [Vector<T, DIM>; ORD + 1]  ) -> Self {
        Self {
            mass,
            position,
            rotation,
        }
    }

    pub fn dim() -> usize { DIM }
    pub fn ord() -> usize { ORD }

    pub fn mass<'a>( &'a self ) -> &'a T { &self.mass }
    pub fn mass_mut<'b>( &'b mut self ) -> &'b mut T { &mut self.mass }

    pub fn position<'a>( &'a self ) -> &'a Vector<T, DIM> { &self.position[0] }
    pub fn position_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> { &mut self.position[0] }
    pub fn x<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue { &self.position[0][0] }
    pub fn y<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue { &self.position[0][1] }
    pub fn z<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue { &self.position[0][2] }
    pub fn x_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue { &mut self.position[0][0] }
    pub fn y_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue { &mut self.position[0][1] }
    pub fn z_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue { &mut self.position[0][2] }

    pub fn rotation<'a>( &'a self ) -> &'a Vector<T, DIM> { &self.rotation[0] }
    pub fn rotation_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> { &mut self.rotation[0] }
    pub fn u<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue { &self.rotation[0][0] }
    pub fn v<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue { &self.rotation[0][1] }
    pub fn w<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue { &self.rotation[0][2] }
    pub fn u_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue { &mut self.rotation[0][0] }
    pub fn v_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue { &mut self.rotation[0][1] }
    pub fn w_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue { &mut self.rotation[0][2] }

    pub fn spatial_velocity<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 1 }>: IsTrue { &self.position[1] }
    pub fn spatial_velocity_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 1 }>: IsTrue { &mut self.position[1] }
    pub fn v_x<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &self.position[1][0] }
    pub fn v_y<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &self.position[1][1] }
    pub fn v_z<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &self.position[1][2] }
    pub fn v_x_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &mut self.position[1][0] }
    pub fn v_y_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &mut self.position[1][1] }
    pub fn v_z_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &mut self.position[1][2] }

    pub fn angular_velocity<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 1 }>: IsTrue { &self.rotation[1] }
    pub fn angular_velocity_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 1 }>: IsTrue { &mut self.rotation[1] }
    pub fn v_u<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &self.rotation[1][0] }
    pub fn v_v<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &self.rotation[1][1] }
    pub fn v_w<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &self.rotation[1][2] }
    pub fn v_u_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &mut self.rotation[1][0] }
    pub fn v_v_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &mut self.rotation[1][1] }
    pub fn v_w_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &mut self.rotation[1][2] }

    pub fn spatial_acceleration<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 2 }>: IsTrue { &self.position[2] }
    pub fn spatial_acceleration_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 2 }>: IsTrue { &mut self.position[2] }
    pub fn a_x<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &self.position[2][0] }
    pub fn a_y<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &self.position[2][1] }
    pub fn a_z<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &self.position[2][2] }
    pub fn a_x_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &mut self.position[2][0] }
    pub fn a_y_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &mut self.position[2][1] }
    pub fn a_z_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &mut self.position[2][2] }

    pub fn angular_acceleration<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 2 }>: IsTrue { &self.rotation[2] }
    pub fn angular_acceleration_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 2 }>: IsTrue { &mut self.rotation[2] }
    pub fn a_u<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &self.rotation[2][0] }
    pub fn a_v<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &self.rotation[2][1] }
    pub fn a_w<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &self.rotation[2][2] }
    pub fn a_u_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &mut self.rotation[2][0] }
    pub fn a_v_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &mut self.rotation[2][1] }
    pub fn a_w_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &mut self.rotation[2][2] }

    pub fn spatial_jerk<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 3 }>: IsTrue { &self.position[3] }
    pub fn spatial_jerk_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 3 }>: IsTrue { &mut self.position[3] }
    pub fn j_x<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &self.position[3][0] }
    pub fn j_y<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &self.position[3][1] }
    pub fn j_z<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &self.position[3][2] }
    pub fn j_x_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &mut self.position[3][0] }
    pub fn j_y_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &mut self.position[3][1] }
    pub fn j_z_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &mut self.position[3][2] }

    pub fn angular_jerk<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 3 }>: IsTrue { &self.rotation[3] }
    pub fn angular_jerk_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 3 }>: IsTrue { &mut self.rotation[3] }
    pub fn j_u<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &self.rotation[3][0] }
    pub fn j_v<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &self.rotation[3][1] }
    pub fn j_w<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &self.rotation[3][2] }
    pub fn j_u_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &mut self.rotation[3][0] }
    pub fn j_v_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &mut self.rotation[3][1] }
    pub fn j_w_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &mut self.rotation[3][2] }

    pub fn spatial_snap<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 4 }>: IsTrue { &self.position[4] }
    pub fn spatial_snap_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 4 }>: IsTrue { &mut self.position[4] }
    pub fn s_x<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &self.position[4][0] }
    pub fn s_y<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &self.position[4][1] }
    pub fn s_z<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &self.position[4][2] }
    pub fn s_x_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &mut self.position[4][0] }
    pub fn s_y_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &mut self.position[4][1] }
    pub fn s_z_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &mut self.position[4][2] }

    pub fn angular_snap<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 4 }>: IsTrue { &self.rotation[4] }
    pub fn angular_snap_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 4 }>: IsTrue { &mut self.rotation[4] }
    pub fn s_u<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &self.rotation[4][0] }
    pub fn s_v<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &self.rotation[4][1] }
    pub fn s_w<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &self.rotation[4][2] }
    pub fn s_u_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &mut self.rotation[4][0] }
    pub fn s_v_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &mut self.rotation[4][1] }
    pub fn s_w_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &mut self.rotation[4][2] }

    pub fn spatial_crackle<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 5 }>: IsTrue { &self.position[5] }
    pub fn spatial_crackle_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 5 }>: IsTrue { &mut self.position[5] }
    pub fn c_x<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &self.position[5][0] }
    pub fn c_y<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &self.position[5][1] }
    pub fn c_z<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &self.position[5][2] }
    pub fn c_x_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &mut self.position[5][0] }
    pub fn c_y_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &mut self.position[5][1] }
    pub fn c_z_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &mut self.position[5][2] }

    pub fn angular_crackle<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 5 }>: IsTrue { &self.rotation[5] }
    pub fn angular_crackle_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 5 }>: IsTrue { &mut self.rotation[5] }
    pub fn c_u<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &self.rotation[5][0] }
    pub fn c_v<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &self.rotation[5][1] }
    pub fn c_w<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &self.rotation[5][2] }
    pub fn c_u_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &mut self.rotation[5][0] }
    pub fn c_v_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &mut self.rotation[5][1] }
    pub fn c_w_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &mut self.rotation[5][2] }

    pub fn spatial_pop<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 6 }>: IsTrue { &self.position[6] }
    pub fn spatial_pop_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 6 }>: IsTrue { &mut self.position[6] }
    pub fn p_x<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &self.position[6][0] }
    pub fn p_y<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &self.position[6][1] }
    pub fn p_z<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &self.position[6][2] }
    pub fn p_x_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &mut self.position[6][0] }
    pub fn p_y_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &mut self.position[6][1] }
    pub fn p_z_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &mut self.position[6][2] }

    pub fn angular_pop<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 6 }>: IsTrue { &self.rotation[6] }
    pub fn angular_pop_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 6 }>: IsTrue { &mut self.rotation[6] }
    pub fn p_u<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &self.rotation[6][0] }
    pub fn p_v<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &self.rotation[6][1] }
    pub fn p_w<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &self.rotation[6][2] }
    pub fn p_u_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &mut self.rotation[6][0] }
    pub fn p_v_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &mut self.rotation[6][1] }
    pub fn p_w_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &mut self.rotation[6][2] }

    pub fn update_position( &mut self, time_step: T )
    where
        Assert<{ ORD >= 1 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Mul<Output = T> + AddAssign,
    {
        let spatial_velocity = *self.spatial_velocity();
        *self.position_mut() += spatial_velocity * time_step;
    }

    pub fn update_rotation( &mut self, time_step: T )
    where
        Assert<{ ORD >= 1 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Mul<Output = T> + AddAssign
    {
        let angular_velocity = *self.angular_velocity();
        *self.rotation_mut() += angular_velocity * time_step;
    }

    pub fn update_spatial_velocity( &mut self, time_step: T )
    where
        Assert<{ ORD >= 1 }>: IsTrue,
        Assert<{ ORD >= 2 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Mul<Output = T> + AddAssign
    {
        let spatial_acceleration = *self.spatial_acceleration();
        *self.spatial_velocity_mut() += spatial_acceleration * time_step;
    }

    pub fn update_angular_velocity( &mut self, time_step: T )
    where
        Assert<{ ORD >= 1 }>: IsTrue,
        Assert<{ ORD >= 2 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Mul<Output = T> + AddAssign
    {
        let angular_acceleration = *self.angular_acceleration();
        *self.angular_velocity_mut() += angular_acceleration * time_step;
    }

    pub fn update_spatial_acceleration( &mut self, time_step: T )
    where
        Assert<{ ORD >= 2 }>: IsTrue,
        Assert<{ ORD >= 3 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Mul<Output = T> + AddAssign
    {
        let spatial_jerk = *self.spatial_jerk();
        *self.spatial_acceleration_mut() += spatial_jerk * time_step;
    }

    pub fn update_angular_acceleration( &mut self, time_step: T )
    where
        Assert<{ ORD >= 2 }>: IsTrue,
        Assert<{ ORD >= 3 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Mul<Output = T> + AddAssign
    {
        let angular_jerk = *self.angular_jerk();
        *self.angular_acceleration_mut() += angular_jerk * time_step;
    }

    pub fn update_spatial_jerk( &mut self, time_step: T )
    where
        Assert<{ ORD >= 3 }>: IsTrue,
        Assert<{ ORD >= 4 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Mul<Output = T> + AddAssign
    {
        let spatial_snap = *self.spatial_snap();
        *self.spatial_jerk_mut() += spatial_snap * time_step;
    }

    pub fn update_angular_jerk( &mut self, time_step: T )
    where
        Assert<{ ORD >= 3 }>: IsTrue,
        Assert<{ ORD >= 4 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Mul<Output = T> + AddAssign
    {
        let angular_snap = *self.angular_snap();
        *self.angular_jerk_mut() += angular_snap * time_step;
    }

    pub fn update_spatial_snap( &mut self, time_step: T )
    where
        Assert<{ ORD >= 4 }>: IsTrue,
        Assert<{ ORD >= 5 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Mul<Output = T> + AddAssign
    {
        let spatial_crackle = *self.spatial_crackle();
        *self.spatial_snap_mut() += spatial_crackle * time_step;
    }

    pub fn update_angular_snap( &mut self, time_step: T )
    where
        Assert<{ ORD >= 4 }>: IsTrue,
        Assert<{ ORD >= 5 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Mul<Output = T> + AddAssign
    {
        let angular_crackle = *self.angular_crackle();
        *self.angular_snap_mut() += angular_crackle * time_step;
    }

    pub fn update_spatial_crackle( &mut self, time_step: T )
    where
        Assert<{ ORD >= 5 }>: IsTrue,
        Assert<{ ORD >= 6 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Mul<Output = T> + AddAssign
    {
        let spatial_pop = *self.spatial_pop();
        *self.spatial_crackle_mut() += spatial_pop * time_step;
    }

    pub fn update_angular_crackle( &mut self, time_step: T )
    where
        Assert<{ ORD >= 5 }>: IsTrue,
        Assert<{ ORD >= 6 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Mul<Output = T> + AddAssign
    {
        let angular_pop = *self.angular_pop();
        *self.angular_crackle_mut() += angular_pop * time_step;
    }

    pub fn update( &mut self, time_step: T )
    where
        T: 'static + Default + Copy + Debug + Mul<Output = T> + AddAssign
    {
        for i in ORD..0 {
            self.position[ i - 1 ] += self.position[ i ] * time_step;
            self.rotation[ i - 1 ] += self.rotation[ i ] * time_step;
        }
    }
}

impl<T, const DIM: usize, const ORD: usize> Default for Body<T, DIM, ORD>
where
    T: 'static + Default + Copy + Debug,
    [(); ORD + 1]:
{
    fn default() -> Self {
        Self {
            mass: T::default(),
            position: [Vector::<T, DIM>::default(); ORD + 1],
            rotation: [Vector::<T, DIM>::default(); ORD + 1],
        }
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
        let position = Vector3::<f32>::take([ 1.0, 2.0, 3.0 ]);
        let rotation = Vector3::<f32>::take([ 3.0, 2.0, 1.0 ]);
        let spatial_velocity = Vector3::<f32>::take([ 0.0, 0.0, 0.0 ]);
        let angular_velocity = Vector3::<f32>::take([ 0.0, 0.0, 0.0 ]);
        let body = Body3D::<f32, 1>::new( 1.0, [ position.clone(), Vector3::default() ], [ rotation.clone(), Vector3::default() ] );
        assert_eq!( *body.position(), position );
        assert_eq!( *body.rotation(), rotation );
        assert_eq!( *body.spatial_velocity(), spatial_velocity );
        assert_eq!( *body.angular_velocity(), angular_velocity );
    }
}
