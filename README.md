# Hex Grid
[![Build Status](https://travis-ci.org/fuchsnj/hex_grid.svg?branch=master)](https://travis-ci.org/fuchsnj/hex_grid)
[![crates.io](https://img.shields.io/crates/v/pubsub.svg)](https://crates.io/crates/hex_grid)

[Documentation](https://docs.rs/hex_grid)

A library to easily work with 2d hex grids of arbitrary shapes.
This library currently only supports "Pointy Top" hexagons.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
hex_grid = "*"
```

and this to your crate root:

```rust
extern crate hex_grid;
```

## Quick Start

```rust
use hex_grid::*;
use std::collections::HashMap;

struct CustomData{
    //..whatever data you want associated with each tile
}

//empty grid
let mut grid: HashMap<Coordinate, CustomData> = HashMap::new();

//fill the grid with tiles in a hexagon shape of size 3
let coords = CENTER + Offset::fill_hex(3);
for coord in coords {
    let data:CustomData = //...
    grid.insert(coord, data);
}

//get the tile that is to the right 2 tiles from the center tile
let tile:Option<CustomData> = grid.get(CENTER + RIGHT*2);

```
