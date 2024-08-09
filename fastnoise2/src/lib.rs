#![allow(clippy::too_many_arguments)]
mod error;
mod metadata;

pub use error::FastNoiseError;
pub use metadata::MemberType;
use metadata::{format_lookup, Member, METADATA_NAME_LOOKUP, NODE_METADATA};

use fastnoise2_sys::*;
use std::{any::type_name, ffi::CString};

#[derive(Debug)]
pub struct FastNoise {
    handle: *mut core::ffi::c_void,
    metadata_id: i32,
}

impl FastNoise {
    pub fn from_name(metadata_name: &str) -> Result<Self, FastNoiseError> {
        let metadata_name = format_lookup(metadata_name);
        let metadata_id = *METADATA_NAME_LOOKUP
            .get(&metadata_name)
            .ok_or(FastNoiseError::MetadataNameNotFound(metadata_name))?;
        let handle = unsafe { fnNewFromMetadata(metadata_id, 0) };
        Ok(Self {
            handle,
            metadata_id,
        })
    }

    pub fn from_encoded_node_tree(encoded_node_tree: &str) -> Result<Self, FastNoiseError> {
        let cstring =
            CString::new(encoded_node_tree).map_err(FastNoiseError::CStringCreationFailed)?;
        let node_ptr = unsafe { fnNewFromEncodedNodeTree(cstring.as_ptr(), 0) };
        if node_ptr.is_null() {
            Err(FastNoiseError::NodeCreationFailed)
        } else {
            Ok(Self {
                handle: node_ptr,
                metadata_id: unsafe { fnGetMetadataID(node_ptr) },
            })
        }
    }

    pub fn get_simd_level(&self) -> u32 {
        unsafe { fnGetSIMDLevel(self.handle) }
    }

    #[allow(private_bounds)]
    pub fn set<T: MemberValue>(
        &mut self,
        member_name: &str,
        value: T,
    ) -> Result<(), FastNoiseError> {
        let metadata = &NODE_METADATA[self.metadata_id as usize];
        let member_name = format_lookup(member_name);
        let member = metadata
            .members
            .get(&member_name)
            .ok_or(FastNoiseError::MemberNameNotFound(member_name))?;

        value.apply(self, member)
    }

    pub fn gen_uniform_grid_2d(
        &self,
        noise_out: &mut [f32],
        x_start: i32,
        y_start: i32,
        x_size: i32,
        y_size: i32,
        frequency: f32,
        seed: i32,
    ) -> OutputMinMax {
        let mut min_max = [0.0; 2];
        unsafe {
            fnGenUniformGrid2D(
                self.handle,
                noise_out.as_mut_ptr(),
                x_start,
                y_start,
                x_size,
                y_size,
                frequency,
                seed,
                min_max.as_mut_ptr(),
            )
        }
        OutputMinMax::new(min_max)
    }

    pub fn gen_uniform_grid_3d(
        &self,
        noise_out: &mut [f32],
        x_start: i32,
        y_start: i32,
        z_start: i32,
        x_size: i32,
        y_size: i32,
        z_size: i32,
        frequency: f32,
        seed: i32,
    ) -> OutputMinMax {
        let mut min_max = [0.0; 2];
        unsafe {
            fnGenUniformGrid3D(
                self.handle,
                noise_out.as_mut_ptr(),
                x_start,
                y_start,
                z_start,
                x_size,
                y_size,
                z_size,
                frequency,
                seed,
                min_max.as_mut_ptr(),
            )
        }
        OutputMinMax::new(min_max)
    }

    pub fn gen_uniform_grid_4d(
        &self,
        noise_out: &mut [f32],
        x_start: i32,
        y_start: i32,
        z_start: i32,
        w_start: i32,
        x_size: i32,
        y_size: i32,
        z_size: i32,
        w_size: i32,
        frequency: f32,
        seed: i32,
    ) -> OutputMinMax {
        let mut min_max = [0.0; 2];
        unsafe {
            fnGenUniformGrid4D(
                self.handle,
                noise_out.as_mut_ptr(),
                x_start,
                y_start,
                z_start,
                w_start,
                x_size,
                y_size,
                z_size,
                w_size,
                frequency,
                seed,
                min_max.as_mut_ptr(),
            )
        }
        OutputMinMax::new(min_max)
    }

    pub fn gen_position_array_2d(
        &self,
        noise_out: &mut [f32],
        x_pos_array: &[f32],
        y_pos_array: &[f32],
        x_offset: f32,
        y_offset: f32,
        seed: i32,
    ) -> OutputMinMax {
        let mut min_max = [0.0; 2];
        unsafe {
            fnGenPositionArray2D(
                self.handle,
                noise_out.as_mut_ptr(),
                x_pos_array.len() as i32,
                x_pos_array.as_ptr(),
                y_pos_array.as_ptr(),
                x_offset,
                y_offset,
                seed,
                min_max.as_mut_ptr(),
            )
        }
        OutputMinMax::new(min_max)
    }

    pub fn gen_position_array_3d(
        &self,
        noise_out: &mut [f32],
        x_pos_array: &[f32],
        y_pos_array: &[f32],
        z_pos_array: &[f32],
        x_offset: f32,
        y_offset: f32,
        z_offset: f32,
        seed: i32,
    ) -> OutputMinMax {
        let mut min_max = [0.0; 2];
        unsafe {
            fnGenPositionArray3D(
                self.handle,
                noise_out.as_mut_ptr(),
                x_pos_array.len() as i32,
                x_pos_array.as_ptr(),
                y_pos_array.as_ptr(),
                z_pos_array.as_ptr(),
                x_offset,
                y_offset,
                z_offset,
                seed,
                min_max.as_mut_ptr(),
            )
        }
        OutputMinMax::new(min_max)
    }

