use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};

// source: https://github.com/bevyengine/bevy/blob/main/examples/3d/lines.rs
#[derive(Default, AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "cc3af62a-f8c1-49dd-82e8-7dcac3fc8830"]
pub struct FaceMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material for FaceMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/face_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // This is the important part to tell bevy to render this material as a line between vertices
        descriptor.primitive.polygon_mode = PolygonMode::Fill;
        Ok(())
    }
}
