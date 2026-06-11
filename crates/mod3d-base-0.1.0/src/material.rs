//a Documentation

/*!

This provides for abstract Materials which can be used by any 3D model

!*/

//a Imports
use crate::{Material, MaterialAspect, ShortIndex};

//a BaseData
//tp BaseData
/// The basic data for a material; the most simple material is
/// actually just RGB, but to keep the system simple the [BaseData]
/// includes an alpha, metallicness and roughness.
///
/// For a simple material the alpha should be 1.0, and the metallic 0,
/// and roughness 1
///
/// The simplest of shaders will use just the RGB values
///
/// Gltf
#[derive(Debug, Clone)]
pub struct BaseData {
    /// Color of the material
    ///
    /// Least signficant is R, most A (0 transparent, of course)
    pub rgba: u32,
    /// Metallic nature of the material: 0 is fully dielectric, 1.0 is fully metallic
    ///
    /// Roughness of the material:  0.5 is specular, no specular down to 0 full reflection, up to 1 fully matt
    ///
    /// Each is 16 bits; least significant is metallic
    pub metallic_roughness: u32,
    /// The emissive color of the texture; 0 if unused
    ///
    /// Least signficant is R
    pub emissive_rgb: u32,
    /// The alpha data for the texture (alpha mode, alpha cutoff, etc)
    ///
    /// effectively extensions
    pub alpha_etc: u32,
}

//ip Default for BaseData
impl Default for BaseData {
    fn default() -> Self {
        0xffffffff_u32.into()
    }
}

//ip BaseData
impl BaseData {
    //cp of_rgba
    /// Create a new material with a given RGBA
    pub fn of_rgba((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        let rgba: u32 = (r as u32) | ((g as u32) << 8) | ((b as u32) << 16) | ((a as u32) << 24);
        rgba.into()
    }

    //mp set_rgba
    /// Set the r,g,b,a
    pub fn set_rgba(&mut self, (r, g, b, a): (u8, u8, u8, u8)) {
        let rgba: u32 = (r as u32) | ((g as u32) << 8) | ((b as u32) << 16) | ((a as u32) << 24);
        self.rgba = rgba;
    }

    //mp set_emissive_rgb
    /// Set the emissive r,g,b
    pub fn set_emissive_rgb(&mut self, (r, g, b): (u8, u8, u8)) {
        let rgb: u32 = (r as u32) | ((g as u32) << 8) | ((b as u32) << 16);
        self.emissive_rgb = rgb
    }

    //mp set_mr
    /// Set the metallic and roughness of a material
    pub fn set_mr(&mut self, metallic: f32, roughness: f32) {
        let metallic = (metallic * 65535.0) as u32;
        let roughness = (roughness * 65535.0) as u32;
        self.metallic_roughness = (roughness << 16) | metallic;
    }

    //mp metallic_roughness
    /// Get the metallic and roughness of a material
    pub fn metallic_roughness(&self) -> (f32, f32) {
        let metallic = self.metallic_roughness & 65535;
        let roughness = (self.metallic_roughness >> 16) & 65535;
        let metallic = (metallic as f32) / 65535.0;
        let roughness = (roughness as f32) / 65535.0;
        (metallic, roughness)
    }

    //ap rgba_tuple
    /// Return a tuple of R, G, B, A of the color
    pub fn rgba_tuple(&self) -> (u8, u8, u8, u8) {
        let r = self.rgba & 0xff;
        let g = (self.rgba >> 8) & 0xff;
        let b = (self.rgba >> 16) & 0xff;
        let a = (self.rgba >> 24) & 0xff;
        (r as u8, g as u8, b as u8, a as u8)
    }

    //zz All done
}

impl From<u32> for BaseData {
    fn from(rgba: u32) -> Self {
        Self {
            rgba,
            metallic_roughness: 0,
            emissive_rgb: 0,
            alpha_etc: 0,
        }
    }
}

//a BaseMaterial
//tp BaseMaterial
/// Base material that provides simply color and constant metallicness/roughness
#[derive(Debug)]
pub struct BaseMaterial {
    /// Base material data
    base_data: BaseData,
}

//ip BaseMaterial
impl BaseMaterial {
    //fp of_rgba
    /// Create a new [BaseMaterial] of an RGB color and alpha
    pub fn of_rgba(rgba: u32) -> Self {
        let base_data: BaseData = rgba.into();
        Self { base_data }
    }

    //cp set_mr
    /// Set the metallicness and roughness value for the [BaseMaterial]
    pub fn set_mr(&mut self, metallic: f32, roughness: f32) {
        self.base_data.set_mr(metallic, roughness);
    }
}

//ip Material for BaseMaterial
impl Material for BaseMaterial {
    fn base_data(&self) -> &BaseData {
        &self.base_data
    }
}

//a PbrMaterial
//tp PbrMaterial
/// A physically-based rendered material with full set of textures
#[derive(Debug, Default)]
pub struct PbrMaterial {
    base_data: BaseData,
    base_texture: ShortIndex,
    normal_texture: ShortIndex,
    mr_texture: ShortIndex,
    occlusion_texture: ShortIndex,
    emission_texture: ShortIndex,
}

//ip PbrMaterial
impl PbrMaterial {
    //fp of_rgba
    /// Create a new [BaseMaterial] of an RGB color and alpha
    pub fn of_rgba(rgba: u32) -> Self {
        let base_data: BaseData = rgba.into();
        Self {
            base_data,
            ..Default::default()
        }
    }

    //mp set_rgba
    /// Set the RGBA
    //mp set_rgba
    /// Set the r,g,b,a
    pub fn set_rgba(&mut self, (r, g, b, a): (u8, u8, u8, u8)) {
        self.base_data.set_rgba((r, g, b, a));
    }

    //mp set_emissive_rgb
    /// Set the emission RGB
    pub fn set_emissive_rgb(&mut self, (r, g, b): (u8, u8, u8)) {
        self.base_data.set_emissive_rgb((r, g, b));
    }

    //mp set_mr
    /// Set the metallicness and roughness value for the [BaseMaterial]
    pub fn set_mr(&mut self, metallic: f32, roughness: f32) {
        self.base_data.set_mr(metallic, roughness);
    }

    //mp set_base_data
    /// Set the base data
    pub fn set_base_data(&mut self, base_data: &BaseData) {
        self.base_data = base_data.clone();
    }

    //mp set_texture
    /// Set a texture (currently just base texture) to an index in the Textures of an object
    pub fn set_texture(&mut self, aspect: MaterialAspect, index: ShortIndex) {
        use MaterialAspect::*;
        #[allow(unreachable_patterns)]
        match aspect {
            Color => {
                self.base_texture = index;
            }
            Normal => {
                self.normal_texture = index;
            }
            MetallicRoughness => {
                self.mr_texture = index;
            }
            Occlusion => {
                self.occlusion_texture = index;
            }
            Emission => {
                self.emission_texture = index;
            }
            _ => (),
        }
    }
}

//ip Material for PbrMaterial
impl Material for PbrMaterial {
    fn base_data(&self) -> &BaseData {
        &self.base_data
    }

    fn texture(&self, aspect: MaterialAspect) -> ShortIndex {
        use MaterialAspect::*;
        #[allow(unreachable_patterns)]
        match aspect {
            Color => self.base_texture,
            Normal => self.normal_texture,
            MetallicRoughness => self.mr_texture,
            Occlusion => self.occlusion_texture,
            Emission => self.emission_texture,
            _ => ShortIndex::none(),
        }
    }
}
