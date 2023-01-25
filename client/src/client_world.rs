use std::collections::VecDeque;

use crate::rendering::{mesh::Mesh, tessellator::TerrainTessellator, world_renders::WorldRenderer};
use orange_rs::{
    block::Block,
    level::{
        chunk::CHUNK_SECTION_AXIS_SIZE,
        dimension::{Dimension, DimensionChunkDescriptor}, World, 
    },
    registry::Register,
};
use ultraviolet::{IVec2, Vec3, Vec2};
use wgpu::RenderPass;

pub type ClientChunkStorage = Option<Mesh>;

pub struct ClientWorld {
    dimensions: Vec<Dimension>,
    player_level_id: usize,
    world_render: WorldRenderer,
    chunk_height: u32,
}

impl World for ClientWorld {

}

impl ClientWorld {

    pub fn new(chunk_height: u32) -> Self {
        Self {
            dimensions: Vec::new(),
            player_level_id: 0,
            world_render: WorldRenderer::new(chunk_height),
            chunk_height,
        }
    }

    pub fn get_dimension(&self, id: usize) -> Option<&Dimension> {
        self.dimensions.get(id)
    }

    pub fn get_dimension_mut(&mut self, id: usize) -> Option<&mut Dimension> {
        self.dimensions.get_mut(id)
    }

    pub fn get_player_level_id(&self) -> usize {
        self.player_level_id
    }

    pub fn get_player_dimension(&self) -> Option<&Dimension> {
        self.get_dimension(self.player_level_id)
    }

    pub fn get_player_dimension_mut(&mut self) -> Option<&mut Dimension> {
        self.get_dimension_mut(self.player_level_id)
    }

    pub fn add_dimension(&mut self, dim: Dimension) {
        self.dimensions.push(dim);
    } 

    pub fn draw_chunk<'a>(&'a self, x: i32, z: i32, render_pass: &mut RenderPass<'a>, tesselation_queue: &mut VecDeque<DimensionChunkDescriptor>) {
        let world_render = &self.world_render;
        world_render.draw_chunk_mesh((x, z).into(), render_pass, tesselation_queue);
    }

    pub fn draw_chunks<'a>(&'a self, min_extent: IVec2, max_extent: IVec2, render_pass: &mut RenderPass<'a>, tesselation_queue: &mut VecDeque<DimensionChunkDescriptor>) {
        for chunk in self.world_render.get_chunks() {
            let chunk_in_range = chunk.in_range(min_extent, max_extent);
            if !chunk_in_range {
                // println!("Chunk not in range!");
                chunk.mark_for_removal();
            }
        }

        for x in min_extent.x..=max_extent.x {
            for z in min_extent.y..=max_extent.y {
                self.draw_chunk(x, z, render_pass, tesselation_queue);
            }
        }
    } 

    pub fn tesselate_chunk(
        &mut self,
        chunk_pos: IVec2,
        tessellator: &mut TerrainTessellator,
        device: &wgpu::Device,
        blocks: &Register<Block>,
        ) {
        
        let (chunk_x, chunk_z) = (chunk_pos * IVec2::new(CHUNK_SECTION_AXIS_SIZE as i32, CHUNK_SECTION_AXIS_SIZE as i32)).into();

        let chunk = {
            let level = self.get_player_dimension();
            if level.is_none() {
                return;
            }
            let level = level.unwrap();

            let chunk = level.get_chunk_at_vec(chunk_pos);

            // If chunk is none, nothing to build
            if chunk.is_none() {
                return;
            }
            chunk.unwrap()
        };

        let mut section_index: usize = 0;

        let mut meshes: Vec<Mesh> = vec![];
        for section in chunk.get_sections() {
            let section_position = Vec3::new(
                chunk_x as f32,
                (section_index * CHUNK_SECTION_AXIS_SIZE) as f32,
                chunk_z as f32,
                );
            tessellator.tesselate_chunk_section(section, section_position, blocks);
            let mesh = tessellator.build(device);
            meshes.push(mesh);
            section_index += 1;
        }
        self.world_render.construct_chunk(meshes, chunk_pos, self.chunk_height as usize);
    }

    pub fn tesselate_chunks(
        &mut self,
        tesselator: &mut TerrainTessellator,
        tesselation_queue: &mut VecDeque<IVec2>,
        device: &wgpu::Device,
        blocks: &Register<Block>,
        ) {

        for chunk_pos in tesselation_queue {
            self.tesselate_chunk(chunk_pos.clone(), tesselator, device, blocks); 
        }
    }

    pub fn process_chunks(&mut self) {
        self.world_render.remove_marked_chunks();
    }
}