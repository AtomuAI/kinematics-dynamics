// Copyright 2024 Bewusstsein Labs

use linear_algebra::vector::Vector3;

#[derive( Clone, Default )]
pub struct Body<T>
where
    T: 'static + Default + Copy
{
    position: Vector3<T>,
    rotation: Vector3<T>,
}

impl<T> Body<T>
where
    T: 'static + Default + Copy
{
    pub fn new( position: Vector3<T>, rotation: Vector3<T> ) -> Self {
        Self {
            position,
            rotation,
        }
    }

    pub fn position( &self ) -> &Vector3<T> { &self.position }
    pub fn rotation( &self ) -> &Vector3<T> { &self.rotation }
    pub fn x( &self ) -> &T { &self.position[0] }
    pub fn y( &self ) -> &T { &self.position[1] }
    pub fn z( &self ) -> &T { &self.position[2] }
    pub fn u( &self ) -> &T { &self.rotation[0] }
    pub fn v( &self ) -> &T { &self.rotation[1] }
    pub fn w( &self ) -> &T { &self.rotation[2] }
    pub fn position_mut( &mut self ) -> &mut Vector3<T> { &mut self.position }
    pub fn rotation_mut( &mut self ) -> &mut Vector3<T> { &mut self.rotation }
    pub fn x_mut( &mut self ) -> &mut T { &mut self.position[0] }
    pub fn y_mut( &mut self ) -> &mut T { &mut self.position[1] }
    pub fn z_mut( &mut self ) -> &mut T { &mut self.position[2] }
    pub fn u_mut( &mut self ) -> &mut T { &mut self.rotation[0] }
    pub fn v_mut( &mut self ) -> &mut T { &mut self.rotation[1] }
    pub fn w_mut( &mut self ) -> &mut T { &mut self.rotation[2] }
}