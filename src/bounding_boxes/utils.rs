use bounding_boxes::axis_aligned::AxisAlignedBoundingBox;
use vec3::Vec3;

/// Calculates a bounding box surrounding two separate bounding boxes
pub fn calc_surrounding_box(
    box_1: &AxisAlignedBoundingBox,
    box_2: &AxisAlignedBoundingBox,
) -> AxisAlignedBoundingBox {
    let small_box = Vec3::new(
        box_1.min_bound.x().min(box_2.min_bound.x()),
        box_1.min_bound.y().min(box_2.min_bound.y()),
        box_1.min_bound.z().min(box_2.min_bound.z()),
    );
    let big_box = Vec3::new(
        box_1.max_bound.x().max(box_2.max_bound.x()),
        box_1.max_bound.y().max(box_2.max_bound.y()),
        box_1.max_bound.z().max(box_2.max_bound.z()),
    );

    AxisAlignedBoundingBox::new(small_box, big_box)
}
