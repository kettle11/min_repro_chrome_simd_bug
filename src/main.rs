use parry3d::bounding_volume::AABB;

fn main() {}

#[no_mangle]
pub(crate) extern "C" fn test() -> f32 {
    let aabbs = [
        (
            0,
            AABB::new(
                [0.0, -50.0, 0.0].into(),
                [199.90764, 175.0, 197.43108].into(),
            ),
        ),
        (
            1,
            AABB::new(
                [-2824.0708, 46.78899, 948.4769].into(),
                [-2822.8105, 47.78899, 949.73706].into(),
            ),
        ),
    ];
    let mut qbvh = parry3d::partitioning::QBVH::<usize>::new();
    qbvh.clear_and_rebuild(aabbs.iter().cloned(), 0.01);
    qbvh.root_aabb().maxs.x
}
