use bevy::{
    asset::embedded_asset,
    math::primitives::Rectangle,
    pbr::{
        ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline,
        NotShadowCaster,
    },
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, CompareFunction, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};

pub struct DecalPlugin;
impl Plugin for DecalPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "decal.wgsl");
        app.add_plugins(
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, DecalMaterial>> {
                prepass_enabled: false,
                ..default()
            },
        );
    }
}
/// A quad with specified size, rotated so that normal is facing Vec3::Y and generated tangents.
pub fn decal_mesh_quad(normal: Vec3) -> Mesh {
    Rectangle::from_size(Vec2::ONE)
        .mesh()
        .build()
        .rotated_by(Quat::from_rotation_arc(Vec3::Z, normal))
        .with_generated_tangents()
        .unwrap()
}
/// Bundle containing what you need for a bundle, use [decal_mesh_quad] to generate the mesh.
/*#[derive(Bundle, Default)]
pub struct DecalBundle {
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub decal_material: Handle<ExtendedMaterial<StandardMaterial, DecalMaterial>>,
    /// mesh should preferably be generated by [decal_mesh_quad].
    pub mesh: Handle<Mesh>,
    pub not_shadow_caster: NotShadowCaster,
}*/

/*
#[derive(Component )]
#[require( NotShadowCaster )]
pub struct DecalMaterialComponent (  pub Handle<ExtendedMaterial<StandardMaterial, DecalMaterial>>  ) ;
*/

pub type DecalMaterialExtension = ExtendedMaterial<StandardMaterial, DecalMaterial>;

//need to attach this alongside a Mesh3d()  on the decal entity ! 
pub type DecalMeshMaterial3d = MeshMaterial3d< DecalMaterialExtension >;



impl MaterialExtension for DecalMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_contact_projective_decals/decal.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(label) = &mut descriptor.label {
            *label = format!("decal_{}", *label).into();
        }
        if let Some(ref mut depth) = &mut descriptor.depth_stencil {
            depth.depth_compare = CompareFunction::Always;
        }

        Ok(())
    }
}

/// This is the struct that will be passed to your shader
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct DecalMaterial {
    #[uniform(200)]
    /// Variable for how far the decal will fade onto intersecting geometry.
    /// Default is 8.0
    pub depth_fade_factor: f32,
}
impl Default for DecalMaterial {
    fn default() -> Self {
        Self {
            depth_fade_factor: 8.0,
        }
    }
}
