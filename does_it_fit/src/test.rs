mod areas_volumes;

pub type GeometricalShapes = areas_volumes::GeometricalShapes;
pub type GeometricalVolumes = areas_volumes::GeometricalVolumes;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let holder_area = areas_volumes::rectangle_area(x, y);

    let shape_area = match kind {
        areas_volumes::GeometricalShapes::Circle => areas_volumes::circle_area(a),
        areas_volumes::GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) as f64,
        areas_volumes::GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        areas_volumes::GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b)
    };

    holder_area as f64 > shape_area * times as f64
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let holder_volume = areas_volumes::cube_volume(x);

    let shape_volume = match kind {
        areas_volumes::GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b),
        areas_volumes::GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        areas_volumes::GeometricalVolumes::Parallelepiped => areas_volumes::parallelepiped_volume(a, b, c) as f64,
        areas_volumes::GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a),
        areas_volumes::GeometricalVolumes::TriangularPyramid => areas_volumes::triangular_pyramid_volume(a as f64, b),
    };

    return match kind {
        areas_volumes::GeometricalVolumes::Cone => holder_volume as f64 >= shape_volume * times as f64 && a <= x,
        _ => holder_volume as f64 >= shape_volume * times as f64 
    }
}