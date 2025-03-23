use csgrs::csg::CSG;

fn main() {
    let len_side: f64 = 10.0;
    let tube_diameter: f64= 4.0;
    let segments: i32 = 4;

    // Create the cube
    let cube: CSG<f64> = CSG::cube(len_side, len_side, len_side, None);

    // Create the tube and remove the material it's from the cube
    let tube_radius = tube_diameter / 2.0;
    let tube = CSG::cylinder(tube_radius, len_side, segments as usize, None);
    let tube = tube.translate(len_side / 2.0, len_side / 2.0, 0.0);

    let cube_with_tube = cube.difference(&tube);

    if !cube_with_tube.is_manifold() {
        println!("The cube_with_tube is not a manifold");
    }

    // Write the result as an ASCII STL:
    let name = format!(
        "cube_with_tube.len_side-{:0.3}_tube_diameter-{:0.3}_segments-{}",
        len_side, tube_diameter, segments
    );

    let stl = cube_with_tube.to_stl_ascii(&name);
    std::fs::write(name.to_owned() + ".stl", stl).unwrap();
}
