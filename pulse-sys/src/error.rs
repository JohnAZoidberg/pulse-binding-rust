// Copyright 2017 Lyndon Brown
//
// This file is part of the PulseAudio Rust language linking library.
//
// This library is free software; you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation; either version
// 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this library;
// if not, see <http://www.gnu.org/licenses/>.

//! Error management

use std::os::raw::c_char;

/// These represent the i32 error codes returned by many of the underlying PulseAudio C functions.
/// Beware, these enum values are positive values, whilst PA functions return them in negative form,
/// i.e. the `Invalid` variant here has a value of `3`, while functions returning this error code
/// return `-3`. (This is identical to the enum provided in the PA C API).
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum pa_error_code_t {
    /// No error
    Ok = 0,
    /// Access failure
    Access,
    /// Unknown command
    Command,
    /// Invalid argument
    Invalid,
    /// Entity exists
    Exist,
    /// No such entity
    NoEntity,
    /// Connection refused
    ConnectionRefused,
    /// Protocol error
    Protocol,
    Timeout,
    /// No authentication key
    AuthKey,
    Internal,
    ConnectionTerminated,
    /// Entity killed
    Killed,
    InvalidServer,
    ModInitFailed,
    BadState,
    NoData,
    /// Incompatible protocol version
    Version,
    /// Data too large
    TooLarge,
    /// Operation not supported
    NotSupported,
    /// The error code was unknown to the client
    Unknown,
    /// Extension does not exist.
    NoExtension,
    /// Obsolete functionality.
    Obsolete,
    /// Missing implementation.
    NotImplemented,
    /// The caller forked without calling execve() and tried to reuse the context.
    Forked,
    /// An IO error happened.
    Io,
    /// Device or resource busy.
    Busy,
}

pub const PA_ERR_MAX: usize = 27;

pub const PA_OK: pa_error_code_t = pa_error_code_t::Ok;
pub const PA_ERR_ACCESS: pa_error_code_t = pa_error_code_t::Access;
pub const PA_ERR_COMMAND: pa_error_code_t = pa_error_code_t::Command;
pub const PA_ERR_INVALID: pa_error_code_t = pa_error_code_t::Invalid;
pub const PA_ERR_EXIST: pa_error_code_t = pa_error_code_t::Exist;
pub const PA_ERR_NOENTITY: pa_error_code_t = pa_error_code_t::NoEntity;
pub const PA_ERR_CONNECTIONREFUSED: pa_error_code_t = pa_error_code_t::ConnectionRefused;
pub const PA_ERR_PROTOCOL: pa_error_code_t = pa_error_code_t::Protocol;
pub const PA_ERR_TIMEOUT: pa_error_code_t = pa_error_code_t::Timeout;
pub const PA_ERR_AUTHKEY: pa_error_code_t = pa_error_code_t::AuthKey;
pub const PA_ERR_INTERNAL: pa_error_code_t = pa_error_code_t::Internal;
pub const PA_ERR_CONNECTIONTERMINATED: pa_error_code_t = pa_error_code_t::ConnectionTerminated;
pub const PA_ERR_KILLED: pa_error_code_t = pa_error_code_t::Killed;
pub const PA_ERR_INVALIDSERVER: pa_error_code_t = pa_error_code_t::InvalidServer;
pub const PA_ERR_MODINITFAILED: pa_error_code_t = pa_error_code_t::ModInitFailed;
pub const PA_ERR_BADSTATE: pa_error_code_t = pa_error_code_t::BadState;
pub const PA_ERR_NODATA: pa_error_code_t = pa_error_code_t::NoData;
pub const PA_ERR_VERSION: pa_error_code_t = pa_error_code_t::Version;
pub const PA_ERR_TOOLARGE: pa_error_code_t = pa_error_code_t::TooLarge;
pub const PA_ERR_NOTSUPPORTED: pa_error_code_t = pa_error_code_t::NotSupported;
pub const PA_ERR_UNKNOWN: pa_error_code_t = pa_error_code_t::Unknown;
pub const PA_ERR_NOEXTENSION: pa_error_code_t = pa_error_code_t::NoExtension;
pub const PA_ERR_OBSOLETE: pa_error_code_t = pa_error_code_t::Obsolete;
pub const PA_ERR_NOTIMPLEMENTED: pa_error_code_t = pa_error_code_t::NotImplemented;
pub const PA_ERR_FORKED: pa_error_code_t = pa_error_code_t::Forked;
pub const PA_ERR_IO: pa_error_code_t = pa_error_code_t::Io;
pub const PA_ERR_BUSY: pa_error_code_t = pa_error_code_t::Busy;

extern "C" {
    pub fn pa_strerror(error: i32) -> *const c_char;
}
