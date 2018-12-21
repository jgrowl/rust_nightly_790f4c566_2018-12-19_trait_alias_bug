#![feature(trait_alias)]

pub mod identifiable1;
pub mod thing;

use crate::identifiable1::Identifiable as IdentifiableBase;

pub trait Identifiable = IdentifiableBase;