    pub fn gen_position_array_4d(
        &self,
        noise_out: &mut [f32],
        x_pos_array: &[f32],
        y_pos_array: &[f32],
        z_pos_array: &[f32],
        w_pos_array: &[f32],
        x_offset: f32,
        y_offset: f32,
        z_offset: f32,
        w_offset: f32,
        seed: i32,
    ) -> OutputMinMax {
        let mut min_max = [0.0; 2];
        unsafe {
            fnGenPositionArray4D(
                self.handle,
                noise_out.as_mut_ptr(),
                x_pos_array.len() as i32,
                x_pos_array.as_ptr(),
                y_pos_array.as_ptr(),
                z_pos_array.as_ptr(),
                w_pos_array.as_ptr(),
                x_offset,
                y_offset,
                z_offset,
                w_offset,
                seed,
                min_max.as_mut_ptr(),
            )
        }
        OutputMinMax::new(min_max)
    }

    pub fn gen_tileable_2d(
        &self,
        noise_out: &mut [f32],
        x_size: i32,
        y_size: i32,
        frequency: f32,
        seed: i32,
    ) -> OutputMinMax {
        let mut min_max = [0.0; 2];
        unsafe {
            fnGenTileable2D(
                self.handle,
                noise_out.as_mut_ptr(),
                x_size,
                y_size,
                frequency,
                seed,
                min_max.as_mut_ptr(),
            )
        }
        OutputMinMax::new(min_max)
    }

    pub fn gen_single_2d(&self, x: f32, y: f32, seed: i32) -> f32 {
        unsafe { fnGenSingle2D(self.handle, x, y, seed) }
    }

    pub fn gen_single_3d(&self, x: f32, y: f32, z: f32, seed: i32) -> f32 {
        unsafe { fnGenSingle3D(self.handle, x, y, z, seed) }
    }

    pub fn gen_single_4d(&self, x: f32, y: f32, z: f32, w: f32, seed: i32) -> f32 {
        unsafe { fnGenSingle4D(self.handle, x, y, z, w, seed) }
    }
}

impl Drop for FastNoise {
    fn drop(&mut self) {
        unsafe { fnDeleteNodeRef(self.handle) };
    }
}

#[derive(Debug)]
pub struct OutputMinMax {
    pub min: f32,
    pub max: f32,
}

impl OutputMinMax {
    fn new([min, max]: [f32; 2]) -> Self {
        Self { min, max }
    }
}

trait MemberValue {
    fn apply(self, node: &mut FastNoise, member: &Member) -> Result<(), FastNoiseError>;

    fn invalid_member_type_error(member: &Member) -> FastNoiseError {
        FastNoiseError::InvalidMemberType {
            member_name: member.name.clone(),
            given_type: type_name::<Self>().to_string(),
            expected_type: member.member_type,
        }
    }
}

impl MemberValue for f32 {
    fn apply(self, node: &mut FastNoise, member: &Member) -> Result<(), FastNoiseError> {
        match member.member_type {
            MemberType::Float => {
                if !unsafe { fnSetVariableFloat(node.handle, member.index, self) } {
                    return Err(FastNoiseError::SetFloatFailed);
                }
            }
            MemberType::Hybrid => {
                if !unsafe { fnSetHybridFloat(node.handle, member.index, self) } {
                    return Err(FastNoiseError::SetHybridFloatFailed);
                }
            }
            _ => return Err(Self::invalid_member_type_error(member)),
        }
        Ok(())
    }
}

impl MemberValue for i32 {
    fn apply(self, node: &mut FastNoise, member: &Member) -> Result<(), FastNoiseError> {
        match member.member_type {
            MemberType::Int => {
                if !unsafe { fnSetVariableIntEnum(node.handle, member.index, self) } {
                    return Err(FastNoiseError::SetIntFailed);
                }
            }
            _ => return Err(Self::invalid_member_type_error(member)),
        }
        Ok(())
    }
}

impl MemberValue for &str {
    fn apply(self, node: &mut FastNoise, member: &Member) -> Result<(), FastNoiseError> {
        match member.member_type {
            MemberType::Enum => {
                let enum_idx = member
                    .enum_names
                    .get(&format_lookup(self))
                    .ok_or(FastNoiseError::EnumValueNotFound(self.to_string()))?;
                if !unsafe { fnSetVariableIntEnum(node.handle, member.index, *enum_idx) } {
                    return Err(FastNoiseError::SetEnumFailed);
                }
            }
            _ => return Err(Self::invalid_member_type_error(member)),
        }
        Ok(())
    }
}

impl MemberValue for &FastNoise {
    fn apply(self, node: &mut FastNoise, member: &Member) -> Result<(), FastNoiseError> {
        match member.member_type {
            MemberType::NodeLookup => {
                if !unsafe { fnSetNodeLookup(node.handle, member.index, self.handle) } {
                    return Err(FastNoiseError::SetNodeLookupFailed);
                }
            }
            MemberType::Hybrid => {
                if !unsafe { fnSetHybridNodeLookup(node.handle, member.index, self.handle) } {
                    return Err(FastNoiseError::SetHybridNodeLookupFailed);
                }
            }
            _ => return Err(Self::invalid_member_type_error(member)),
        }
        Ok(())
    }
}
