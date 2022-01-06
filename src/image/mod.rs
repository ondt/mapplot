use std::fmt::{Debug, Display, Formatter};

use futures::{stream, StreamExt, TryStreamExt};
use itertools::Itertools;
use svg::Document;
use svg::node::element::Element;

use crate::{BoundingBox, Location, proj};
use crate::image::loaders::{TilesetLoader, TilesetLoaderError};


pub mod loaders;


#[derive(Debug)]
pub struct ImageMap {
	bbox: BoundingBox,
	zoom: u8,
	tiles: Vec<Element>,
	doc: Document,
	shapes: Vec<Box<dyn Shape + Send + Sync>>,
}


impl ImageMap {
	/// # Errors
	/// TODO
	pub async fn new(bbox: BoundingBox, zoom: u8, loader: impl TilesetLoader) -> Result<Self, TilesetLoaderError> {
		let tiles = stream::iter(Self::tile_grid(bbox, zoom))
			.map(|(x, y)| loader.load_tile(zoom, x, y))
			.buffer_unordered(8)
			.try_collect().await?;
		
		Ok(ImageMap {
			bbox,
			zoom,
			tiles,
			doc: Document::new(),
			shapes: Vec::default(),
		})
	}
	
	fn tile_grid(bbox: BoundingBox, zoom: u8) -> impl Iterator<Item=(u32, u32)> {
		let p1 = slippy_map_tilenames::lonlat2tile(bbox.p1.lon, bbox.p1.lat, zoom);
		let p2 = slippy_map_tilenames::lonlat2tile(bbox.p2.lon, bbox.p2.lat, zoom);
		
		let a = p1.0.min(p2.0);
		let b = p1.0.max(p2.0);
		let c = p1.1.min(p2.1);
		let d = p1.1.max(p2.1);
		
		(a..=b).cartesian_product(c..=d)
	}
	
	/// Draw a shape on the map.
	pub fn draw(&mut self, shape: impl Shape + Send + Sync + 'static) -> &mut Self {
		self.shapes.push(Box::new(shape));
		self
	}
	
	
	#[must_use]
	pub fn export_svg(&self) -> String {
		let p1 = proj(self.bbox.p1, self.zoom);
		let p2 = proj(self.bbox.p2, self.zoom);
		
		let min_x = f64::min(p1.x, p2.x);
		let min_y = f64::min(p1.y, p2.y);
		let width = p2.x - p1.x;
		let height = p2.y - p1.y;
		
		let mut doc = Document::new()
			.set("viewBox", (min_x, min_y, width, height));
		
		for tile in &self.tiles {
			doc = doc.add(tile.clone());
		}
		
		for shape in &self.shapes {
			doc = doc.add(shape.to_element(self.zoom));
		}
		
		doc.to_string()
	}
	
	
	/// # Panics
	/// TODO
	#[must_use]
	pub fn export_png(&self, scale: u16) -> Vec<u8> {
		let mut opt = usvg::Options::default();
		opt.fontdb.load_system_fonts();
		
		let svg = self.export_svg();
		let rtree = usvg::Tree::from_data(svg.as_bytes(), &opt.to_ref()).unwrap();
		
		// why not `rtree.svg_node().size.to_screen_size()`?
		let size = rtree.svg_node().view_box.rect.to_screen_size();
		
		let width = size.width() * u32::from(scale);
		let height = size.height() * u32::from(scale);
		let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
		resvg::render(&rtree, usvg::FitTo::Zoom(f32::from(scale)), tiny_skia::Transform::default(), pixmap.as_mut()).unwrap();
		
		pixmap.encode_png().unwrap()
	}
}


impl Display for ImageMap {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.export_svg())
	}
}


////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


pub trait Shape: Debug {
	fn to_element(&self, zoom: u8) -> svg::node::element::Element;
}


// TODO
pub struct Marker {}


#[derive(Debug)]
pub struct Polyline {
	path: Vec<Location>,
	element: svg::node::element::Polyline,
}


impl Polyline {
	pub fn new(points: impl IntoIterator<Item=impl Into<Location>>) -> Self {
		Polyline {
			path: points.into_iter().map(Into::into).collect(),
			element: svg::node::element::Polyline::new(),
		}
	}
	
	#[must_use]
	pub fn color(mut self, value: &str) -> Self {
		self.element = self.element.set("stroke", value);
		self
	}
	
	#[must_use]
	pub fn width(mut self, value: usize) -> Self {
		self.element = self.element.set("stroke-width", value);
		self
	}
}


impl Shape for Polyline {
	fn to_element(&self, zoom: u8) -> svg::node::element::Element {
		let points = self.path.iter().map(|loc| {
			let out = proj(*loc, zoom);
			(out.x, out.y)
		}).collect::<Vec<_>>();
		
		let el = self.element.clone()
			.set("fill", "none")
			.set("points", points);
		
		el.into()
	}
}


pub struct Polygon {}


pub struct Rectangle {}


pub struct Circle {}














