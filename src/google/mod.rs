use std::fmt::{self, Debug, Display, Formatter};

use crate::google::style::{Color, PolygonStyle, PolylineStyle, StrokePosition};


pub mod style;


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
		self.entry_opt(key, &Some(value))
	}
	
	fn entry_opt(&mut self, key: &str, value: &Option<impl JavaScript>) -> &mut Self {
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


pub trait JavaScript {
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


pub struct GoogleMap {
	apikey: String,
	page_title: Option<String>,
	center: LatLng,
	zoom: u8,
	map_type: Option<MapType>,
	disable_default_gui: Option<bool>,
	disable_double_click_zoom: Option<bool>,
	shapes: Vec<Box<dyn JavaScript>>,
}


impl GoogleMap {
	// TODO: auto center & zoom
	pub fn new(center: impl Into<LatLng>, zoom: u8, apikey: impl AsRef<str>) -> Self {
		GoogleMap {
			apikey: apikey.as_ref().to_string(),
			page_title: None,
			center: center.into(),
			zoom,
			map_type: None,
			disable_default_gui: None,
			disable_double_click_zoom: None,
			shapes: Vec::default(),
		}
	}
	
	/// Set the title of the HTML page.
	pub fn page_title(&mut self, value: impl AsRef<str>) -> &mut Self {
		self.page_title = Some(value.as_ref().to_string());
		self
	}
	
	/// The initial map type. Defaults to [`MapType::Roadmap`].
	pub fn map_type(&mut self, value: MapType) -> &mut Self {
		self.map_type = Some(value);
		self
	}
	
	/// Enable/disable all default UI buttons.
	pub fn disable_default_gui(&mut self, value: bool) -> &mut Self {
		self.disable_default_gui = Some(value);
		self
	}
	
	/// Enable/disable zoom and center on double click. Enabled by default.
	pub fn disable_double_click_zoom(&mut self, value: bool) -> &mut Self {
		self.disable_double_click_zoom = Some(value);
		self
	}
	
	/// Draw a shape on the map.
	pub fn draw(&mut self, shape: impl JavaScript + 'static) -> &mut Self {
		self.shapes.push(Box::new(shape));
		self
	}
	
	/// Draw multiple shapes at once.
	pub fn draw_all(&mut self, shapes: impl IntoIterator<Item=impl JavaScript + 'static>) -> &mut Self {
		for shape in shapes {
			self.shapes.push(Box::new(shape))
		}
		self
	}
}


impl JavaScript for GoogleMap {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, r#"		const {} = new google.maps.Map(document.getElementById("map_canvas"), "#, MAP_IDENT)?;
		f.write_object()
			.entry("center", &self.center)
			.entry("zoom", &self.zoom)
			.entry_opt("mapTypeId", &self.map_type)
			.entry_opt("disableDefaultUI", &self.disable_default_gui)
			.entry_opt("disableDoubleClickZoom", &self.disable_double_click_zoom)
			.finish()?;
		f.write_str(");\n\n")?;
		
		for shape in &self.shapes {
			f.write_str("\t\t")?;
			shape.fmt_js(f)?;
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
"#, title = if let Some(t) = &self.page_title { t.as_str() } else { "Google Maps - mapplot" }, apikey = self.apikey)?;
		
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


impl JavaScript for Color {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match *self {
			Color::RGB(r, g, b) => write!(f, "\"#{:02x}{:02x}{:02x}\"", r, g, b),
			Color::RGBA(r, g, b, a) => write!(f, "\"#{:02x}{:02x}{:02x}{:02x}\"", r, g, b, a),
			Color::HSL(h, s, l) => write!(f, "\"hsl({h}, {s}%, {l}%)\"", h = h, s = 100.0 * f64::from(s) / 255.0, l = 100.0 * f64::from(l) / 255.0),
			Color::HSLA(h, s, l, a) => write!(f, "\"hsla({}, {}%, {}%, {}%)\"", h = h, s = 100.0 * f64::from(s) / 255.0, l = 100.0 * f64::from(l) / 255.0, a = 100.0 * f64::from(a) / 255.0),
			named => format!("{:?}", named).to_lowercase().fmt_js(f),
		}
	}
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


// TODO: AsRef?
impl From<&(f64, f64)> for LatLng {
	fn from((lat, lon): &(f64, f64)) -> Self {
		LatLng { lat: *lat, lon: *lon }
	}
}


impl From<Marker> for LatLng {
	fn from(m: Marker) -> Self {
		m.position
	}
}


impl JavaScript for LatLng {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "new google.maps.LatLng({}, {})", self.lat, self.lon)
	}
}


#[derive(Debug, Copy, Clone)]
pub struct LatLngBounds {
	p1: LatLng,
	p2: LatLng,
}


impl LatLngBounds {
	#[must_use]
	pub fn new(p1: LatLng, p2: LatLng) -> Self {
		LatLngBounds { p1, p2 }
	}
}


impl JavaScript for LatLngBounds {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str("new google.maps.LatLngBounds(")?;
		self.p1.fmt_js(f)?;
		f.write_str(", ")?;
		self.p2.fmt_js(f)?;
		f.write_str(")")?;
		Ok(())
	}
}


#[derive(Debug, Clone)]
pub struct Marker {
	position: LatLng,
	label: Option<String>,
	title: Option<String>,
	opacity: Option<f64>,
	z_index: Option<isize>,
}


impl Marker {
	#[must_use]
	pub fn new(pos: impl Into<LatLng>) -> Self {
		Marker {
			position: pos.into(),
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
			.entry_opt("label", &self.label)
			.finish()?;
		f.write_str(")")?;
		Ok(())
	}
}


/// A polyline is a linear overlay of connected line segments on the map.
///
/// # Examples
/// ```
/// use mapplot::google::{GoogleMap, MapType, Polyline};
///
/// let html = GoogleMap::new((0.0, 0.0), 1, "<your-apikey-here>")
///     .draw(Polyline::new([(11.1, 22.2), (33.3, 44.4), (-22.2, 11.1)]))
///     .to_string();
///
/// std::fs::write("map.html", html).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Polyline {
	path: Vec<LatLng>,
	geodesic: Option<bool>,
	style: PolylineStyle,
	common: CommonOptions,
}


impl Polyline {
	/// Create a new Polyline.
	#[must_use]
	pub fn new(points: impl IntoIterator<Item=impl Into<LatLng>>) -> Self {
		Polyline {
			path: points.into_iter().map(Into::into).collect(),
			geodesic: None,
			style: PolylineStyle::default(),
			common: CommonOptions::default(),
		}
	}
	
	/// When `true`, edges of the polygon are interpreted as geodesic and will follow the curvature of the Earth. When `false`, edges of the polygon are rendered as straight lines in screen space. Note that the shape of a geodesic polygon may appear to change when dragged, as the dimensions are maintained relative to the surface of the earth. Defaults to `false`.
	#[must_use]
	pub fn geodesic(mut self, value: bool) -> Self {
		self.geodesic = Some(value);
		self
	}
	
	/// Set a style for this shape.
	#[must_use]
	pub fn style(mut self, value: impl Into<PolylineStyle>) -> Self {
		self.style = value.into();
		self
	}
	
	/// If set to `true`, the user can drag this shape over the map. The `geodesic` property defines the mode of dragging. Defaults to `false`.
	#[must_use]
	pub fn draggable(mut self, value: bool) -> Self {
		self.common.draggable = Some(value);
		self
	}
	
	/// If set to `true`, the user can edit this shape by dragging the control points shown at the vertices and on each segment. Defaults to `false`.
	#[must_use]
	pub fn editable(mut self, value: bool) -> Self {
		self.common.editable = Some(value);
		self
	}
	
	/// Whether this polyline is visible on the map. Defaults to `true`.
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


impl JavaScript for Polyline {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str("new google.maps.Polyline(")?;
		f.write_object()
			.entry("map", &MAP_IDENT)
			.entry("path", &self.path)
			.entry_opt("geodesic", &self.geodesic)
			.entry_opt("strokeColor", &self.style.stroke_color)
			.entry_opt("strokeOpacity", &self.style.stroke_opacity)
			.entry_opt("strokeWeight", &self.style.stroke_weight)
			.entry_opt("draggable", &self.common.draggable)
			.entry_opt("editable", &self.common.editable)
			.entry_opt("visible", &self.common.visible)
			.entry_opt("zIndex", &self.common.z_index)
			.finish()?;
		f.write_str(")")?;
		Ok(())
	}
}


/// A geodesic or non-geodesic polygon.
///
/// A polygon (like a polyline) defines a series of connected coordinates in an ordered sequence. Additionally,
/// polygons form a closed loop and define a filled region.
///
/// # Examples
/// ```
/// use mapplot::google::{GoogleMap, MapType, Polygon};
///
/// let html = GoogleMap::new((0.0, 0.0), 1, "<your-apikey-here>")
///     .draw(Polygon::new([(11.1, 22.2), (33.3, 44.4), (-22.2, 11.1)]))
///     .to_string();
///
/// std::fs::write("map.html", html).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Polygon {
	paths: Vec<Vec<LatLng>>,
	geodesic: Option<bool>,
	style: PolygonStyle,
	common: CommonOptions,
}


