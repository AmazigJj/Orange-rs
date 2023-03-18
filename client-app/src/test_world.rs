use std::io::Read;
use legion::EntityStore;
use ultraviolet::{IVec2, IVec3, Vec3};
use orange_rs::entities::{EntityController, EntityTransform};
use orange_rs::util::pos::{BlockPos, EntityPos, NewChunkPosition};
use orange_rs::world::chunk::{CHUNK_SECTION_AXIS_SIZE, ChunkSection};
use orange_rs::world::{ChunkStorage, ChunkStoragePlanar, ChunkStorageTrait};

pub struct TestWorld {
    height: usize,
    time: u64,
    spawn_position: BlockPos,
    dimension_id: i8,
    seed: i64,
    has_weather: bool,
    pub chunk_storage: ChunkStorage<ChunkSection>,
    pub entities: legion::World,

    pub player: Option<legion::Entity>,
}

impl TestWorld {
    pub fn new(height: usize) -> Self {
        let entity_world = legion::World::default();

        Self {
            height,
            time: 0,
            spawn_position: BlockPos::new(0, 0, 0),
            dimension_id: 0,
            seed: 0,
            has_weather: false,
            chunk_storage: ChunkStorage::Planar(ChunkStoragePlanar::new(height)),
            entities: entity_world,
            player: None,
        }
    }

    pub fn set_time(&mut self, time: u64) {
        self.time = time;
    }

    pub fn set_spawn_point(&mut self, spawn_position: BlockPos) {
        self.spawn_position = spawn_position;
    }

    pub fn set_dimension_id(&mut self, id: i8) {
        self.dimension_id = id;
    }

    pub fn set_seed(&mut self, seed: i64) {
        self.seed = seed;
    }

    pub fn set_weather(&mut self, has_weather: bool) {
        self.has_weather = has_weather;
    }

    pub fn get_time(&self) -> u64 {
        self.time
    }

    pub fn get_spawn_point(&self) -> BlockPos {
        self.spawn_position.clone()
    }

    pub fn get_dimension_id(&self) -> i8 {
        self.dimension_id
    }

    pub fn get_seed(&self) -> i64 {
        self.seed
    }

    pub fn get_weather(&self) -> bool {
        self.has_weather
    }

    pub fn tick(&mut self) {

    }

