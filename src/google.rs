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


#[derive(Debug)]
pub struct GoogleMap {
	apikey: String,
	center: LatLng,
	zoom: u8,
	map_type: MapType,
	title: Option<String>,
	markers: Vec<Marker>,
	circles: Vec<Circle>,
	rectangles: Vec<Rectangle>,
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
			circles: Vec::default(),
			rectangles: Vec::default(),
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
	
	pub fn circle(&mut self, circle: Circle) -> &mut Self {
		self.circles.push(circle);
		self
	}
	
	pub fn circles(&mut self, circles: impl IntoIterator<Item=Circle>) -> &mut Self {
		self.circles.extend(circles.into_iter());
		self
	}
	
	pub fn rectangle(&mut self, rectangle: Rectangle) -> &mut Self {
		self.rectangles.push(rectangle);
		self
	}
	
	pub fn rectangles(&mut self, rectangles: impl IntoIterator<Item=Rectangle>) -> &mut Self {
		self.rectangles.extend(rectangles.into_iter());
		self
	}
}


impl JavaScript for GoogleMap {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, r#"		const {} = new google.maps.Map(document.getElementById("map_canvas"), "#, MAP_IDENT)?;
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
		
		f.write_str("\n")?;
		
		for circle in &self.circles {
			f.write_str("\t\t")?;
			circle.fmt_js(f)?;
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


#[derive(Default, Debug, Copy, Clone)]
struct CommonOptions {
	// // TODO: this would have no effect
	// clickable: Option<bool>,
	draggable: Option<bool>,
	// TODO: Marker has everything but this
	editable: Option<bool>,
	visible: Option<bool>,
	z_index: Option<isize>,
}


#[derive(Default, Debug, Copy, Clone)]
struct StrokeOptions {
	stroke_color: Option<Color>,
	stroke_opacity: Option<f32>,
	stroke_weight: Option<usize>,
}


#[derive(Default, Debug, Copy, Clone)]
struct FillOptions {
	fill_color: Option<Color>,
	fill_opacity: Option<f32>,
	stroke_position: Option<StrokePosition>,
}


#[derive(Debug, Copy, Clone)]
pub enum Color {
	RGB(u8, u8, u8),
	RGBA(u8, u8, u8, u8),
	HSL(u16, u8, u8),
	HSLA(u16, u8, u8, u8),
}


impl JavaScript for Color {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Color::RGB(r, g, b) => write!(f, "\"#{:02x}{:02x}{:02x}\"", r, g, b),
			Color::RGBA(r, g, b, a) => write!(f, "\"#{:02x}{:02x}{:02x}{:02x}\"", r, g, b, a),
			Color::HSL(h, s, l) => write!(f, "\"hsl({h}, {s}%, {l}%)\"", h = h, s = 100.0 * f64::from(*s) / 255.0, l = 100.0 * f64::from(*l) / 255.0),
			Color::HSLA(h, s, l, a) => write!(f, "\"hsla({}, {}%, {}%, {}%)\"", h = h, s = 100.0 * f64::from(*s) / 255.0, l = 100.0 * f64::from(*l) / 255.0, a = 100.0 * f64::from(*a) / 255.0),
		}
	}
}


#[derive(Debug, Copy, Clone)]
pub enum StrokePosition {
	/// The stroke is centered on the polygon's path, with half the stroke inside the polygon and half the stroke outside the polygon.
	Center,
	/// The stroke lies inside the polygon.
	Inside,
	/// The stroke lies outside the polygon.
	Outside,
}


impl JavaScript for StrokePosition {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			StrokePosition::Center => f.write_str("google.maps.StrokePosition.CENTER"),
			StrokePosition::Inside => f.write_str("google.maps.StrokePosition.INSIDE"),
			StrokePosition::Outside => f.write_str("google.maps.StrokePosition.OUTSIDE"),
		}
	}
}


#[derive(Debug, Copy, Clone)]
pub struct LatLng {
	lat: f64,
	lon: f64,
}