impl Polygon {
	/// Create a new Polygon.
	#[must_use]
	pub fn new(points: impl IntoIterator<Item=impl Into<LatLng>>) -> Self {
		Polygon {
			paths: vec![points.into_iter().map(Into::into).collect()],
			geodesic: None,
			style: PolygonStyle::default(),
			common: CommonOptions::default(),
		}
	}
	
	/// Add a new path to the polygon. Points forming an inner path need to wind in the opposite direction to those in an outer path to form a hole.
	#[must_use]
	pub fn path(mut self, points: impl IntoIterator<Item=impl Into<LatLng>>) -> Self {
		self.paths.push(points.into_iter().map(Into::into).collect());
		self
	}
	
	/// When `true`, edges of the polygon are interpreted as geodesic and will follow the curvature of the Earth. When `false`, edges of the polygon are rendered as straight lines in screen space. Note that the shape of a geodesic polygon may appear to change when dragged, as the dimensions are maintained relative to the surface of the earth. Defaults to `false`.
	#[must_use]
	pub fn geodesic(mut self, value: bool) -> Self {
		self.geodesic = Some(value);
		self
	}
	
	/// Set a style for this shape.
	#[must_use]
	pub fn style(mut self, value: impl Into<PolygonStyle>) -> Self {
		self.style = value.into();
		self
	}
	
