use core::f32;

use bevy::math::{FloatPow, Vec2, VectorSpace};

use crate::components::Shape;

pub fn point_in_shape(shape: &Shape, shape_position: Vec2, point: Vec2) -> bool {
    match shape {
        Shape::Circle(radius) => (point - shape_position).length() <= *radius,
        Shape::Polygon(polygon_points) => {

            let point = point - shape_position;
            let mut inside = false;
            let mut j = polygon_points.len() - 1;

            for i in 0..polygon_points.len() {
                let pi = polygon_points[i];
                let pj = polygon_points[j];

                if (pi.y > point.y) != (pj.y > point.y)
                    && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
                {
                    inside = !inside;
                }
                j = i;
            }

            inside
        },
    }
}

// pub fn signed_distance_to_sahpe(shape: &Shape, shape_position: Vec2, point: Vec2) -> f32 {
//     match shape {
//         Shape::Circle(radius) => (point - shape_position).length() - radius,
//         Shape::Polygon(polygon_points) => {

//             let mut min_distance = f32::INFINITY;

//             for (p1, p2) in polygon_points.iter().zip(polygon_points.iter().skip(1)) {
//                 let r = p1.dot(*p2);

//                 let r = r / (p2 - p1).length_squared();

//                 let dist = match r {
//                    x if x < 0. => (point - p1).length(),
//                    x if x > 1. => (p2 -point).length(),
//                    _ => f32::sqrt((point - p1).length_squared() - (r * (p2 - p1).length()).squared())
//                 };

//                 min_distance = min_distance.min(dist);
//             }
            
//             if point_in_shape(shape, shape_position, point){
//                 return - min_distance;
//             }

//             min_distance
//         },
//     }
// }


pub fn signed_distance_to_sahpe(shape: &Shape, shape_position: Vec2, point: Vec2) -> f32 {
    match shape {
        Shape::Circle(radius) => (point - shape_position).length() - radius,
        Shape::Polygon(polygon_points) => {
        
            let num_points = polygon_points.len();
            let mut min_dist: f32 = f32::INFINITY;

            for i in 0..num_points{

                let a = polygon_points[i];
                let b = polygon_points[(i + 1) % num_points];

                let t = (point - a).dot(b - a) / (b - a).dot( b - a);

                let t = t.clamp(0., 1.);

                let closes_point = a.lerp(b, t);

                min_dist = min_dist.min((point - closes_point).length());
            }
            
            return min_dist;
        }
    }
}


pub fn signed_distance_and_normal_to_sahpe(shape: &Shape, shape_position: Vec2, point: Vec2) -> (Vec2, f32) {
    match shape {
        Shape::Circle(radius) => ( point - shape_position, (point - shape_position).length() - radius),
        Shape::Polygon(polygon_points) => {
        
            let point = point - shape_position;

            let num_points = polygon_points.len();
            let mut min_dist: f32 = f32::INFINITY;
            let mut segment = (Vec2::ZERO, Vec2::ZERO);
            let mut t_of_min = 0.;

            for i in 0..num_points{

                let a = polygon_points[i];
                let b = polygon_points[(i + 1) % num_points];

                let t = (point - a).dot(b - a) / (b - a).dot( b - a);

                let closes_point = a.lerp(b, t.clamp(0., 1.));
                
                let distance = (point - closes_point).length();

                if distance < min_dist{
                    min_dist = distance;
                    segment = (a, b);
                    t_of_min = t;
                }
            }

            let ab = segment.1 - segment.0;
            // Vec2::new(ab.y, -ab.x)
            let normal = match t_of_min {
                x if x < 0. => point - segment.0,
                x if x > 1. => point - segment.1,
                _ => Vec2::new(ab.y, -ab.x)
            };

            return (normal, min_dist);
        }
    }
}