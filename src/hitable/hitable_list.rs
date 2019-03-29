use bounding_boxes::axis_aligned::AxisAlignedBoundingBox;
use bounding_boxes::utils;
use hitable::hit_record::HitRecord;
use hitable::hitable::Hitable;
use ray::Ray;

/// Represents a list of Hitable objects.
#[derive(Clone)]
pub struct HitableList {
    pub list: Vec<Box<Hitable>>,
}

impl HitableList {
    /// Returns the number of elements in the underlying list
    pub fn len(&self) -> usize {
        self.list.len()
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit = false;
        let mut current_closest = t_max;
        for obj in self.list.iter() {
            if obj.hit(ray, t_min, current_closest, rec) {
                hit = true;
                current_closest = rec.t;
            }
        }

        hit
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<AxisAlignedBoundingBox> {
        if self.list.is_empty() {
            return None;
        }
        let mut bounding_box = self.list[0].bounding_box(start_time, end_time);
        if bounding_box.is_none() {
            return None;
        }
        for i in 1..self.list.len() {
            match self.list[i].bounding_box(start_time, end_time) {
                Some(temp_box) => {
                    bounding_box = Some(utils::calc_surrounding_box(
                        &bounding_box.unwrap(),
                        &temp_box,
                    ));
                }
                None => {
                    bounding_box = None;
                    break;
                }
            }
        }

        bounding_box
    }

    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
}
