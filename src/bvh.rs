use crate::aabb::Aabb;
use crate::hit::{HitRecord, Hitable, HitableHandle};
use crate::ray::Ray;
use crate::util::local_rng;
use rand::Rng;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BvhNode {
    pub bbox: Aabb,
    pub left: Arc<HitableHandle>,
    pub right: Arc<HitableHandle>,
}

impl BvhNode {
    pub fn new(elements: &mut [Arc<HitableHandle>], t0: f32, t1: f32) -> Self {
        let left: Arc<HitableHandle>;
        let right: Arc<HitableHandle>;
        let axis = local_rng().gen_range(0..=2);
        elements.sort_by(|a, b| sort_bbox_by_axis(a, b, axis));
        let len = elements.len();
        if len == 1 {
            left = elements[0].clone();
            right = elements[0].clone();
        } else if len == 2 {
            left = elements[0].clone();
            right = elements[1].clone();
        } else {
            let (left_elems, right_elems) = elements.split_at_mut(len / 2);
            left = Arc::new(Box::new(BvhNode::new(left_elems, t0, t1)));
            right = Arc::new(Box::new(BvhNode::new(right_elems, t0, t1)));
        }
        let box_left = left.bounding_box(0.0, 0.0).unwrap();
        let box_right = right.bounding_box(0.0, 0.0).unwrap();
        let bbox = Aabb::merge(box_left, box_right);

        BvhNode { left, right, bbox }
    }
}

fn sort_bbox_by_axis(left: &HitableHandle, right: &HitableHandle, axis: u32) -> Ordering {
    let box_left = left.bounding_box(0.0, 0.0).unwrap();
    let box_right = right.bounding_box(0.0, 0.0).unwrap();

    box_left.min[axis]
        .partial_cmp(&box_right.min[axis])
        .unwrap()
}

impl Hitable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bbox.hit(ray, t_min, t_max) {
            return None;
        }

        let left_hit = self.left.hit(ray, t_min, t_max);
        let right_hit = self.right.hit(ray, t_min, t_max);

        if let Some(left_hit) = left_hit {
            if let Some(right_hit) = right_hit {
                if left_hit.t < right_hit.t {
                    Some(left_hit)
                } else {
                    Some(right_hit)
                }
            } else {
                Some(left_hit)
            }
        } else if let Some(right_hit) = right_hit {
            Some(right_hit)
        } else {
            None
        }
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        Some(self.bbox)
    }
}
