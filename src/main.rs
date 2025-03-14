use std::env;

// Alias the library’s generic CSG type;
type CSG<T> = csgrs::csg::CSG<T>;

fn create_cube_with_tube(len_side: f64, tube_diameter: f64, segments: usize) -> CSG<f64> {
    let cube = CSG::cube(len_side, len_side, len_side, None);

    // Create the tube and translate it to the center of the cube
    let tube_radius = tube_diameter / 2.0;
    let tube = CSG::cylinder(tube_radius, len_side, segments, None);
    let tube = tube.translate(len_side / 2.0, len_side / 2.0, 0.0);

    //let font_data = include_bytes!("../fonts/courier-prime-sans/courier-prime-sans.ttf");
    //let text = format!("{:0.3}", tube_diameter);
    //let csg_text: CSG<f64> = CSG::text(&text, font_data, 20.0, None);
    //let text_3d = csg_text.extrude(1.0);
    //let text_3d = text_3d.rotate(90.0, 0.0, 0.0);
    //let text_3d = text_3d.translate(0.0, len_side / 2.0, len_side / 2.0);

    //// Union the cube with the tube
    //cube.union(&text_3d);

    // Remove the material from the cube to create the tube
    cube.difference(&tube)
}

fn main() {
    // Check for the correct number of command line arguments
    if env::args().len() != 6 {
        eprintln!("Usage: cube-with-tube len_side tube_diameter segments cube_count tube_diameter_step");
        std::process::exit(1);
    }

    // Parse command line arguments to get the spindle dimensions
    let args: Vec<String> = env::args().collect();
    let len_side = args[1].parse::<f64>().unwrap();
    let smallest_tube_diameter = args[2].parse::<f64>().unwrap();
    let segments = args[3].parse::<usize>().unwrap();
    let cube_count = args[4].parse::<usize>().unwrap();
    let tube_diameter_step = args[5].parse::<f64>().unwrap();

    for cube_idx in 0..cube_count {
        let tube_diameter = smallest_tube_diameter + (cube_idx as f64 * tube_diameter_step);
        let cube_with_tube = create_cube_with_tube(len_side, tube_diameter, segments);

        // Write the result as an ASCII STL:
        let name = &format!(
            "cube-with-tube-{}.len_side-{:0.3}_tube_diameter-{:0.3}_segments-{}",
            cube_idx, len_side, tube_diameter, segments
        );
        let stl = cube_with_tube.to_stl_ascii(name);
        std::fs::write(name.to_owned() + ".stl", stl).unwrap();
    }
}
