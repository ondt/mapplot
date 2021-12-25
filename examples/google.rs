use std::fs;

use mapplot::google::{GoogleMap, MapType, Marker};


fn main() {
	let html = GoogleMap::new((49.817500, 15.473000), 8, MapType::Roadmap, "AIzaSyCcasQyg6nywlUdCkxBqfeNITOEf6pfMZ4")
		.marker(Marker::new(50.0, 14.0).label("A"))
		.marker(Marker::new(50.0, 15.0).label("B"))
		.marker(Marker::new(50.0, 16.0).label("C"))
		.to_string();
	
	fs::write("map.html", html).unwrap();
}