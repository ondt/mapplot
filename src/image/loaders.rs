use std::fmt::Debug;

use async_trait::async_trait;
use reqwest::StatusCode;
use svg::node::element::{self, Element};
use thiserror::Error;

use crate::proj;


#[async_trait]
pub trait TilesetLoader: Debug {
	async fn load_tile(&self, zoom: u8, x: u32, y: u32) -> Result<Element, TilesetLoaderError>;
}


#[derive(Error, Debug)]
pub enum TilesetLoaderError {
	#[error("reqwest error")]
	Reqwest(#[from] reqwest::Error),
	#[error("tile request error: {text:?} (status code: {status_code:?})")]
	ApiError { status_code: StatusCode, text: String },
	#[error("unexpected MIME type: {0:?}")]
	UnexpectedMimeType(Option<&'static str>),
}


#[derive(Debug, Clone)]
pub struct MapboxTilesetLoader {
	tileset: String,
	hires: bool,
	token: String,
}


impl MapboxTilesetLoader {
	pub fn new(tileset: impl AsRef<str>, hires: bool, token: impl AsRef<str>) -> Self {
		MapboxTilesetLoader {
			tileset: tileset.as_ref().to_string(),
			hires,
			token: token.as_ref().to_string(),
		}
	}
}


#[async_trait]
impl TilesetLoader for MapboxTilesetLoader {
	async fn load_tile(&self, zoom: u8, x: u32, y: u32) -> Result<Element, TilesetLoaderError> {
		let url = format!("https://api.mapbox.com/v4/{tileset}/{zoom}/{x}/{y}{hires}.png?access_token={token}", tileset = self.tileset, zoom = zoom, x = x, y = y, hires = if self.hires { "@2x" } else { "" }, token = self.token);
		let response = reqwest::get(url).await?;
		let status_code = response.status();
		
		if status_code != StatusCode::OK {
			let text = response.text().await?;
			return Err(TilesetLoaderError::ApiError { status_code, text });
		}
		
		let image = response.bytes().await?;
		
		let mime_type = match infer::get(&image).map(|t| t.mime_type()) {
			Some("image/png") => "image/png",
			Some("image/jpeg") => "image/jpeg",
			t => return Err(TilesetLoaderError::UnexpectedMimeType(t)),
		};
		
		let merc = {
			let (lon, lat) = slippy_map_tilenames::tile2lonlat(x, y, zoom);
			proj((lat, lon), zoom)
		};
		
		let element = element::Image::new()
			.set("x", merc.x)
			.set("y", merc.y)
			.set("width", 256)
			.set("height", 256)
			.set("href", format!("data:{};base64,{}", mime_type, base64::encode(image)));
		
		Ok(element.into())
	}
}

