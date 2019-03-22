use bounding_boxes::axis_aligned::AxisAlignedBoundingBox;
use bounding_boxes::utils;
use hitable::hit_record::HitRecord;
use hitable::hitable::Hitable;
use hitable::hitable_list::HitableList;
use rand::Rng;
use ray::Ray;

/// Represents a Bounding Volume Hierarchy
#[derive(Clone)]
pub struct BvhNode {
    pub left: Box<Hitable>,
    pub right: Box<Hitable>,
    pub bounding_box: AxisAlignedBoundingBox,
}

impl BvhNode {
    /// Creates a new Bounding Volume Hierarchy Node containing the
    /// elements of the provided HitableList
    pub fn new(hitable_list: &mut HitableList, start_time: f64, end_time: f64) -> Self {
        // Sort the hitable list by a randomly chosen axis
        let rand_axis = (rand::thread_rng().gen::<f64>() * 3.0) as u8;
        let sort_ord = |a: &Box<Hitable>, b: &Box<Hitable>| {
            let a_box = a
                .bounding_box(0.0, 0.0)
                .expect("No bounding box for left child!");
            let b_box = b
                .bounding_box(0.0, 0.0)
                .expect("No bounding box for right child!");
            let (a_min_bound, b_min_bound) = match rand_axis {
                0 => (a_box.min_bound.x(), b_box.min_bound.x()),
                1 => (a_box.min_bound.y(), b_box.min_bound.y()),
                _ => (a_box.min_bound.z(), b_box.min_bound.z()),
            };
            a_min_bound.partial_cmp(&b_min_bound).unwrap()
        };
        hitable_list.list.sort_by(sort_ord);
        // If there are more than 2 elements in the list, split it a la binary search
        let (left, right) = match hitable_list.len() {
            1 => (hitable_list.list[0].clone(), hitable_list.list[0].clone()),
            2 => (hitable_list.list[0].clone(), hitable_list.list[1].clone()),
            _ => {
                let (left_list, right_list) = hitable_list.list.split_at(hitable_list.len() / 2);
                (
                    Box::new(BvhNode::new(
                        &mut HitableList {
                            list: left_list.to_vec(),
                        },
                        start_time,
                        end_time,
                    )) as Box<Hitable>,
                    Box::new(BvhNode::new(
                        &mut HitableList {
                            list: right_list.to_vec(),
                        },
                        start_time,
                        end_time,
                    )) as Box<Hitable>,
                )
            }
        };
        let left_box = left
            .bounding_box(start_time, end_time)
            .expect("No bounding box for left child!");
        let right_box = right
            .bounding_box(start_time, end_time)
            .expect("No bounding box for right child!");
        let bounding_box = utils::calc_surrounding_box(&left_box, &right_box);
        BvhNode {
            left,
            right,
            bounding_box,
        }
    }
}

impl Hitable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return false;
        }
        let ref mut left_rec = HitRecord::new();
        let ref mut right_rec = HitRecord::new();
        let left_hit = self.left.hit(ray, t_min, t_max, left_rec);
        let right_hit = self.right.hit(ray, t_min, t_max, right_rec);
        return if left_hit && right_hit {
            if left_rec.t < right_rec.t {
                rec.from(left_rec);
            } else {
                rec.from(right_rec);
            }
            true
        } else if left_hit {
            rec.from(left_rec);
            true
        } else if right_hit {
            rec.from(right_rec);
            true
        } else {
            false
        };
    }

    fn bounding_box(&self, _start_time: f64, _end_time: f64) -> Option<AxisAlignedBoundingBox> {
        Some(self.bounding_box)
    }

    fn box_clone(&self) -> Box<Hitable> {
        Box::new((*self).clone())
    }
}
