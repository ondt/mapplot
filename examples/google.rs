use std::fs;

use mapplot::google::{Circle, Color, GoogleMap, MapType, Marker};


fn main() {
	let points = [
		Marker::new(12.44, 55.22),
		Marker::new(33.33, 44.44),
	];
	
	let html = GoogleMap::new((49.817500, 15.473000), 8, MapType::Roadmap, "AIzaSyCcasQyg6nywlUdCkxBqfeNITOEf6pfMZ4")
		.marker(Marker::new(50.0, 14.0).label("A"))
		.marker(Marker::new(50.0, 15.0).label("B"))
		.marker(Marker::new(50.0, 16.0).label("C"))
		.markers(points)
		.circle(Circle::new(50.0, 17.0, 30_000.0).color(Color::HSL(200, 128, 100)))
		.to_string();
	
	println!("{}", html);
	fs::write("map.html", html).unwrap();
}