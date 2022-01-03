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
#![allow(
	clippy::semicolon_if_nothing_returned,
	clippy::module_name_repetitions,
	clippy::multiple_crate_versions
)]

use std::fmt;
use std::fmt::{Display, Formatter};

pub mod google;
pub mod image;

fn hijack_formatter(f: impl Fn(&mut Formatter<'_>) -> fmt::Result) -> String {
	struct Wrapper<F>(F)
	where
		F: Fn(&mut Formatter<'_>) -> fmt::Result;
	impl<F> Display for Wrapper<F>
	where
		F: Fn(&mut Formatter<'_>) -> fmt::Result,
	{
		fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
			self.0(formatter)
		}
	}

	format!("{}", Wrapper(f))
}

#[derive(Debug, Copy, Clone)]
pub struct Location {
	lat: f64,
	lon: f64,
}

impl Location {
	#[must_use]
	pub fn new(lat: f64, lon: f64) -> Self {
		Location { lat, lon }
	}
}

impl From<(f64, f64)> for Location {
	fn from((lat, lon): (f64, f64)) -> Self {
		Location { lat, lon }
	}
}

// TODO: AsRef?
impl From<&(f64, f64)> for Location {
	fn from((lat, lon): &(f64, f64)) -> Self {
		Location {
			lat: *lat,
			lon: *lon,
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
	p1: Location,
	p2: Location,
}

fn proj(p: impl Into<Location>, zoom: u8) -> Mercator {
	let p = p.into();
	let (x, y) = googleprojection::from_ll_to_subpixel(&(p.lon, p.lat), zoom as usize).unwrap();
	Mercator { x, y }
}

#[derive(Debug, Copy, Clone)]
pub struct Mercator {
	x: f64,
	y: f64,
}

impl BoundingBox {
	#[must_use]
	pub fn new(p1: impl Into<Location>, p2: impl Into<Location>) -> Self {
		let (p1, p2) = (p1.into(), p2.into());
		BoundingBox { p1, p2 }
	}
}
