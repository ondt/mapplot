use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use crate::hijack_formatter;


trait FormatterExt<'f> {
	fn write_object<'a>(&'a mut self) -> JavaScriptObject<'a, 'f>;
}


impl<'f> FormatterExt<'f> for Formatter<'f> {
	fn write_object<'a>(&'a mut self) -> JavaScriptObject<'a, 'f> {
		JavaScriptObject::new(self)
	}
}


struct JavaScriptObject<'a, 'f> {
	fmt: &'a mut Formatter<'f>,
	result: fmt::Result,
	pending_comma: bool,
}


impl<'a, 'f> JavaScriptObject<'a, 'f> {
	fn new(fmt: &'a mut Formatter<'f>) -> Self {
		let result = fmt.write_str("{ ");
		JavaScriptObject { fmt, result, pending_comma: false }
	}
	
	fn entry(&mut self, key: &str, value: &impl JavaScript) -> &mut Self {
		self.entry_maybe(key, &Some(value))
	}
	
	fn entry_maybe(&mut self, key: &str, value: &Option<impl JavaScript>) -> &mut Self {
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
	
	fn finish(&mut self) -> fmt::Result {
		self.result.and_then(|_| self.fmt.write_str(" }"))
	}
}


trait JavaScript: Sized {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result;
	
	fn to_stringg(&self) -> String {
		hijack_formatter(|f| self.fmt_js(f))
	}
}


macro_rules! render {
    ($($t:ty)*) => ($(
        impl JavaScript for $t {
            fn fmt_js(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                Display::fmt(self, fmt)
            }
        }
    )*)
}

render! { u8 f64 isize }


impl<R: JavaScript> JavaScript for &R {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		(*self).fmt_js(f)
	}
}


// string literal
impl JavaScript for &str {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		// TODO: replace '\n' and stuff
		write!(f, "\"{}\"", self)
	}
}


// string literal
impl JavaScript for String {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		// TODO: replace '\n' and stuff
		write!(f, "\"{}\"", self)
	}
}


const MAP_IDENT: RawIdent<'static> = RawIdent("__map");


struct RawIdent<'a>(&'a str);


impl<'a> JavaScript for RawIdent<'a> {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		Display::fmt(self.0, f)
	}
}


impl<'a> Display for RawIdent<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		Display::fmt(self.0, f)
	}
}


#[derive(Debug, Copy, Clone)]
pub enum MapType {
	/// A normal street map.
	Roadmap,
	/// Satellite images.
	Satellite,
	/// A transparent layer of major streets on satellite images.
	Hybrid,
	/// Maps with physical features such as terrain and vegetation.
	Terrain,
}


impl JavaScript for MapType {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			MapType::Roadmap => f.write_str("google.maps.MapTypeId.ROADMAP"),
			MapType::Satellite => f.write_str("google.maps.MapTypeId.SATELLITE"),
			MapType::Hybrid => f.write_str("google.maps.MapTypeId.HYBRID"),
			MapType::Terrain => f.write_str("google.maps.MapTypeId.TERRAIN"),
		}
	}
}


#[derive(Debug)]
pub struct GoogleMap {
	apikey: String,
	center: LatLng,
	zoom: u8,
	map_type: MapType,
	title: Option<String>,
	markers: Vec<Marker>,
}


impl GoogleMap {
	// TODO: auto center & zoom
	pub fn new(center: impl Into<LatLng>, zoom: u8, map_type: MapType, apikey: impl AsRef<str>) -> Self {
		GoogleMap {
			apikey: apikey.as_ref().to_string(),
			center: center.into(),
			zoom,
			map_type,
			title: None,
			markers: Vec::default(),
		}
	}
	
	pub fn title(&mut self, title: impl AsRef<str>) -> &mut Self {
		self.title = Some(String::from(title.as_ref()));
		self
	}
	
	pub fn marker(&mut self, marker: Marker) -> &mut Self {
		self.markers.push(marker);
		self
	}
	
	pub fn markers(&mut self, markers: impl IntoIterator<Item=Marker>) -> &mut Self {
		self.markers.extend(markers.into_iter());
		self
	}
}


impl JavaScript for GoogleMap {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, r#"		var {} = new google.maps.Map(document.getElementById("map_canvas"), "#, MAP_IDENT)?;
		f.write_object()
			.entry("center", &self.center)
			.entry("zoom", &self.zoom)
			.entry("mapTypeId", &self.map_type)
			.finish()?;
		f.write_str(");\n\n")?;
		
		for marker in &self.markers {
			f.write_str("\t\t")?;
			marker.fmt_js(f)?;
			f.write_str(";\n")?;
		}
		
		Ok(())
	}
}


impl Display for GoogleMap {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, r#"
<html>
<head>
<meta name="viewport" content="initial-scale=1.0, user-scalable=no" />
<meta http-equiv="content-type" content="text/html; charset=UTF-8"/>
<title>{title}</title>
<script type="text/javascript" src="https://maps.googleapis.com/maps/api/js?libraries=visualization&sensor=true_or_false&key={apikey}"></script>
<script type="text/javascript">
	function initialize() {{
"#, title = if let Some(t) = &self.title { t.as_str() } else { "Default Title" }, apikey = self.apikey)?;
		
		self.fmt_js(f)?;
		
		write!(f, r#"
	}}
</script>
</head>
<body style="margin:0px; padding:0px;" onload="initialize()">
	<div id="map_canvas" style="width: 100%; height: 100%;"></div>
</body>
</html>
"#)
	}
}


#[derive(Debug)]
pub struct Marker {
	position: LatLng,
	label: Option<String>,
	title: Option<String>,
	opacity: Option<f64>,
	z_index: Option<isize>,
}


impl Marker {
	#[must_use]
	pub fn new(lat: f64, lng: f64) -> Self {
		Marker {
			position: LatLng { lat, lon: lng },
			label: None,
			title: None,
			opacity: None,
			z_index: None,
		}
	}
	
	pub fn label(mut self, value: impl AsRef<str>) -> Self {
		self.label = Some(value.as_ref().to_string());
		self
	}
}


impl JavaScript for Marker {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str("new google.maps.Marker(")?;
		f.write_object()
			.entry("map", &MAP_IDENT)
			.entry("position", &self.position)
			.entry_maybe("label", &self.label)
			.finish()?;
		f.write_str(")")?;
		Ok(())
	}
}


#[derive(Debug, Copy, Clone)]
pub struct LatLng {
	lat: f64,
	lon: f64,
}


impl From<(f64, f64)> for LatLng {
	fn from((lat, lon): (f64, f64)) -> Self {
		LatLng { lat, lon }
	}
}


impl JavaScript for LatLng {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "new google.maps.LatLng({}, {})", self.lat, self.lon)
	}
}
