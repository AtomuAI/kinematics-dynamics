// Copyright 2024 Bewusstsein Labs

use std::{
    fmt::Debug,
    ops::{ Sub, Mul, Div, AddAssign }
};

use linear_algebra::vector::Vector;

use const_expr_bounds::{ Assert, IsTrue };

#[derive( Clone, Debug, PartialEq )]
pub struct Particle<T, const DIM: usize, const ORD: usize>
where
    T: 'static + Default + Copy + Debug,
    [(); ORD + 1]:
{
    pub(crate) spatial: [Vector<T, DIM>; ORD + 1],
    pub(crate) angular: [Vector<T, DIM>; ORD + 1],
}

#[allow(clippy::needless_lifetimes)]
impl<T, const DIM: usize, const ORD: usize> Particle<T, DIM, ORD>
where
    T: 'static + Default + Copy + Debug,
    [(); ORD + 1]:
{
    pub fn new( spatial: [Vector<T, DIM>; ORD + 1], angular: [Vector<T, DIM>; ORD + 1]  ) -> Self {
        Self {
            spatial,
            angular,
        }
    }

    pub fn dim() -> usize { DIM }
    pub fn ord() -> usize { ORD }

    pub fn position<'a>( &'a self ) -> &'a Vector<T, DIM> { &self.spatial[0] }
    pub fn position_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> { &mut self.spatial[0] }
    pub fn x<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue { &self.spatial[0][0] }
    pub fn y<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue { &self.spatial[0][1] }
    pub fn z<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue { &self.spatial[0][2] }
    pub fn x_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue { &mut self.spatial[0][0] }
    pub fn y_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue { &mut self.spatial[0][1] }
    pub fn z_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue { &mut self.spatial[0][2] }

    pub fn rotation<'a>( &'a self ) -> &'a Vector<T, DIM> { &self.angular[0] }
    pub fn rotation_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> { &mut self.angular[0] }
    pub fn u<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue { &self.angular[0][0] }
    pub fn v<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue { &self.angular[0][1] }
    pub fn w<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue { &self.angular[0][2] }
    pub fn u_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue { &mut self.angular[0][0] }
    pub fn v_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue { &mut self.angular[0][1] }
    pub fn w_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue { &mut self.angular[0][2] }

    pub fn spatial_velocity<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 1 }>: IsTrue { &self.spatial[1] }
    pub fn spatial_velocity_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 1 }>: IsTrue { &mut self.spatial[1] }
    pub fn v_x<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &self.spatial[1][0] }
    pub fn v_y<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &self.spatial[1][1] }
    pub fn v_z<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &self.spatial[1][2] }
    pub fn v_x_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &mut self.spatial[1][0] }
    pub fn v_y_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &mut self.spatial[1][1] }
    pub fn v_z_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &mut self.spatial[1][2] }

    pub fn angular_velocity<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 1 }>: IsTrue { &self.angular[1] }
    pub fn angular_velocity_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 1 }>: IsTrue { &mut self.angular[1] }
    pub fn v_u<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &self.angular[1][0] }
    pub fn v_v<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &self.angular[1][1] }
    pub fn v_w<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &self.angular[1][2] }
    pub fn v_u_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &mut self.angular[1][0] }
    pub fn v_v_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &mut self.angular[1][1] }
    pub fn v_w_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 1 }>: IsTrue { &mut self.angular[1][2] }

    pub fn spatial_acceleration<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 2 }>: IsTrue { &self.spatial[2] }
    pub fn spatial_acceleration_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 2 }>: IsTrue { &mut self.spatial[2] }
    pub fn a_x<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &self.spatial[2][0] }
    pub fn a_y<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &self.spatial[2][1] }
    pub fn a_z<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &self.spatial[2][2] }
    pub fn a_x_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &mut self.spatial[2][0] }
    pub fn a_y_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &mut self.spatial[2][1] }
    pub fn a_z_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &mut self.spatial[2][2] }

    pub fn angular_acceleration<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 2 }>: IsTrue { &self.angular[2] }
    pub fn angular_acceleration_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 2 }>: IsTrue { &mut self.angular[2] }
    pub fn a_u<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &self.angular[2][0] }
    pub fn a_v<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &self.angular[2][1] }
    pub fn a_w<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &self.angular[2][2] }
    pub fn a_u_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &mut self.angular[2][0] }
    pub fn a_v_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &mut self.angular[2][1] }
    pub fn a_w_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 2 }>: IsTrue { &mut self.angular[2][2] }

    pub fn spatial_jerk<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 3 }>: IsTrue { &self.spatial[3] }
    pub fn spatial_jerk_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 3 }>: IsTrue { &mut self.spatial[3] }
    pub fn j_x<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &self.spatial[3][0] }
    pub fn j_y<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &self.spatial[3][1] }
    pub fn j_z<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &self.spatial[3][2] }
    pub fn j_x_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &mut self.spatial[3][0] }
    pub fn j_y_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &mut self.spatial[3][1] }
    pub fn j_z_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &mut self.spatial[3][2] }

    pub fn angular_jerk<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 3 }>: IsTrue { &self.angular[3] }
    pub fn angular_jerk_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 3 }>: IsTrue { &mut self.angular[3] }
    pub fn j_u<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &self.angular[3][0] }
    pub fn j_v<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &self.angular[3][1] }
    pub fn j_w<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &self.angular[3][2] }
    pub fn j_u_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &mut self.angular[3][0] }
    pub fn j_v_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &mut self.angular[3][1] }
    pub fn j_w_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 3 }>: IsTrue { &mut self.angular[3][2] }

    pub fn spatial_snap<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 4 }>: IsTrue { &self.spatial[4] }
    pub fn spatial_snap_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 4 }>: IsTrue { &mut self.spatial[4] }
    pub fn s_x<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &self.spatial[4][0] }
    pub fn s_y<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &self.spatial[4][1] }
    pub fn s_z<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &self.spatial[4][2] }
    pub fn s_x_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &mut self.spatial[4][0] }
    pub fn s_y_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &mut self.spatial[4][1] }
    pub fn s_z_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &mut self.spatial[4][2] }

    pub fn angular_snap<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 4 }>: IsTrue { &self.angular[4] }
    pub fn angular_snap_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 4 }>: IsTrue { &mut self.angular[4] }
    pub fn s_u<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &self.angular[4][0] }
    pub fn s_v<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &self.angular[4][1] }
    pub fn s_w<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &self.angular[4][2] }
    pub fn s_u_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &mut self.angular[4][0] }
    pub fn s_v_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &mut self.angular[4][1] }
    pub fn s_w_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 4 }>: IsTrue { &mut self.angular[4][2] }

    pub fn spatial_crackle<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 5 }>: IsTrue { &self.spatial[5] }
    pub fn spatial_crackle_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 5 }>: IsTrue { &mut self.spatial[5] }
    pub fn c_x<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &self.spatial[5][0] }
    pub fn c_y<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &self.spatial[5][1] }
    pub fn c_z<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &self.spatial[5][2] }
    pub fn c_x_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &mut self.spatial[5][0] }
    pub fn c_y_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &mut self.spatial[5][1] }
    pub fn c_z_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &mut self.spatial[5][2] }

    pub fn angular_crackle<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 5 }>: IsTrue { &self.angular[5] }
    pub fn angular_crackle_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 5 }>: IsTrue { &mut self.angular[5] }
    pub fn c_u<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &self.angular[5][0] }
    pub fn c_v<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &self.angular[5][1] }
    pub fn c_w<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &self.angular[5][2] }
    pub fn c_u_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &mut self.angular[5][0] }
    pub fn c_v_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &mut self.angular[5][1] }
    pub fn c_w_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 5 }>: IsTrue { &mut self.angular[5][2] }

    pub fn spatial_pop<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 6 }>: IsTrue { &self.spatial[6] }
    pub fn spatial_pop_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 6 }>: IsTrue { &mut self.spatial[6] }
    pub fn p_x<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &self.spatial[6][0] }
    pub fn p_y<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &self.spatial[6][1] }
    pub fn p_z<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &self.spatial[6][2] }
    pub fn p_x_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &mut self.spatial[6][0] }
    pub fn p_y_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &mut self.spatial[6][1] }
    pub fn p_z_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &mut self.spatial[6][2] }

    pub fn angular_pop<'a>( &'a self ) -> &'a Vector<T, DIM> where Assert<{ ORD >= 6 }>: IsTrue { &self.angular[6] }
    pub fn angular_pop_mut<'b>( &'b mut self ) -> &'b mut Vector<T, DIM> where Assert<{ ORD >= 6 }>: IsTrue { &mut self.angular[6] }
    pub fn p_u<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &self.angular[6][0] }
    pub fn p_v<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &self.angular[6][1] }
    pub fn p_w<'a>( &'a self ) -> &'a T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &self.angular[6][2] }
    pub fn p_u_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 1 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &mut self.angular[6][0] }
    pub fn p_v_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 2 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &mut self.angular[6][1] }
    pub fn p_w_mut<'b>( &'b mut self ) -> &'b mut T where Assert<{ DIM >= 3 }>: IsTrue, Assert<{ ORD >= 6 }>: IsTrue { &mut self.angular[6][2] }

    pub fn update_position( &mut self, position: Vector<T, DIM>, time_step: T )
    where
        Assert<{ ORD >= 1 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign,
    {
        *self.spatial_velocity_mut() = ( position - *self.position() ) / time_step;
        *self.position_mut() = position;
    }

    pub fn update_rotation( &mut self, rotation: Vector<T, DIM>, time_step: T )
    where
        Assert<{ ORD >= 1 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign,
    {
        *self.angular_velocity_mut() = ( rotation - *self.rotation() ) / time_step;
        *self.rotation_mut() = rotation;
    }

    pub fn update_spatial_velocity( &mut self, velocity: Vector<T, DIM>, time_step: T )
    where
        Assert<{ ORD >= 1 }>: IsTrue,
        Assert<{ ORD >= 2 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign,
    {
        *self.spatial_acceleration_mut() = ( velocity - *self.spatial_velocity() ) / time_step;
        *self.spatial_velocity_mut() = velocity;
    }

    pub fn update_angular_velocity( &mut self, velocity: Vector<T, DIM>, time_step: T )
    where
        Assert<{ ORD >= 1 }>: IsTrue,
        Assert<{ ORD >= 2 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign,
    {
        *self.angular_acceleration_mut() = ( velocity - *self.angular_velocity() ) / time_step;
        *self.angular_velocity_mut() = velocity;
    }

    pub fn update_spatial_acceleration( &mut self, acceleration: Vector<T, DIM>, time_step: T )
    where
        Assert<{ ORD >= 2 }>: IsTrue,
        Assert<{ ORD >= 3 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign,
    {
        *self.spatial_jerk_mut() = ( acceleration - *self.spatial_acceleration() ) / time_step;
        *self.spatial_acceleration_mut() = acceleration;
    }

    pub fn update_angular_acceleration( &mut self, acceleration: Vector<T, DIM>, time_step: T )
    where
        Assert<{ ORD >= 2 }>: IsTrue,
        Assert<{ ORD >= 3 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign,
    {
        *self.angular_jerk_mut() = ( acceleration - *self.angular_acceleration() ) / time_step;
        *self.angular_acceleration_mut() = acceleration;
    }

    pub fn update_spatial_jerk( &mut self, jerk: Vector<T, DIM>, time_step: T )
    where
        Assert<{ ORD >= 3 }>: IsTrue,
        Assert<{ ORD >= 4 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign,
    {
        *self.spatial_snap_mut() = ( jerk - *self.spatial_jerk() ) / time_step;
        *self.spatial_jerk_mut() = jerk;
    }

    pub fn update_angular_jerk( &mut self, jerk: Vector<T, DIM>, time_step: T )
    where
        Assert<{ ORD >= 3 }>: IsTrue,
        Assert<{ ORD >= 4 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign,
    {
        *self.angular_snap_mut() = ( jerk - *self.angular_jerk() ) / time_step;
        *self.angular_jerk_mut() = jerk;
    }

    pub fn update_spatial_snap( &mut self, snap: Vector<T, DIM>, time_step: T )
    where
        Assert<{ ORD >= 4 }>: IsTrue,
        Assert<{ ORD >= 5 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign,
    {
        *self.spatial_crackle_mut() = ( snap - *self.spatial_snap() ) / time_step;
        *self.spatial_snap_mut() = snap;
    }

    pub fn update_angular_snap( &mut self, snap: Vector<T, DIM>, time_step: T )
    where
        Assert<{ ORD >= 4 }>: IsTrue,
        Assert<{ ORD >= 5 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign,
    {
        *self.angular_crackle_mut() = ( snap - *self.angular_snap() ) / time_step;
        *self.angular_snap_mut() = snap;
    }

    pub fn update_spatial_crackle( &mut self, crackle: Vector<T, DIM>, time_step: T )
    where
        Assert<{ ORD >= 5 }>: IsTrue,
        Assert<{ ORD >= 6 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign,
    {
        *self.spatial_pop_mut() = ( crackle - *self.spatial_crackle() ) / time_step;
        *self.spatial_crackle_mut() = crackle;
    }

    pub fn update_angular_crackle( &mut self, crackle: Vector<T, DIM>, time_step: T )
    where
        Assert<{ ORD >= 5 }>: IsTrue,
        Assert<{ ORD >= 6 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign,
    {
        *self.angular_pop_mut() = ( crackle - *self.angular_crackle() ) / time_step;
        *self.angular_crackle_mut() = crackle;
    }

    pub fn update_spatial_pop( &mut self, pop: Vector<T, DIM> )
    where
        Assert<{ ORD >= 6 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign,
    {
        *self.spatial_pop_mut() = pop;
    }

    pub fn update_angular_pop( &mut self, pop: Vector<T, DIM> )
    where
        Assert<{ ORD >= 6 }>: IsTrue,
        T: 'static + Default + Copy + Debug + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + AddAssign,
    {
        *self.angular_pop_mut() = pop;
    }

    pub fn update( &mut self, time_step: T )
    where
        T: 'static + Default + Copy + Debug + Mul<Output = T> + AddAssign,
    {
        for i in ( 1..=ORD ).rev() {
            self.spatial[ i - 1 ] += self.spatial[ i ] * time_step;
            self.angular[ i - 1 ] += self.angular[ i ] * time_step;
        }
    }
}

impl<T, const DIM: usize, const ORD: usize> Default for Particle<T, DIM, ORD>
where
    T: 'static + Default + Copy + Debug,
    [(); ORD + 1]:
{
    fn default() -> Self {
        Self {
            spatial: [Vector::<T, DIM>::default(); ORD + 1],
            angular: [Vector::<T, DIM>::default(); ORD + 1],
        }
    }
}

pub type Particle1D<T, const ORD: usize> = Particle<T, 1, ORD>;
pub type Particle2D<T, const ORD: usize> = Particle<T, 2, ORD>;
pub type Particle3D<T, const ORD: usize> = Particle<T, 3, ORD>;
pub type Particle4D<T, const ORD: usize> = Particle<T, 4, ORD>;

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
        let body = Particle3D::<f32, 1>::new( [ position, Vector3::default() ], [ rotation, Vector3::default() ] );
        assert_eq!( *body.position(), position );
        assert_eq!( *body.rotation(), rotation );
        assert_eq!( *body.spatial_velocity(), spatial_velocity );
        assert_eq!( *body.angular_velocity(), angular_velocity );
    }
}
