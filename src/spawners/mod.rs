pub mod spring;
pub mod square;

use bevy::asset::AssetPath;
use bevy::prelude::*;

pub struct Spawner<'a, 'w, 's> {
    commands: &'a mut Commands<'w, 's>,
    entity: Entity,
}

impl<'a, 'w, 's> Spawner<'a, 'w, 's> {
    pub fn new(marker: impl Bundle, commands: &'a mut Commands<'w, 's>) -> Self {
        let entity = commands.spawn(marker).id();
        Self { commands, entity }
    }

    pub fn with_bundle(self, bundle: impl Bundle) -> Self {
        self.commands.entity(self.entity).insert(bundle);
        self
    }

    pub fn with_mesh(self, mesh: impl Into<Mesh>, meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        self.commands
            .entity(self.entity)
            .insert(Mesh2d(meshes.add(mesh)));
        self
    }

    pub fn with_color(
        self,
        color: impl Into<ColorMaterial>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        self.commands
            .entity(self.entity)
            .insert(MeshMaterial2d(materials.add(color)));
        self
    }

    #[expect(dead_code)]
    pub fn with_sprite<'path>(
        self,
        path: impl Into<AssetPath<'path>>,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        self.commands.entity(self.entity).insert(Sprite {
            image: asset_server.load(path),
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..Default::default()
        });
        self
    }

    pub fn with_z_value(self, z_value: f32) -> Self {
        self.commands
            .entity(self.entity)
            .insert(Transform::from_xyz(0.0, 0.0, z_value));
        self
    }

    pub fn id(self) -> Entity {
        self.entity
    }
}
