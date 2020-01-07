use crate::gfx::kgl::create_whitespace_cstring;
use crate::gfx::program::{ProgramId, ProgramParameter};
use crate::gfx::shader::{ShaderId, ShaderParameter};
use std::convert::TryInto;
use std::ffi::CString;

/// The kinds of things you can get info logs for from OpenGL, and any information you would need
/// to get that information.
pub enum InfoLogKind {
    Program(ProgramId),
    Shader(ShaderId),
}

/// Gets info logs for the specified `kind`.  OpenGL may generate an info log in many scenarios,
/// such as failed shader compilation.
pub fn info_log_for(kind: InfoLogKind) -> CString {
    let mut info_log_len: gl::types::GLint = 0;
    unsafe {
        match kind {
            InfoLogKind::Program(id) => {
                gl::GetProgramiv(id, ShaderParameter::InfoLogLength.into(), &mut info_log_len);
            }
            InfoLogKind::Shader(id) => {
                gl::GetShaderiv(
                    id,
                    ProgramParameter::InfoLogLength.into(),
                    &mut info_log_len,
                );
            }
        }
    }
    let info_log: CString =
        create_whitespace_cstring(info_log_len.try_into().expect("info log len too long"));
    unsafe {
        match kind {
            InfoLogKind::Program(id) => {
                gl::GetProgramInfoLog(
                    id,
                    info_log_len,
                    std::ptr::null_mut(),
                    info_log.as_ptr() as *mut gl::types::GLchar,
                );
            }
            InfoLogKind::Shader(id) => {
                gl::GetShaderInfoLog(
                    id,
                    info_log_len,
                    std::ptr::null_mut(),
                    info_log.as_ptr() as *mut gl::types::GLchar,
                );
            }
        }
    }
    info_log
}
