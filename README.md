# `mapplot` - A map plotter library for Rust
_mapplot_ is a Rust library that plots all kinds of data on all kinds of maps.

It provides plotters for generating static images (~~`mapplot::image`~~) and interactive HTML maps
(`mapplot::google`, ~~`mapplot::mapbox`~~).

### Available Plotters
- `mapplot::google` - Generates an HTML file that uses the
  [Google Maps JavaScript API](https://developers.google.com/maps/documentation/javascript/overview) to display map
  data. Inspired by [gmplot](https://github.com/gmplot/gmplot).
- ~~`mapplot::mapbox` - Generates an HTML file similar to `mapplot::google`, but uses
  [Mapbox GL JS](https://github.com/mapbox/mapbox-gl-js) instead and has more features.~~ **(not implemented yet)**
- ~~`mapplot::image` - Generates a static SVG or raster image.~~ **(not implemented yet)**

<br>

## Example
![example](https://i.imgur.com/opOfZ4A.png)

```rust
use std::fs;
use mapplot::google::{Circle, GoogleMap, Marker, Polygon, Polyline, Rectangle, style::Color};

fn main() {
    let netherlands = [
        (53.3224787, 7.1852322),
        (53.0056055, 7.1962228),
        // --snip--
        (51.2176932, 3.8900991),
        (51.3706174, 3.3641251),
    ];
    
    let switzerland = [
        (47.5976076, 8.1243554),
        (47.4744889, 7.0147812),
        // --snip--
        (47.5320018, 9.6006684),
        (47.7892979, 8.5809824),
    ];
    
    let bern = [
        (46.9666268, 7.1781895),
        (47.1238637, 7.3361174),
        (47.0593473, 7.6190164),
        (46.8390079, 7.6863061),
        (46.7638649, 7.3683927),
    ];
    
    let html = GoogleMap::new((49.7973, 5.4173), 6, "<your-apikey-here>")
        .draw(Marker::new((51.507, -0.127)).label("A").title("London"))
        .draw(Marker::new((52.48, -1.902)).title("Birmingham"))
        .draw(Polyline::new(netherlands).style(Color::Red))
        .draw(Polygon::new(switzerland).path(bern).style(Color::Red))
        .draw(Rectangle::new((53.0833, 8.8), (51.3333, 12.3833)).style(Color::Green).editable(true).draggable(true))
        .draw(Circle::new((48.856, 2.352), 100_000.0).style(Color::HSL(200, 128, 100))) // Paris
        .to_string();
    
    fs::write("map.html", html).unwrap();
}
```

<br>

---

#### License
<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
