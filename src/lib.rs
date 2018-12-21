#![feature(trait_alias)]

// This import shows good and normal unresolved import message
// use crate::NON_EXISTENT;

// This import causes panic!
use crate::NON_EXISTENT::ANY_GIBERISH_HERE_CAUSES_PANIC;

