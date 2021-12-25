# `mapplot` - Map plotter library for Rust
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

