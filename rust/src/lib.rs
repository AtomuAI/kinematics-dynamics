// Copyright 2024 Bewusstsein Labs

#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

pub mod body;
pub mod quat;
pub mod rot;
pub mod joint;
pub mod link;
pub mod constraint;
pub mod linkage;

pub struct Assert<const CHECK: bool>;
pub trait IsTrue {}
impl IsTrue for Assert<true> {}
pub trait IsFalse {}
impl IsFalse for Assert<false> {}
