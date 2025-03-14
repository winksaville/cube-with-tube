# Cube with a tube in the center

Create cubes with a tube in the center

## Install

```
cargo install --path .
```
## Usage

```
$ cube-with-tube
Usage: cube-with-tube len_side tube_diameter segments cube_count tube_diameter_step
```

## Run

### Create one cube

Create one cube with a tube in the center with the following dimensions:
```
$ cargo run 3 0.561 50 1 0
```

or if installed

```
$ cargo-with-tube 3 0.561 50 1 0
```

Display the spindle in a 3D viewer.
```

$ f3d cube-with-tube-0.len_side-3.000_tube_diameter-0.561_segments-50.stl
```

Create a png image of the spindle.
```
$ f3d cube-with-tube-0.len_side-3.000_tube_diameter-0.561_segments-50.stl --output cube-with-tube-0.len_side-3.000_tube_diameter-0.561_segments-50.stl.png 
```

![cargo-with-tube -- 3 0.561 50 1 0](./cube-with-tube-0.len_side-3.000_tube_diameter-0.561_segments-50.stl.png)

### Print mulitple cubes with tubes in the center

Here we create 9 cubes with a 2.397 side length and a tube diameter of 0.561.
The tube diameter is increased by 0.085 for each cube so the last cube has a
tube diameter of 1.241.

I've chosen the physical dimension number, 2.397, 0.561 and 0.085 as they are
multiples of 0.017, which is the resolution of the 3D printer I'm using.

```
cargo run 2.397 0.561 50 9 0.085
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
