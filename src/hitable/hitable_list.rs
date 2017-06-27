use ray::Ray;
use hitable::hit_record::HitRecord;
use hitable::hitable::Hitable;

pub struct HitableList {
    pub list: Vec<Box<Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit = false;
        let mut currentClosest = t_max;
        for obj in self.list.iter() {
            if obj.hit(ray, t_min, currentClosest, rec) {
                hit = true;
                currentClosest = rec.t;
            }
        }

        hit
    }
}