	/// If set to `true`, the user can drag this shape over the map. The `geodesic` property defines the mode of dragging. Defaults to `false`.
	#[must_use]
	pub fn draggable(mut self, value: bool) -> Self {
		self.common.draggable = Some(value);
		self
	}
	
	/// If set to `true`, the user can edit this shape by dragging the control points shown at the vertices and on each segment. Defaults to `false`.
	#[must_use]
	pub fn editable(mut self, value: bool) -> Self {
		self.common.editable = Some(value);
		self
	}
	
	/// Whether this polygon is visible on the map. Defaults to `true`.
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


impl JavaScript for Polygon {
	fn fmt_js(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str("new google.maps.Polygon(")?;
		f.write_object()
			.entry("map", &MAP_IDENT)
			.entry("paths", &self.paths)
			.entry_opt("geodesic", &self.geodesic)
			.entry_opt("fillColor", &self.style.fill_color)
			.entry_opt("fillOpacity", &self.style.fill_opacity)
			.entry_opt("strokePosition", &self.style.stroke_position)
			.entry_opt("strokeColor", &self.style.stroke_color)
			.entry_opt("strokeOpacity", &self.style.stroke_opacity)
			.entry_opt("strokeWeight", &self.style.stroke_weight)
			.entry_opt("draggable", &self.common.draggable)
			.entry_opt("editable", &self.common.editable)
			.entry_opt("visible", &self.common.visible)
			.entry_opt("zIndex", &self.common.z_index)
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
/// let html = GoogleMap::new((0.0, 0.0), 1, "<your-apikey-here>")
///     .draw(Rectangle::new((11.1, 22.2), (33.3, 44.4)))
///     .to_string();
///
/// std::fs::write("map.html", html).unwrap();
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
	bounds: LatLngBounds,
	style: PolygonStyle,
	common: CommonOptions,
}


impl Rectangle {
	/// Create a new Rectangle by specifying any two locations.
	#[must_use]
	pub fn new(p1: impl Into<LatLng>, p2: impl Into<LatLng>) -> Self {
		Rectangle {
			bounds: LatLngBounds::new(p1.into(), p2.into()),
			style: PolygonStyle::default(),
			common: CommonOptions::default(),
		}
	}
	
