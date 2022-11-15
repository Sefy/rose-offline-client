use bevy::prelude::{Assets, Color, Commands, ResMut};
use bevy_polyline::prelude::{Polyline, PolylineBundle, PolylineMaterial};

use crate::resources::{
    DebugRenderColliderData, DebugRenderDirectionalLightData, DebugRenderPolyline,
    DebugRenderSkeletonData,
};

const COLOR_LIST: [Color; 8] = [
    Color::RED,
    Color::GREEN,
    Color::BLUE,
    Color::YELLOW,
    Color::CYAN,
    Color::FUCHSIA,
    Color::WHITE,
    Color::BLACK,
];

pub fn debug_render_polylines_setup_system(
    mut commands: Commands,
    mut polylines: ResMut<Assets<Polyline>>,
    mut materials: ResMut<Assets<PolylineMaterial>>,
) {
    let mut collider_line_data = Vec::with_capacity(COLOR_LIST.len());
    for color in COLOR_LIST {
        let polyline = polylines.add(Polyline {
            vertices: Vec::with_capacity(3 * 1024),
        });

        let material = materials.add(PolylineMaterial {
            width: 2.0,
            color,
            perspective: false,
            depth_bias: 0.0,
        });

        collider_line_data.push(DebugRenderPolyline {
            entity: commands
                .spawn(PolylineBundle {
                    polyline: polyline.clone(),
                    material,
                    ..Default::default()
                })
                .id(),
            polyline,
            vertices: Vec::with_capacity(3 * 1024),
        });
    }
    commands.insert_resource(DebugRenderColliderData {
        collider: collider_line_data,
    });

    let skeleton = {
        let polyline = polylines.add(Polyline {
            vertices: Vec::with_capacity(3 * 1024),
        });

        let material = materials.add(PolylineMaterial {
            width: 2.0,
            color: Color::WHITE,
            perspective: false,
            depth_bias: -0.12,
        });

        DebugRenderPolyline {
            entity: commands
                .spawn(PolylineBundle {
                    polyline: polyline.clone(),
                    material,
                    ..Default::default()
                })
                .id(),
            polyline,
            vertices: Vec::with_capacity(3 * 1024),
        }
    };

    let bone_up = {
        let polyline = polylines.add(Polyline {
            vertices: Vec::with_capacity(3 * 1024),
        });

        let material = materials.add(PolylineMaterial {
            width: 2.0,
            color: Color::PINK,
            perspective: false,
            depth_bias: -0.12,
        });

        DebugRenderPolyline {
            entity: commands
                .spawn(PolylineBundle {
                    polyline: polyline.clone(),
                    material,
                    ..Default::default()
                })
                .id(),
            polyline,
            vertices: Vec::with_capacity(3 * 1024),
        }
    };
    commands.insert_resource(DebugRenderSkeletonData { skeleton, bone_up });

    let frustum = {
        let polyline = polylines.add(Polyline {
            vertices: Vec::with_capacity(32),
        });

        let material = materials.add(PolylineMaterial {
            width: 2.0,
            color: Color::LIME_GREEN,
            perspective: false,
            depth_bias: -1.0,
        });

        DebugRenderPolyline {
            entity: commands
                .spawn(PolylineBundle {
                    polyline: polyline.clone(),
                    material,
                    ..Default::default()
                })
                .id(),
            polyline,
            vertices: Vec::with_capacity(32),
        }
    };
    commands.insert_resource(DebugRenderDirectionalLightData { frustum });
}

fn update_line_data(polylines: &mut Assets<Polyline>, line_data: &mut DebugRenderPolyline) {
    if let Some(polyline) = polylines.get_mut(&line_data.polyline) {
        std::mem::swap(&mut polyline.vertices, &mut line_data.vertices);
        line_data.vertices.clear();
    }
}

pub fn debug_render_polylines_update_system(
    mut render_collider_data: ResMut<DebugRenderColliderData>,
    mut render_skeleton_data: ResMut<DebugRenderSkeletonData>,
    mut render_directional_light_data: ResMut<DebugRenderDirectionalLightData>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    for collider in render_collider_data.collider.iter_mut() {
        update_line_data(polylines.as_mut(), collider);
    }

    update_line_data(polylines.as_mut(), &mut render_skeleton_data.bone_up);
    update_line_data(polylines.as_mut(), &mut render_skeleton_data.skeleton);

    update_line_data(
        polylines.as_mut(),
        &mut render_directional_light_data.frustum,
    );
}
