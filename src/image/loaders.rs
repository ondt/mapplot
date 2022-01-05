use std::fmt::Debug;

use async_trait::async_trait;
use svg::node::element::Element;


#[async_trait]
pub trait TilesetLoader: Debug {
	async fn load_tile(&self, zoom: u8, x: isize, y: isize) -> Element;
}

