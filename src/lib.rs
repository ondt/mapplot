#![deny(unused_must_use)]
#![deny(unsafe_code)]

#![warn(rust_2018_idioms)]
#![warn(nonstandard_style)]
#![warn(future_incompatible)]
#![warn(macro_use_extern_crate)]
#![warn(explicit_outlives_requirements)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(unused_crate_dependencies)]

#![warn(clippy::pedantic)]

#![allow(dead_code)]
#![allow(clippy::semicolon_if_nothing_returned, clippy::module_name_repetitions, clippy::multiple_crate_versions)]


use std::fmt;
use std::fmt::{Display, Formatter};


pub mod google;


fn hijack_formatter(f: impl Fn(&mut Formatter<'_>) -> fmt::Result) -> String {
	struct Wrapper<F>(F) where F: Fn(&mut Formatter<'_>) -> fmt::Result;
	impl<F> Display for Wrapper<F> where F: Fn(&mut Formatter<'_>) -> fmt::Result {
		fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
			self.0(formatter)
		}
	}
	
	format!("{}", Wrapper(f))
}
