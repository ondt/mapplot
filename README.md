# `mapplot` - A map plotter library for Rust
`mapplot` is a Rust library that plots all kinds of data on all kinds of maps.

It provides plotters for generating interactive maps (`mapplot::google`, ~~`mapplot::mapbox`~~) and static images
(~~`mapplot::raster`~~, ~~`mapplot::svg`~~).

### Available Plotters
- `mapplot::google` - Generates an HTML file that uses the
  [Google Maps JavaScript API](https://developers.google.com/maps/documentation/javascript/overview) to display map
  data. Inspired by [gmplot](https://github.com/gmplot/gmplot).
- ~~`mapplot::mapbox` - Generates an HTML file similar to `mapplot::google`, but uses
  [Mapbox GL JS](https://github.com/mapbox/mapbox-gl-js) instead and has more features.~~ **(not implemented yet)**
- ~~`mapplot::raster` - Generates a static raster image.~~ **(not implemented yet)**
- ~~`mapplot::svg` - Generates a static SVG image.~~ **(not implemented yet)**

<br>

## Example
![example](https://i.imgur.com/opOfZ4A.png)

```rust
use std::fs;
use mapplot::google::{Circle, Color, GoogleMap, Marker, Polygon, Polyline, Rectangle};

fn main() {
    let netherlands = [
        (53.3224787, 7.1852322),
        (53.0056055, 7.1962228),
        (52.6438557, 7.0463535),
        (52.6447300, 6.7291260),
        (52.4812789, 6.6894452),
        (52.4342406, 7.0236950),
        (52.2424500, 7.0470565),
        (52.0470369, 6.6932297),
        (51.9830783, 6.8290338),
        (51.8964998, 6.7298075),
        (51.8259113, 5.9564802),
        (51.3774719, 6.2276121),
        (51.0097412, 5.8965660),
        (50.9158272, 6.0903844),
        (50.7539195, 6.0194753),
        (50.7542591, 5.6965799),
        (50.8729117, 5.6644788),
        (51.1726854, 5.8394065),
        (51.2601708, 5.2948931),
        (51.4591271, 5.0513089),
        (51.4187846, 4.3904119),
        (51.2176932, 3.8900991),
        (51.3706174, 3.3641251),
    ];
    
    let switzerland = [
        (47.5976076, 8.1243554),
        (47.4744889, 7.0147812),
        (46.0979767, 5.9697126),
        (46.4142137, 6.5677645),
        (45.8690191, 7.1026676),
        (45.9015149, 7.8545345),
        (46.4086133, 8.4121003),
        (45.8274156, 9.0094792),
        (46.4741242, 9.3507219),
        (46.2297483, 10.1575419),
        (46.5731435, 10.0682603),
        (46.5810388, 10.4967421),
        (46.9718163, 10.4143382),
        (47.0381692, 9.6020123),
        (47.5320018, 9.6006684),
        (47.7892979, 8.5809824),
        (47.5976076, 8.1243554),
    ];
    
    let bern = [
        (46.9666268, 7.1781895),
        (47.1238637, 7.3361174),
        (47.0593473, 7.6190164),
        (46.8390079, 7.6863061),
        (46.7638649, 7.3683927),
        (46.9666268, 7.1781895),
    ];
    
    let html = GoogleMap::new((49.7973, 5.4173), 6, "<your-apikey-here>")
        .marker(Marker::new((51.507, -0.127)).label("A"))  // London
        .marker(Marker::new((52.48, -1.902))) // Birmingham
        .polyline(Polyline::new(netherlands).color(Color::Red))
        .polygon(Polygon::new(switzerland).path(bern).color(Color::Red))
        .rectangle(Rectangle::new((53.0833, 8.8), (51.3333, 12.3833)).color(Color::Green).editable(true).draggable(true))
        .circle(Circle::new((48.856, 2.352), 100_000.0).color(Color::HSL(200, 128, 100))) // Paris
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
