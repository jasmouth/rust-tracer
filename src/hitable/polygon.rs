use bounding_boxes::axis_aligned::AxisAlignedBoundingBox;
use hitable::bvh_node::BvhNode;
use hitable::hit_record::HitRecord;
use hitable::hitable::Hitable;
use hitable::hitable_list::HitableList;
use material::material::Material;
use ray::Ray;
use std::f64::MAX as FLOAT_MAX;
use std::f64::MIN as FLOAT_MIN;
use vec3::{cross, dot, unit_vector, Vec3};

/// Represents an n-sided polygon
#[derive(Clone)]
pub struct Polygon {
    pub vertices: Vec<Vec3>,
    pub normal: Vec3,
    pub vertex_normals: Option<Vec<Vec3>>,
    bounding_box: Option<AxisAlignedBoundingBox>,
    material: Box<Material>,
}

impl Polygon {
    /// Constructs a new Polygon from the list of given vertices
    pub fn new(vertices: Vec<Vec3>, material: Box<Material>) -> Self {
        let mut poly = Polygon {
            vertices,
            normal: Vec3::new(0.0, 0.0, 0.0),
            vertex_normals: None,
            bounding_box: None,
            material,
        };
        poly.normal = unit_vector(poly.get_normal());
        poly.bounding_box = poly.bounding_box(0.0, 0.0);

        poly
    }

    /// Calculates the surface normal of the polygon's plane
    ///
    /// **Note:** This will only return the proper face normal
    /// for a *planar* polygon. For any other non-planar polygon, this
    /// merely returns the average normal of its vertices.
    fn get_normal(&self) -> Vec3 {
        let mut normal = Vec3::new(0.0, 0.0, 0.0);
        let n = self.vertices.len();
        for i in 0..n {
            normal += cross(&(self.vertices[i]), &(self.vertices[(i + 1) % n]));
        }
        normal
    }

    /// Interpolates a *triangle's* vertex normals at a given `hit_point`
    fn interpolate_normal(&self, hit_point: Vec3) -> Vec3 {
        match self.vertex_normals {
            Some(ref norms) => {
                let a = self.vertices[0];
                let b = self.vertices[1];
                let c = self.vertices[2];
                let area_abc = dot(&self.normal, &cross(&(b - a), &(c - a)));
                let area_pbc = dot(&self.normal, &cross(&(b - hit_point), &(c - hit_point)));
                let area_pca = dot(&self.normal, &cross(&(c - hit_point), &(a - hit_point)));
                let u = area_pbc / area_abc;
                let v = area_pca / area_abc;
                unit_vector(u * norms[0] + v * norms[1] + (1.0 - u - v) * norms[2])
            }
            None => self.normal,
        }
    }

    /// Uses the even/odd test to determine if the given point lies within the polygon
    fn is_point_in_poly(&self, point: (f64, f64), poly: Vec<(f64, f64)>) -> bool {
        let len = poly.len();
        let mut j = len - 1;
        let mut answer = false;
        let (u, v) = point;
        // Src: https://wrf.ecse.rpi.edu/Research/Short_Notes/pnpoly.html
        for i in 0..len {
            if ((poly[i].1 > v) != (poly[j].1 > v))
                && (u
                    < poly[i].0
                        + (poly[j].0 - poly[i].0) * (v - poly[i].1) / (poly[j].1 - poly[i].1))
            {
                answer = !answer;
            }
            j = i;
        }

        answer
    }
}

impl Hitable for Polygon {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let point_on_plane;
        match self.vertices.get(0) {
            Some(point) => {
                point_on_plane = *point;
            }
            None => return false,
        }
        let denominator = dot(&self.normal, &ray.direction);
        if denominator == 0_f64 {
            return false;
        }
        let t = dot(&self.normal, &(point_on_plane - ray.origin)) / denominator;
        if t < t_min || t > t_max {
            return false;
        }

        // This is the intersection point between the ray and the polygon's plane
        let hit_point = ray.point_at_param(t);
        let bbox = self.bounding_box.unwrap();
        let x_extent = bbox.max_bound.x() - bbox.min_bound.x();
        let y_extent = bbox.max_bound.y() - bbox.min_bound.y();
        let z_extent = bbox.max_bound.z() - bbox.min_bound.z();

        let projected_hit_point: (f64, f64);
        let projected_polygon: Vec<(f64, f64)>;
        match x_extent.min(y_extent).min(z_extent) {
            _x_extent if _x_extent == x_extent => {
                projected_hit_point = (hit_point.y(), hit_point.z());
                projected_polygon = self.vertices.iter().map(|v| (v.y(), v.z())).collect();
            }
            _y_extent if _y_extent == y_extent => {
                projected_hit_point = (hit_point.x(), hit_point.z());
                projected_polygon = self.vertices.iter().map(|v| (v.x(), v.z())).collect();
            }
            _z_extent if _z_extent == z_extent => {
                projected_hit_point = (hit_point.x(), hit_point.y());
                projected_polygon = self.vertices.iter().map(|v| (v.x(), v.y())).collect();
            }
            _ => {
                println!("2D projection failed. Defaulting to throwing away Z-coordinate.");
                projected_hit_point = (hit_point.x(), hit_point.y());
                projected_polygon = self.vertices.iter().map(|v| (v.x(), v.y())).collect();
            }
        };
        if !self.is_point_in_poly(projected_hit_point, projected_polygon) {
            return false;
        }
        rec.t = t;
        rec.hit_point = hit_point;
        rec.normal = self.interpolate_normal(hit_point);
        rec.material = Some(self.material.clone());
        // TODO: calculate u and v
        true
    }

    fn bounding_box(&self, _start_time: f64, _end_time: f64) -> Option<AxisAlignedBoundingBox> {
        let mut min_bound = Vec3::new(FLOAT_MAX, FLOAT_MAX, FLOAT_MAX);
        let mut max_bound = Vec3::new(FLOAT_MIN, FLOAT_MIN, FLOAT_MIN);
        for vert in self.vertices.iter() {
            min_bound.e[0] = vert.x().min(min_bound.x());
            min_bound.e[1] = vert.y().min(min_bound.y());
            min_bound.e[2] = vert.z().min(min_bound.z());
            max_bound.e[0] = vert.x().max(max_bound.x());
            max_bound.e[1] = vert.y().max(max_bound.y());
            max_bound.e[2] = vert.z().max(max_bound.z());
        }
        Some(AxisAlignedBoundingBox::new(min_bound, max_bound))
    }

    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
}

/// Convenience wrapper around a list of Polygons defining
/// a polygon mesh
#[derive(Clone)]
pub struct PolygonMesh {
    pub polygons: BvhNode,
}

impl PolygonMesh {
    /// Constructs a new polygon mesh with the given list of individual polygons
    pub fn new(polygons: Vec<Box<Hitable>>) -> Self {
        PolygonMesh {
            polygons: BvhNode::new(&mut HitableList { list: polygons }, 0.0, 0.0),
        }
    }
}

impl Hitable for PolygonMesh {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.polygons.hit(ray, t_min, t_max, rec)
    }

    fn bounding_box(&self, _start_time: f64, _end_time: f64) -> Option<AxisAlignedBoundingBox> {
        Some(self.polygons.bounding_box)
    }

    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
}