impl LatLng {
	#[must_use]
	pub fn new(lat: f64, lon: f64) -> Self {
		LatLng { lat, lon }
	}
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


#[derive(Debug, Copy, Clone)]
pub struct LatLngBounds {
	sw: LatLng,
	ne: LatLng,
}


impl LatLngBounds {
	#[must_use]
	pub fn new(p1: LatLng, p2: LatLng) -> Self {
		// TODO: correction
		LatLngBounds {
			sw: p1,
			ne: p2,
		}
	}
}


impl JavaScript for LatLngBounds {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str("new google.maps.LatLngBounds(")?;
		self.sw.fmt_js(f)?;
		f.write_str(", ")?;
		self.ne.fmt_js(f)?;
		f.write_str(")")?;
		Ok(())
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


/// A circle on the Earth's surface; also known as a "spherical cap".
///
/// # Examples
/// ```
/// todo!()
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Circle {
	center: LatLng,
	radius: f64,
	fill: FillOptions,
	stroke: StrokeOptions,
	common: CommonOptions,
}


impl Circle {
	/// Create a new circle.
	///
	/// # Arguments
	/// * `lat`: Latitude in degrees.
	/// * `lon`: Longitude in degrees.
	/// * `radius`: The radius in meters on the Earth's surface.
	#[must_use]
	pub fn new(lat: f64, lon: f64, radius: f64) -> Self {
		Circle {
			center: LatLng { lat, lon },
			radius,
			fill: FillOptions::default(),
			stroke: StrokeOptions::default(),
			common: CommonOptions::default(),
		}
	}
	
	/// Set both `fill_color` and `stroke_color`.
	#[must_use]
	pub fn color(mut self, value: Color) -> Self {
		self.fill.fill_color = Some(value);
		self.stroke.stroke_color = Some(value);
		self
	}
	
	/// The fill color.
	#[must_use]
	pub fn fill_color(mut self, value: Color) -> Self {
		self.fill.fill_color = Some(value);
		self
	}
	
	/// The fill opacity between 0.0 and 1.0.
	#[must_use]
	pub fn fill_opacity(mut self, value: f32) -> Self {
		self.fill.fill_opacity = Some(value);
		self
	}
	
	/// The stroke position. Defaults to [`StrokePosition::Center`]. This property is not supported on Internet Explorer 8 and earlier.
	#[must_use]
	pub fn stroke_position(mut self, value: StrokePosition) -> Self {
		self.fill.stroke_position = Some(value);
		self
	}
	
	/// The stroke color.
	#[must_use]
	pub fn stroke_color(mut self, value: Color) -> Self {
		self.stroke.stroke_color = Some(value);
		self
	}
	
	/// The stroke opacity between 0.0 and 1.0.
	#[must_use]
	pub fn stroke_opacity(mut self, value: f32) -> Self {
		self.stroke.stroke_opacity = Some(value);
		self
	}
	
	/// The stroke width in pixels.
	#[must_use]
	pub fn stroke_weight(mut self, value: usize) -> Self {
		self.stroke.stroke_weight = Some(value);
		self
	}
	
	/// If set to `true`, the user can drag this circle over the map. Defaults to `false`.
	#[must_use]
	pub fn draggable(mut self, value: bool) -> Self {
		self.common.draggable = Some(value);
		self
	}
	
	/// If set to `true`, the user can edit this circle by dragging the control points shown at the center and around the circumference of the circle. Defaults to `false`.
	#[must_use]
	pub fn editable(mut self, value: bool) -> Self {
		self.common.editable = Some(value);
		self
	}
	
	/// Whether this circle is visible on the map. Defaults to `true`.
	#[must_use]
	pub fn visible(mut self, value: bool) -> Self {
		self.common.visible = Some(value);
		self
	}
	
	/// The z-index compared to other polygons.
	#[must_use]
	pub fn z_index(mut self, value: isize) -> Self {
		self.common.z_index = Some(value);
		self
	}
}


impl JavaScript for Circle {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str("new google.maps.Circle(")?;
		f.write_object()
			.entry("map", &MAP_IDENT)
			.entry("center", &self.center)
			.entry("radius", &self.radius)
			.entry_maybe("fillColor", &self.fill.fill_color)
			.entry_maybe("fillOpacity", &self.fill.fill_opacity)
			.entry_maybe("strokePosition", &self.fill.stroke_position)
			.entry_maybe("strokeColor", &self.stroke.stroke_color)
			.entry_maybe("strokeOpacity", &self.stroke.stroke_opacity)
			.entry_maybe("strokeWeight", &self.stroke.stroke_weight)
			.entry_maybe("draggable", &self.common.draggable)
			.entry_maybe("editable", &self.common.editable)
			.entry_maybe("visible", &self.common.visible)
			.entry_maybe("zIndex", &self.common.z_index)
			.finish()?;
		f.write_str(")")?;
		Ok(())
	}
}


/// A rectangle overlay.
///
/// # Examples
/// ```
/// use mapplot::google::{GoogleMap, MapType, Rectangle};
///
/// let html = GoogleMap::new((0.0, 0.0), 1, MapType::Roadmap, "<your-apikey-here>")
///     .rectangle(Rectangle::new((11.1, 22.2), (33.3, 44.4)))
///     .to_string();
///
/// std::fs::write("map.html", html).unwrap();
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
	bounds: LatLngBounds,
	fill: FillOptions,
	stroke: StrokeOptions,
	common: CommonOptions,
}