	/// Set a style for this shape.
	#[must_use]
	pub fn style(mut self, value: impl Into<PolygonStyle>) -> Self {
		self.style = value.into();
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
			.entry_opt("fillColor", &self.style.fill_color)
			.entry_opt("fillOpacity", &self.style.fill_opacity)
			.entry_opt("strokePosition", &self.style.stroke_position)
			.entry_opt("strokeColor", &self.style.stroke_color)
			.entry_opt("strokeOpacity", &self.style.stroke_opacity)
			.entry_opt("strokeWeight", &self.style.stroke_weight)
			.entry_opt("draggable", &self.common.draggable)
			.entry_opt("editable", &self.common.editable)
			.entry_opt("visible", &self.common.visible)
			.entry_opt("zIndex", &self.common.z_index)
			.finish()?;
		f.write_str(")")?;
		Ok(())
	}
}


/// A circle on the Earth's surface; also known as a "spherical cap".
///
/// # Examples
/// ```
/// use mapplot::google::{GoogleMap, MapType, Circle};
///
/// let html = GoogleMap::new((0.0, 0.0), 1, "<your-apikey-here>")
///     .draw(Circle::new((22.2, 33.3), 30_000.0))
///     .to_string();
///
/// std::fs::write("map.html", html).unwrap();
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Circle {
	center: LatLng,
	radius: f64,
	style: PolygonStyle,
	common: CommonOptions,
}


impl Circle {
	/// Create a new circle.
	///
	/// `radius` is the radius in meters on the Earth's surface.
	#[must_use]
	pub fn new(center: impl Into<LatLng>, radius: f64) -> Self {
		Circle {
			center: center.into(),
			radius,
			style: PolygonStyle::default(),
			common: CommonOptions::default(),
		}
	}
	
	/// Set a style for this shape.
	#[must_use]
	pub fn style(mut self, value: impl Into<PolygonStyle>) -> Self {
		self.style = value.into();
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
			.entry_opt("fillColor", &self.style.fill_color)
			.entry_opt("fillOpacity", &self.style.fill_opacity)
			.entry_opt("strokePosition", &self.style.stroke_position)
			.entry_opt("strokeColor", &self.style.stroke_color)
			.entry_opt("strokeOpacity", &self.style.stroke_opacity)
			.entry_opt("strokeWeight", &self.style.stroke_weight)
			.entry_opt("draggable", &self.common.draggable)
			.entry_opt("editable", &self.common.editable)
			.entry_opt("visible", &self.common.visible)
			.entry_opt("zIndex", &self.common.z_index)
			.finish()?;
		f.write_str(")")?;
		Ok(())
	}
}

