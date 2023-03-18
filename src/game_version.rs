use crate::{registry::Registry, block::block_factory::BlockFactory};

pub enum GameVersion {
    B173,
    Orange,
}

impl GameVersion {
    pub fn load_registry(&self, registry: &mut Registry) {
        match self {
            Self::B173 => load_b173(registry),
            _ => {},
        }
    }
}

fn load_b173(registry: &mut Registry) {
    // Blocks & Items
    let blocks = registry.get_block_register_mut();
    let block_register_list = vec![
            BlockFactory::new("air")
                .hardness(0.0)
                .resistance(0.0)
                .texture_index(0)
                .transparent(true)
                .build(),
            BlockFactory::new("stone")
                .hardness(1.5)
                .resistance(10.0)
                .texture_index(1)
                .build(),
            BlockFactory::new("grass")
                .hardness(0.6)
                .texture_index(3)
                .build(),
            BlockFactory::new("dirt")
                .hardness(0.5)
                .texture_index(2)
                .build(),
            BlockFactory::new("cobblestone")
                .hardness(2.0)
                .resistance(10.0)
                .texture_index(16)
                .build(),
            BlockFactory::new("wood")
                .hardness(2.0)
                .resistance(5.0)
                .texture_index(4)
                .build(),
            BlockFactory::new("sapling")
                .hardness(0.0)
                .transparent(true)
                .texture_index(15)
                .build(),
            BlockFactory::new("bedrock")
                .hardness(-1.0)
                .resistance(6000000.0)
                .texture_index(17)
                .build(),
            BlockFactory::new("flowing_water")
                .hardness(100.0)
                .texture_index(222)
                .transparent(true)
                .build(),
            BlockFactory::new("still_water")
                .hardness(100.0)
                .texture_index(222)
                .transparent(true)
                .build(),
            BlockFactory::new("flowing_lava")
                .hardness(0.0)
                .texture_index(255)
                .transparent(true)
                .build(),
            BlockFactory::new("still_lava")
                .hardness(100.0)
                .texture_index(255)
                .transparent(true)
                .build(),
            BlockFactory::new("sand")
                .hardness(0.5)
                .texture_index(18)
                .build(),
            BlockFactory::new("gravel")
                .hardness(0.5)
                .texture_index(19)
                .build(),
            BlockFactory::new("ore_gold")
                .hardness(3.0)
                .resistance(5.0)
                .texture_index(32)
                .build(),
            BlockFactory::new("ore_iron")
                .hardness(3.0)
                .resistance(5.0)
                .texture_index(33)
                .build(),
            BlockFactory::new("ore_coal")
                .hardness(3.0)
                .resistance(5.0)
                .texture_index(34)
                .build(),
            BlockFactory::new("wood")
                .hardness(2.0)
                .texture_index(20)
                .build(),
            BlockFactory::new("leaves")
                .hardness(0.2)
                .texture_index(52)
                .build(),
            BlockFactory::new("sponge")
                .hardness(0.6)
                .texture_index(48)
                .build(),
            BlockFactory::new("glass")
                .hardness(0.3)
                .resistance(6000000.0)
                .texture_index(49)
                .build(),
            BlockFactory::new("ore_lapis")
                .hardness(3.0)
                .resistance(5.0)
                .texture_index(160)
                .build(),
            BlockFactory::new("block_lapis")
                .hardness(3.0)
                .resistance(5.0)
                .texture_index(144)
                .build(),
            BlockFactory::new("dispenser")
                .hardness(3.5)
                .texture_index(46)
                .build(),
            BlockFactory::new("sandstone")
                .hardness(0.8)
                .texture_index(176)
                .build(),
            BlockFactory::new("noteblock")
                .hardness(0.8)
                .texture_index(17)
                .build(),
            BlockFactory::new("bed")
                .hardness(0.2)
                .texture_index(149)
                .build(),
            BlockFactory::new("powered_rail")
                .hardness(0.7)
                .texture_index(179)
                .build(),
            BlockFactory::new("detector_rail")
                .hardness(0.7)
                .texture_index(195)
                .build(),
            BlockFactory::new("sticky_piston")
                .texture_index(106)
                .build(),
            BlockFactory::new("web")
                .hardness(4.0)
                .texture_index(11)
                .build(),
            BlockFactory::new("tall_grass")
                .hardness(0.0)
                .texture_index(39)
                .build(),
            BlockFactory::new("dead_bush")
                .hardness(0.0)
                .texture_index(55)
                .build(),
            BlockFactory::new("piston")
                .texture_index(107)
                .build(),
            BlockFactory::new("piston_extension")
                .texture_index(107)
                .build(),
            BlockFactory::new("wool")
                .hardness(0.8)
                .texture_index(64)
                .build(),
            BlockFactory::new("piston_moving")
                .hardness(-1.0)
                .texture_index(0)
                .build(),
            BlockFactory::new("yellow_flower")
                .hardness(0.0)
                .texture_index(13)
                .build(),
            BlockFactory::new("red_flower")
                .hardness(0.0)
                .texture_index(12)
                .build(),
            BlockFactory::new("brown_mushroom")
                .hardness(0.0)
                .texture_index(29)
                .build(),
            BlockFactory::new("red_mushroom")
                .hardness(0.0)
                .texture_index(28)
                .build(),
            BlockFactory::new("block_gold")
                .hardness(3.0)
                .resistance(10.0)
                .texture_index(23)
                .build(),
            BlockFactory::new("block_iron")
                .hardness(5.0)
                .resistance(10.0)
                .texture_index(22)
                .build(),
            BlockFactory::new("double_stair")
                .hardness(2.0)
                .resistance(10.0)
                .texture_index(6)
                .build(),
            BlockFactory::new("single_stair")
                .hardness(2.0)
                .resistance(10.0)
                .texture_index(6)
                .build(),
            BlockFactory::new("brick_block")
                .hardness(2.0)
                .resistance(10.0)
                .texture_index(7)
                .build(),
            BlockFactory::new("tnt")
                .hardness(0.0)
                .texture_index(8)
                .build(),
            BlockFactory::new("bookshelf")
                .hardness(1.5)
                .texture_index(35)
                .build(),
            BlockFactory::new("mossy_cobblestone")
                .hardness(2.0)
                .resistance(10.0)
                .texture_index(36)
                .build(),
            BlockFactory::new("obsidian")
                .hardness(10.0)
                .resistance(2000.0)
                .texture_index(37)
                .build(),
            BlockFactory::new("torch")
                .hardness(0.0)
                .texture_index(80)
                .build(),
            BlockFactory::new("fire")
                .hardness(0.0)
                .texture_index(31)
                .build(),
            BlockFactory::new("mob_spawner")
                .hardness(5.0)
                .texture_index(65)
                .build(),
            BlockFactory::new("wooden_stairs")
                .texture_index(6)
                .build(),
            BlockFactory::new("chest")
                .hardness(2.5)
                .texture_index(27)
                .build(),
            BlockFactory::new("redstone_dust")
                .hardness(0.0)
                .texture_index(164)
                .build(),
            BlockFactory::new("ore_diamond")
                .hardness(3.0)
                .resistance(5.0)
                .texture_index(50)
                .build(),
            BlockFactory::new("block_diamond")
                .hardness(5.0)
                .resistance(10.0)
                .texture_index(24)
                .build(),
            BlockFactory::new("workbench")
                .hardness(2.5)
                .texture_index(43)
                .build(),
            BlockFactory::new("crops")
                .hardness(0.0)
                .texture_index(88)
                .build(),
            BlockFactory::new("tilled_dirt")
                .hardness(0.6)
                .texture_index(87)
                .build(),
            BlockFactory::new("furnace")
                .hardness(3.5)
                .texture_index(44)
                .build(),
            BlockFactory::new("furnace_active")
                .hardness(3.5)
                .texture_index(61)
                .build(),
            BlockFactory::new("sign")
                .hardness(1.0)
                .texture_index(4)
                .build(),
            BlockFactory::new("wooden_door")
                .hardness(3.0)
                .texture_index(71)
                .build(),
            BlockFactory::new("ladder")
                .hardness(0.4)
                .texture_index(83)
                .build(),
            BlockFactory::new("rail")
                .hardness(0.7)
                .texture_index(128)
                .build(),
            BlockFactory::new("cobblestone_stair")
                .hardness(3.0)
                .texture_index(16)
                .build(),
            BlockFactory::new("wall_sign")
                .hardness(1.0)
                .texture_index(4)
                .build(),
            BlockFactory::new("lever")
                .hardness(0.5)
                .texture_index(96)
                .build(),
            BlockFactory::new("stone_pressure_plate")
                .hardness(0.5)
                .texture_index(1)
                .build(),
            BlockFactory::new("iron_door")
                .hardness(3.0)
                .texture_index(72)
                .build(),
            BlockFactory::new("wooden_pressure_plate")
                .hardness(0.5)
                .texture_index(4)
                .build(),
            BlockFactory::new("ore_redstone")
                .hardness(3.0)
                .resistance(5.0)
                .texture_index(51)
                .build(),
            BlockFactory::new("ore_redstone_glowing")
                .hardness(3.0)
                .resistance(5.0)
                .texture_index(51)
                .build(),
            BlockFactory::new("torch_redstone_off")
                .hardness(0.0)
                .texture_index(115)
                .build(),
            BlockFactory::new("torch_redstone_on")
                .hardness(0.0)
                .texture_index(99)
                .build(),
            BlockFactory::new("button")
                .hardness(0.5)
                .texture_index(1)
                .build(),
            BlockFactory::new("snow_layer")
                .hardness(0.1)
                .texture_index(66)
                .build(),
            BlockFactory::new("ice")
                .hardness(0.5)
                .texture_index(67)
                .transparent(true)
                .build(),
            BlockFactory::new("snow")
                .hardness(0.2)
                .texture_index(66)
                .build(),
            BlockFactory::new("cactus")
                .hardness(0.4)
                .texture_index(70)
                .build(),
            BlockFactory::new("clay_block")
                .hardness(0.6)
                .resistance(6000000.0)
                .texture_index(72)
                .build(),
            BlockFactory::new("reed")
                .hardness(0.0)
                .texture_index(73)
                .transparent(true)
                .build(),
            BlockFactory::new("jukebox")
                .hardness(2.0)
                .resistance(10.0)
                .texture_index(74)
                .build(),
            BlockFactory::new("fence")
                .hardness(2.0)
                .resistance(5.0)
                .texture_index(4)
                .transparent(true)
                .build(),
            BlockFactory::new("pumpkin")
                .hardness(1.0)
                .texture_index(102)
                .build(),
            BlockFactory::new("netherrack")
                .hardness(0.4)
                .texture_index(103)
                .build(),
            BlockFactory::new("soulsand")
                .hardness(0.5)
                .texture_index(104)
                .build(),
            BlockFactory::new("glowstone_block")
                .hardness(0.3)
                .texture_index(105)
                .build(),
            BlockFactory::new("portal")
                .hardness(-1.0)
                .texture_index(14)
                .build(),
            BlockFactory::new("pumpkin_lantern")
                .hardness(1.0)
                .texture_index(102)
                .build(),
            BlockFactory::new("cake")
                .hardness(0.5)
                .texture_index(121)
                .build(),
            BlockFactory::new("repeater_off")
                .hardness(0.0)
                .texture_index(131)
                .build(),
            BlockFactory::new("repeater_on")
                .hardness(0.0)
                .texture_index(147)
                .build(),
            BlockFactory::new("locked_chest")
                .hardness(0.0)
                .texture_index(27)
                .build(),
            BlockFactory::new("trapdoor")
                .hardness(-1.0)
                .texture_index(84)
                .build(),
        ];

        for block in block_register_list {
            blocks.insert(block);
        }

    // Items


    // Recipes
    

    // Dimensions
    


}