impl Rectangle {
	/// Create a new Rectangle by specifying its south-west and north-east corners.
	#[must_use]
	pub fn new(sw: impl Into<LatLng>, ne: impl Into<LatLng>) -> Self {
		Rectangle {
			bounds: LatLngBounds::new(sw.into(), ne.into()),
			fill: FillOptions::default(),
			stroke: StrokeOptions::default(),
			common: CommonOptions::default(),
		}
	}
	
	/// Set both `fill_color` and `stroke_color`.
	#[must_use]
	pub fn color(mut self, value: Color) -> Self {
		self.fill.fill_color = Some(value);
		self.stroke.stroke_color = Some(value);
		self
	}
	
	/// The fill color.
	#[must_use]
	pub fn fill_color(mut self, value: Color) -> Self {
		self.fill.fill_color = Some(value);
		self
	}
	
	/// The fill opacity between 0.0 and 1.0.
	#[must_use]
	pub fn fill_opacity(mut self, value: f32) -> Self {
		self.fill.fill_opacity = Some(value);
		self
	}
	
	/// The stroke position. Defaults to [`StrokePosition::Center`]. This property is not supported on Internet Explorer 8 and earlier.
	#[must_use]
	pub fn stroke_position(mut self, value: StrokePosition) -> Self {
		self.fill.stroke_position = Some(value);
		self
	}
	
	/// The stroke color.
	#[must_use]
	pub fn stroke_color(mut self, value: Color) -> Self {
		self.stroke.stroke_color = Some(value);
		self
	}
	
	/// The stroke opacity between 0.0 and 1.0.
	#[must_use]
	pub fn stroke_opacity(mut self, value: f32) -> Self {
		self.stroke.stroke_opacity = Some(value);
		self
	}
	
	/// The stroke width in pixels.
	#[must_use]
	pub fn stroke_weight(mut self, value: usize) -> Self {
		self.stroke.stroke_weight = Some(value);
		self
	}
	
	/// If set to `true`, the user can drag this rectangle over the map. Defaults to `false`.
	#[must_use]
	pub fn draggable(mut self, value: bool) -> Self {
		self.common.draggable = Some(value);
		self
	}
	
	/// If set to `true`, the user can edit this rectangle by dragging the control points shown at the corners and on each edge. Defaults to `false`.
	#[must_use]
	pub fn editable(mut self, value: bool) -> Self {
		self.common.editable = Some(value);
		self
	}
	
	/// Whether this rectangle is visible on the map. Defaults to `true`.
	#[must_use]
	pub fn visible(mut self, value: bool) -> Self {
		self.common.visible = Some(value);
		self
	}
	
	/// The z-index compared to other polygons.
	#[must_use]
	pub fn z_index(mut self, value: isize) -> Self {
		self.common.z_index = Some(value);
		self
	}
}


impl JavaScript for Rectangle {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str("new google.maps.Rectangle(")?;
		f.write_object()
			.entry("map", &MAP_IDENT)
			.entry("bounds", &self.bounds)
			.entry_maybe("fillColor", &self.fill.fill_color)
			.entry_maybe("fillOpacity", &self.fill.fill_opacity)
			.entry_maybe("strokePosition", &self.fill.stroke_position)
			.entry_maybe("strokeColor", &self.stroke.stroke_color)
			.entry_maybe("strokeOpacity", &self.stroke.stroke_opacity)
			.entry_maybe("strokeWeight", &self.stroke.stroke_weight)
			.entry_maybe("draggable", &self.common.draggable)
			.entry_maybe("editable", &self.common.editable)
			.entry_maybe("visible", &self.common.visible)
			.entry_maybe("zIndex", &self.common.z_index)
			.finish()?;
		f.write_str(")")?;
		Ok(())
	}
}
