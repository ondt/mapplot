use std::fmt::{self, Debug, Display, Formatter};

use crate::{BoundingBox, Location};


pub(crate) trait FormatterExt<'f> {
	fn write_object<'a>(&'a mut self) -> JavaScriptObject<'a, 'f>;
}


impl<'f> FormatterExt<'f> for Formatter<'f> {
	fn write_object<'a>(&'a mut self) -> JavaScriptObject<'a, 'f> {
		JavaScriptObject::new(self)
	}
}


pub(crate) struct JavaScriptObject<'a, 'f> {
	fmt: &'a mut Formatter<'f>,
	result: fmt::Result,
	pending_comma: bool,
}


impl<'a, 'f> JavaScriptObject<'a, 'f> {
	fn new(fmt: &'a mut Formatter<'f>) -> Self {
		let result = fmt.write_str("{ ");
		JavaScriptObject { fmt, result, pending_comma: false }
	}
	
	pub(crate) fn entry(&mut self, key: &str, value: &impl JavaScript) -> &mut Self {
		self.entry_opt(key, &Some(value))
	}
	
	pub(crate) fn entry_opt(&mut self, key: &str, value: &Option<impl JavaScript>) -> &mut Self {
		self.result = self.result.and_then(|_| {
			if let Some(value) = value {
				if self.pending_comma {
					self.fmt.write_str(", ")?;
				}
				
				self.fmt.write_str(key)?;
				self.fmt.write_str(": ")?;
				value.fmt_js(self.fmt)?;
				
				self.pending_comma = true;
			}
			Ok(())
		});
		
		self
	}
	
	pub(crate) fn finish(&mut self) -> fmt::Result {
		self.result.and_then(|_| self.fmt.write_str(" }"))
	}
}


////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


pub(crate) trait JavaScript {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result;
}


macro_rules! literal_default {
    ($($t:ty)*) => ($(
        impl JavaScript for $t {
            fn fmt_js(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                Display::fmt(self, fmt)
            }
        }
    )*)
}

literal_default! { bool u8 f32 f64 usize isize }


impl<R: JavaScript> JavaScript for &R {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		(*self).fmt_js(f)
	}
}


impl JavaScript for &str {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		// TODO: replace '\n' and stuff
		write!(f, "\"{}\"", self)
	}
}


impl JavaScript for String {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		// TODO: replace '\n' and stuff
		write!(f, "\"{}\"", self)
	}
}


impl<T: JavaScript> JavaScript for Vec<T> {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str("[")?;
		let mut first = true;
		for item in self {
			if first {
				first = false;
			} else {
				f.write_str(", ")?;
			}
			item.fmt_js(f)?;
		}
		f.write_str("]")?;
		Ok(())
	}
}


impl JavaScript for Location {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		// https://developers.google.com/maps/documentation/javascript/reference/coordinates#LatLngLiteral
		write!(f, "{{ lat: {}, lng: {} }}", self.lat, self.lon)
	}
}


impl JavaScript for BoundingBox {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str("new google.maps.LatLngBounds(")?;
		self.p1.fmt_js(f)?;
		f.write_str(", ")?;
		self.p2.fmt_js(f)?;
		f.write_str(")")?;
		Ok(())
	}
}


////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug, Copy, Clone)]
pub(crate) struct RawIdent<'a>(pub(crate) &'a str);


impl<'a> JavaScript for RawIdent<'a> {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		Display::fmt(self.0, f)
	}
}
