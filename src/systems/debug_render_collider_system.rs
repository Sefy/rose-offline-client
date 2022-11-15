use bevy::{
    math::Vec3,
    prelude::{Entity, GlobalTransform, Query, Res, ResMut},
};
use bevy_rapier3d::prelude::Collider;

use crate::resources::{DebugRenderColliderData, DebugRenderConfig};

pub fn debug_render_collider_system(
    debug_render_config: Res<DebugRenderConfig>,
    query_colliders: Query<(Entity, &Collider, &GlobalTransform)>,
    mut render_data: ResMut<DebugRenderColliderData>,
) {
    if !debug_render_config.colliders {
        return;
    }

    for (entity, collider, global_transform) in query_colliders.iter() {
        let line_index = entity.index() as usize % render_data.collider.len();
        let line_data = &mut render_data.collider[line_index];

        // The collider shape has already been scaled
        let mut transform = global_transform.compute_transform();
        transform.scale = Vec3::ONE;

        if let Some(cuboid) = collider.as_cuboid() {
            let (vertices, indices) = cuboid.raw.to_outline();

            for idx in indices {
                line_data
                    .vertices
                    .push(transform.transform_point(vertices[idx[0] as usize].into()));
                line_data
                    .vertices
                    .push(transform.transform_point(vertices[idx[1] as usize].into()));
                line_data
                    .vertices
                    .push(Vec3::new(f32::NAN, f32::NAN, f32::NAN));
            }
        }
    }
}
