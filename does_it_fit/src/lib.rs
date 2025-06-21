 mod areas_volumes;
 use core::time;

pub use areas_volumes::*;


// pub fn area_fit(
//     (x, y): (usize, usize),
//     kind: areas_volumes::GeometricalShapes,
//     times: usize,
//     (a, b): (usize, usize),
// ) -> bool {
//    let rec_area = areas_volumes::rectangle_area(x,y) as f64;
//    let area = match kind {
//        GeometricalShapes::Circle =>{areas_volumes::circle_area(a)}
//        GeometricalShapes::Rectangle =>{areas_volumes::rectangle_area(a, b) as f64}
//     GeometricalShapes::Square =>{areas_volumes::square_area(a) as f64}
//     GeometricalShapes::Triangle =>{areas_volumes::triangle_area(a, b)}
//    };
//    area*times as f64 <= rec_area
// }

// pub fn volume_fit(
//     (x, y, z): (usize, usize, usize),
//     kind: areas_volumes::GeometricalVolumes,
//     times: usize,
//     (a, b, c): (usize, usize, usize),
// ) -> bool {
//    let volume:f64 = match kind {
//        GeometricalVolumes::Cone =>{areas_volumes::cone_volume(a, b)}
//        GeometricalVolumes::Cube =>{areas_volumes::cube_volume(a) as f64}
//        GeometricalVolumes::Parallelepiped => {areas_volumes::parallelepiped_volume(a, b, c) as f64}
//        GeometricalVolumes::Sphere =>{areas_volumes::sphere_volume(a)}
//     GeometricalVolumes::TriangularPyramid =>{areas_volumes::triangular_pyramid_volume(a as f64, b)}
//    };
//    let box_volume:f64 = (x*y*z) as f64;
//     volume*times as f64 <= box_volume
// }

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
   let rec_area = areas_volumes::rectangle_area(x,y) as f64;
   let area = match kind {
       GeometricalShapes::Circle =>{areas_volumes::circle_area(a)}
       GeometricalShapes::Rectangle =>{areas_volumes::rectangle_area(a, b) as f64}
    GeometricalShapes::Square =>{areas_volumes::square_area(a) as f64}
    GeometricalShapes::Triangle =>{areas_volumes::triangle_area(a, b)}
   };
   area*times as f64 <= rec_area
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
     let volume:f64 = match kind {
       GeometricalVolumes::Cone =>{areas_volumes::cone_volume(a, b)}
       GeometricalVolumes::Cube =>{areas_volumes::cube_volume(a) as f64}
       GeometricalVolumes::Parallelepiped => {areas_volumes::parallelepiped_volume(a, b, c) as f64}
       GeometricalVolumes::Sphere =>{areas_volumes::sphere_volume(a)}
       GeometricalVolumes::TriangularPyramid =>{areas_volumes::triangular_pyramid_volume(a as f64, b)}
   };
   let box_size = (x*y*z) as f64;
   box_size >= volume*times as f64
}