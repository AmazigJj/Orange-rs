use ultraviolet::Vec2;
use crate::minecraft::identifier::Identifier;
use crate::minecraft::registry::Registerable;

/// A reference to some image in memory
///
#[derive(Copy, Clone, Debug)]
pub enum TextureObject {
    /// A plain-old 2d texture
    Texture2D {},
    /// An array of textures
    TextureArray {},
    /// An atlas of textures
    TextureAtlas {},
    /// A texture within an atlas
    AtlasTexture { internal_uv: [Vec2; 2] },
    /// A texture within an array
    ArrayTexture { layer: usize },
    /// A plain-old 3d texture
    Texture3D {},
}

impl Registerable for TextureObject {
    fn get_identifier(&self) -> &Identifier {
        todo!()
    }
}