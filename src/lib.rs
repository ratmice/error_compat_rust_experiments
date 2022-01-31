#![cfg_attr(not(test), no_std)]
#![allow(non_snake_case, unused_imports, dead_code)]
#![feature(try_trait_v2, termination_trait_lib)]

// Each of these defines a type Error
// Their respective implementations are mutually exclusive
// Union and Intersection are very similar,
// but Intersection can't just use the one defined in Union,
// because of differences in the trait implementations.


// This implementation just uses the existing seL4_Error with minor modifications
// where rust provides an alternate mechanism.
pub mod union;

// This implementation *uses* unsafe in a few places,
// This is ultimitely not required, there are safe mechanisms for this.
//
// But I have included partial checking of the safety invariants at compile time.
// We can do more in this regard, including generating JustError from Error automagically.
pub mod intersection;

// This union based implementation is the worst.
//
// It requires a lot of unsafe, and the resulting ergonomics when you use it is also bad.
// the Mirai checker gives some warnings on this code, but I'm fairly confident these
// are actually false positives, but this code is pretty terrible regardless.
// I would never want to be subjected to it, nor anyone else for that matter.
// It is left here for completeness sake, so that others may be saved the expense of repeating it.

// pub mod disjoint;