    fn ecs_set_entity_component<T: Sync + Send + 'static, F>(entity: legion::Entity, world: &mut legion::World, mut f: F) where F: FnMut(&mut T) {
        if let Some(mut entry) = world.entry(entity) {
            if let Ok(component) = entry.get_component_mut::<T>() {
                f(component);
            }
        }
    }

    fn ecs_get_entity_component<T: Sync + Send + 'static + Clone>(entity: legion::Entity, world: &legion::World) -> Option<T> {
        match world.entry_ref(entity) {
            Ok(entry) => {
                match entry.get_component::<T>() {
                    Ok(component) => { let ret = (*component).clone(); Some(ret) },
                    _ => { None },
                }
            },
            _ => None,
        }
    }

    pub fn set_player_position(&mut self, position: EntityPos) {
        // Self::ecs_set_entity_component::<EntityTransform, _>(self.player.unwrap(), &mut self.entities, |transform: &mut EntityTransform| { transform.position = position; });
        let mut entry = self.entities.entry_mut(self.player.unwrap()).unwrap();
        match entry.get_component_mut::<EntityTransform>() {
            Ok(transform) => {
                transform.position = position;
            },
            _ => {},
        }
    }

    pub fn set_player_look(&mut self, look: Vec3) {
        Self::ecs_set_entity_component::<EntityTransform, _>(self.player.unwrap(), &mut self.entities, |transform| { transform.rotation = look; });
    }

    pub fn set_player_on_ground(&mut self, on_ground: bool) {
        Self::ecs_set_entity_component::<EntityController, _>(self.player.unwrap(), &mut self.entities, |controller| { controller.on_ground = on_ground; });
    }

    pub fn set_player_stance(&mut self, stance: f64) {
        let mut entry = self.entities.entry_mut(self.player.unwrap()).unwrap();
        match entry.get_component_mut::<EntityController>() {
            Ok(controller) => {
                controller.stance = stance;
            },
            _ => {},
        }

    }

    pub fn get_player_transform(&self) -> Option<EntityTransform>{
        // Self::ecs_get_entity_component(self.player.unwrap(), &self.entities)
        match self.entities.entry_ref(self.player.unwrap()) {
            Ok(entry) => {
                match entry.get_component::<EntityTransform>() {
                    Ok(transform) => {
                        let ret = transform.clone();
                        Some(ret)
                    },
                    _ => { None }
                }
            },
            _ => { None }
        }
    }

    pub fn get_player_controller(&self) -> Option<EntityController> {
        // Self::ecs_get_entity_component(self.player.unwrap(), &self.entities)
        match self.entities.entry_ref(self.player.unwrap()) {
            Ok(entry) => {
                match entry.get_component::<EntityController>() {
                    Ok(controller) => { let ret = controller.clone(); Some(ret) },
                    _ => None,
                }
            },
            _ => { None }
        }
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn handle_map_chunk(&mut self, block_x: i32, block_y: i32, block_z: i32, size_x: i8, size_y: i8, size_z: i8, compressed_data: Vec<u8>) {
        let size_x = size_x as usize + 1;
        let size_y = size_y as usize + 1;
        let size_z = size_z as usize + 1;

        // The chunk's position id
        let chunk_x = block_x >> 4;
        let chunk_z = block_z >> 4;

        // The chunk's position as a block position
        let chunk_x_real = chunk_x << 4;
        let chunk_z_real = chunk_z << 4;

        // Starting position as inner chunk coords
        let chunk_x_start = (block_x - chunk_x_real) as u32;
        let chunk_y_start = block_y as u32; // Y should be an offset by nature of minecraft, in the range 0 -> 128
        let chunk_z_start = (block_z - chunk_z_real) as u32;

        let region_size = size_x * size_y * size_z;
        let mut inflater = flate2::read::ZlibDecoder::new(compressed_data.as_slice());
        let expected_size = (region_size * 5) >> 1;
        let mut raw_data = vec![0; expected_size];
        let num_bytes = inflater.read(&mut raw_data).unwrap();

        for y in 0..size_y {

            let actual_y = chunk_y_start + y as u32;
            let chunk_index = (actual_y / CHUNK_SECTION_AXIS_SIZE as u32) as i32;
            let local_y = actual_y % CHUNK_SECTION_AXIS_SIZE as u32;

            let chunk_pos = NewChunkPosition::new(chunk_x, chunk_index, chunk_z);
            let chunk = match self.chunk_storage.get_or_create_chunk(chunk_pos.vec, || { ChunkSection::create_empty() }) {
                Ok(chunk) => chunk,
                _ => continue,
            };

            for x in 0..size_x {
                for z in 0..size_z {

                    let block_index = y + (z * size_y) + (x * size_y * size_z);
                    let byte_index = block_index >> 1;
                    let shift_index = block_index % 2;

                    let meta_index = byte_index + region_size;
                    let block_light_index = 0;
                    let sky_light_index =  0;

                    let data: u64 = raw_data[block_index].into();

                    // let meta = raw_data[meta_index];
                    // let data = Chunk::data_set_meta(data, meta.into());

                    // let block_light = raw_data[block_light_index];
                    // let sky_light = raw_data[sky_light_index];
                    chunk.set_pos(chunk_x_start + x as u32, local_y as u32, chunk_z_start + z as u32, data);
                } // for z
            } // for x
            chunk.set_dirty(true);
        } // for y
        let updated_nearby_chunk_position = [IVec2::new(chunk_x + 1, chunk_z), IVec2::new(chunk_x - 1, chunk_z), IVec2::new(chunk_x, chunk_z + 1), IVec2::new(chunk_x, chunk_z - 1)];
        let a = chunk_y_start as i32 / CHUNK_SECTION_AXIS_SIZE as i32;
        let b = (chunk_y_start as i32 + size_y as i32) / CHUNK_SECTION_AXIS_SIZE as i32;
        for y in a .. b {
            for pos in updated_nearby_chunk_position {
                let pos = IVec3::new(pos.x, y, pos.y);
                let _ = self.chunk_storage.get_chunk_mut(pos).and_then(|chunk| {
                    chunk.set_dirty(true);
                    Ok(())
                });
            }
        }
    }
}