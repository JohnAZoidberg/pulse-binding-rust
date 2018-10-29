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

//! Utility functions for direction.

use std::os::raw::c_char;

/// Direction bitfield
///
/// While we currently do not expose anything bidirectional, one should test against the bit instead
/// of the value because we might add bidirectional stuff in the future.
pub type pa_direction_t = i32;

pub const PA_DIRECTION_OUTPUT: pa_direction_t = 0x1;
pub const PA_DIRECTION_INPUT: pa_direction_t = 0x2;

extern "C" {
    pub fn pa_direction_valid(direction: pa_direction_t) -> i32;
    pub fn pa_direction_to_string(direction: pa_direction_t) -> *const c_char;
}
