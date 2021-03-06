//! Constants and routines for sample type handling.

// This file is part of the PulseAudio Rust language binding.
//
// Copyright (c) 2017 Lyndon Brown
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

//! # Overview
//!
//! PulseAudio is capable of handling a multitude of sample formats, rates and channels,
//! transparently converting and mixing them as needed.
//!
//! # Sample Formats
//!
//! PulseAudio supports the following sample formats:
//!
//! * `U8` - Unsigned 8 bit integer PCM.
//! * `S16LE` - Signed 16 integer bit PCM, little endian.
//! * `S16BE` - Signed 16 integer bit PCM, big endian.
//! * `FLOAT32LE` - 32 bit IEEE floating point PCM, little endian.
//! * `FLOAT32BE` - 32 bit IEEE floating point PCM, big endian.
//! * `ALAW` - 8 bit a-Law.
//! * `ULAW` - 8 bit mu-Law.
//! * `S32LE` - Signed 32 bit integer PCM, little endian.
//! * `S32BE` - Signed 32 bit integer PCM, big endian.
//! * `S24LE` - Signed 24 bit integer PCM packed, little endian.
//! * `S24BE` - Signed 24 bit integer PCM packed, big endian.
//! * `S24_32LE` - Signed 24 bit integer PCM in LSB of 32 bit words, little endian.
//! * `S24_32BE` - Signed 24 bit integer PCM in LSB of 32 bit words, big endian.
//!
//! The floating point sample formats have the range from `-1.0` to `1.0`.
//!
//! # Sample Rates
//!
//! PulseAudio supports any sample rate between 1 Hz and 192000 Hz. There is no point trying to
//! exceed the sample rate of the output device though as the signal will only get downsampled,
//! consuming CPU on the machine running the server.
//!
//! # Channels
//!
//! PulseAudio supports up to 32 individual channels. The order of the channels is up to the
//! application, but they must be continuous. To map channels to speakers, see
//! [`::channelmap`](../channelmap/index.html).
//!
//! # Calculations
//!
//! The PulseAudio library contains a number of convenience functions to do calculations on sample
//! formats:
//!
//! * [`Spec::bytes_per_second`]: The number of bytes one second of audio will take given a sample
//!   format.
//! * [`Spec::frame_size`]: The size, in bytes, of one frame (i.e. one set of samples, one for each
//!   channel).
//! * [`Spec::sample_size`]: The size, in bytes, of one sample.
//! * [`Spec::bytes_to_usec`]: Calculate the time it would take to play a buffer of a certain size.
//!
//! [`Spec::bytes_per_second`]: struct.Spec.html#method.bytes_per_second
//! [`Spec::frame_size`]: struct.Spec.html#method.frame_size
//! [`Spec::sample_size`]: struct.Spec.html#method.sample_size
//! [`Spec::bytes_to_usec`]: struct.Spec.html#method.bytes_to_usec

use std;
use capi;
use std::ffi::{CStr, CString};
use std::borrow::Cow;
use time::MicroSeconds;

pub use capi::PA_CHANNELS_MAX as CHANNELS_MAX;
pub use capi::PA_RATE_MAX as RATE_MAX;

/// Sample format
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Format {
    /// Unsigned 8 Bit PCM.
    U8,
    /// 8 Bit a-Law.
    ALaw,
    /// 8 Bit mu-Law.
    ULaw,
    /// Signed 16 Bit PCM, little endian (PC).
    S16le,
    /// Signed 16 Bit PCM, big endian.
    S16be,
    /// 32 Bit IEEE floating point, little endian (PC), range -1.0 to 1.0.
    F32le,
    /// 32 Bit IEEE floating point, big endian, range -1.0 to 1.0.
    F32be,
    /// Signed 32 Bit PCM, little endian (PC).
    S32le,
    /// Signed 32 Bit PCM, big endian.
    S32be,
    /// Signed 24 Bit PCM packed, little endian (PC).
    S24le,
    /// Signed 24 Bit PCM packed, big endian.
    S24be,
    /// Signed 24 Bit PCM in LSB of 32 Bit words, little endian (PC).
    S24_32le,
    /// Signed 24 Bit PCM in LSB of 32 Bit words, big endian.
    S24_32be,

    /// An invalid value.
    Invalid = -1,
}

impl From<Format> for capi::pa_sample_format_t {
    fn from(f: Format) -> Self {
        unsafe { std::mem::transmute(f) }
    }
}

impl From<capi::pa_sample_format_t> for Format {
    fn from(f: capi::pa_sample_format_t) -> Self {
        unsafe { std::mem::transmute(f) }
    }
}

impl Default for Format {
    fn default() -> Self {
        Format::Invalid
    }
}

pub use self::ei_formats::*;

/// Endian-independent format identifiers
#[cfg(target_endian = "big")]
mod ei_formats {
    use super::Format;

    /// Signed 16 Bit PCM, native endian
    pub const SAMPLE_S16NE: Format = Format::S16be;
    /// 32 Bit IEEE floating point, native endian
    pub const SAMPLE_FLOAT32NE: Format = Format::F32be;
    /// Signed 32 Bit PCM, native endian
    pub const SAMPLE_S32NE: Format = Format::S32be;
    /// Signed 24 Bit PCM packed, native endian.
    pub const SAMPLE_S24NE: Format = Format::S24be;
    /// Signed 24 Bit PCM in LSB of 32 Bit words, native endian.
    pub const SAMPLE_S24_32NE: Format = Format::S24_32be;

    /// Signed 16 Bit PCM reverse endian
    pub const SAMPLE_S16RE: Format = Format::S16le;
    /// 32 Bit IEEE floating point, reverse endian
    pub const SAMPLE_FLOAT32RE: Format = Format::F32le;
    /// Signed 32 Bit PCM, reverse endian
    pub const SAMPLE_S32RE: Format = Format::S32le;
    /// Signed 24 Bit PCM, packed reverse endian.
    pub const SAMPLE_S24RE: Format = Format::S24le;
    /// Signed 24 Bit PCM, in LSB of 32 Bit words, reverse endian.
    pub const SAMPLE_S24_32RE: Format = Format::S24_32le;
}

/// Endian-independent format identifiers
#[cfg(target_endian = "little")]
mod ei_formats {
    use super::Format;

    /// Signed 16 Bit PCM, native endian
    pub const SAMPLE_S16NE: Format = Format::S16le;
    /// 32 Bit IEEE floating point, native endian
    pub const SAMPLE_FLOAT32NE: Format = Format::F32le;
    /// Signed 32 Bit PCM, native endian
    pub const SAMPLE_S32NE: Format = Format::S32le;
    /// Signed 24 Bit PCM packed, native endian.
    pub const SAMPLE_S24NE: Format = Format::S24le;
    /// Signed 24 Bit PCM in LSB of 32 Bit words, native endian.
    pub const SAMPLE_S24_32NE: Format = Format::S24_32le;

    /// Signed 16 Bit PCM, reverse endian
    pub const SAMPLE_S16RE: Format = Format::S16be;
    /// 32 Bit IEEE floating point, reverse endian
    pub const SAMPLE_FLOAT32RE: Format = Format::F32be;
    /// Signed 32 Bit PCM, reverse endian
    pub const SAMPLE_S32RE: Format = Format::S32be;
    /// Signed 24 Bit PCM, packed reverse endian.
    pub const SAMPLE_S24RE: Format = Format::S24be;
    /// Signed 24 Bit PCM, in LSB of 32 Bit words, reverse endian.
    pub const SAMPLE_S24_32RE: Format = Format::S24_32be;
}

/// A Shortcut for [`SAMPLE_FLOAT32NE`](ei_formats/constant.SAMPLE_FLOAT32NE.html)
pub const SAMPLE_FLOAT32: Format = SAMPLE_FLOAT32NE;

/// A sample format and attribute specification
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Spec {
    /// The sample format.
    pub format: Format,
    /// The sample rate. (e.g. 44100).
    pub rate: u32,
    /// Audio channels. (1 for mono, 2 for stereo, ...).
    pub channels: u8,
}

impl Spec {
    /// Initialize the specified sample spec.
    /// The sample spec will have a defined state but [`is_valid`](#method.is_valid) will fail for
    /// it.
    pub fn init(&mut self) {
        unsafe { capi::pa_sample_spec_init(std::mem::transmute(self)); }
    }

    /// Returns `true` when the sample type specification is valid
    pub fn is_valid(&self) -> bool {
        unsafe { capi::pa_sample_spec_valid(std::mem::transmute(self)) != 0 }
    }

    /// Returns `true` when the two sample type specifications match
    pub fn equal_to(&self, to: &Self) -> bool {
        unsafe { capi::pa_sample_spec_equal(std::mem::transmute(self), std::mem::transmute(to)) != 0 }
    }

    /// Returns the amount of bytes that constitute playback of one second of audio, with the
    /// specified sample type.
    pub fn bytes_per_second(&self) -> usize {
        unsafe { capi::pa_bytes_per_second(std::mem::transmute(self)) }
    }

    /// Returns the size of a frame
    pub fn frame_size(&self) -> usize {
        unsafe { capi::pa_frame_size(std::mem::transmute(self)) }
    }

    /// Returns the size of a sample
    pub fn sample_size(&self) -> usize {
        unsafe { capi::pa_sample_size(std::mem::transmute(self)) }
    }

    /// Calculate the time it would take to play a buffer of the specified size.
    /// The return value will always be rounded down for non-integral return values.
    pub fn bytes_to_usec(&self, length: u64) -> MicroSeconds {
        MicroSeconds(unsafe { capi::pa_bytes_to_usec(length, std::mem::transmute(self)) })
    }

    /// Calculates the size of a buffer required, for playback duration of the time specified.
    /// The return value will always be rounded down for non-integral return values.
    pub fn usec_to_bytes(&self, t: MicroSeconds) -> usize {
        unsafe { capi::pa_usec_to_bytes(t.0, std::mem::transmute(self)) }
    }

    /// Pretty print a sample type specification to a string
    pub fn print(&self) -> String {
        const PRINT_MAX: usize = capi::PA_SAMPLE_SPEC_SNPRINT_MAX;
        let mut tmp = Vec::with_capacity(PRINT_MAX);
        unsafe {
            capi::pa_sample_spec_snprint(tmp.as_mut_ptr(), PRINT_MAX, std::mem::transmute(self));
            CStr::from_ptr(tmp.as_mut_ptr()).to_string_lossy().into_owned()
        }
    }
}

/// Returns `true` if the given integer is a valid sample format.
///
/// With pure Rust code, this would be enforced natively through use of the
/// [`Format`](enum.Format.html) enum, but this function may remain useful for miscellaneous int
/// values from less reliable sources.
pub fn format_is_valid(format: u32) -> bool {
    unsafe { capi::pa_sample_format_valid(format) != 0 }
}

/// Returns `true` if the rate is within the supported range.
pub fn rate_is_valid(rate: u32) -> bool {
    unsafe { capi::pa_sample_rate_valid(rate) != 0 }
}

/// Returns `true` if the channel count is within the supported range.
pub fn channels_are_valid(channels: u8) -> bool {
    unsafe { capi::pa_channels_valid(channels) != 0 }
}

/// Pretty print a byte size value (i.e. "2.5 MiB")
pub fn bytes_print(bytes: u32) -> String {
    const PRINT_MAX: usize = capi::PA_BYTES_SNPRINT_MAX;
    let mut tmp = Vec::with_capacity(PRINT_MAX);
    unsafe {
        capi::pa_bytes_snprint(tmp.as_mut_ptr(), PRINT_MAX, bytes);
        CStr::from_ptr(tmp.as_mut_ptr()).to_string_lossy().into_owned()
    }
}

impl Format {
    /// Similar to [`Spec::sample_size`](struct.Spec.html#method.sample_size) but take a sample
    /// format instead of full sample spec.
    pub fn size(&self) -> usize {
        unsafe { capi::pa_sample_size_of_format((*self).into()) }
    }

    /// Returns a descriptive string for the specified sample format.
    pub fn to_string(&self) -> Option<Cow<'static, str>> {
        let ptr = unsafe { capi::pa_sample_format_to_string((*self).into()) };
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { CStr::from_ptr(ptr).to_string_lossy() })
    }

    /// Parse a sample format text. Inverse of [`to_string`](#method.to_string).
    pub fn parse(format: &str) -> Self {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_format = CString::new(format.clone()).unwrap();
        unsafe { capi::pa_parse_sample_format(c_format.as_ptr()).into() }
    }

    /// Returns `true` when the specified format is little endian, `false` if big endian. Returns
    /// `None` when endianness does not apply to this format, or if unknown.
    pub fn is_le(&self) -> Option<bool> {
        match unsafe { capi::pa_sample_format_is_le((*self).into()) } {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }

    /// Returns `true` when the specified format is big endian, `false` if little endian. Returns
    /// `None` when endianness does not apply to this format, or if unknown.
    pub fn is_be(&self) -> Option<bool> {
        match unsafe { capi::pa_sample_format_is_be((*self).into()) } {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }

    /// Is format native endian?
    ///
    /// Returns `true` when the specified format is native endian, `false` when not. Returns `None`
    /// when endianness does not apply to the specified format, or endianness is unknown.
    #[cfg(target_endian = "big")]
    pub fn is_ne(&self) -> Option<bool> {
        Format::is_be(self)
    }
    /// Is format native endian?
    ///
    /// Returns `true` when the specified format is native endian, `false` when not. Returns `None`
    /// when endianness does not apply to the specified format, or endianness is unknown
    #[cfg(target_endian = "little")]
    pub fn is_ne(&self) -> Option<bool> {
        Format::is_le(self)
    }

    /// Is format reverse of native endian?
    ///
    /// Returns `true` when the specified format is reverse endian, `false` when not. Returns `None`
    /// when endianness does not apply to the specified format, or endianness is unknown.
    #[cfg(target_endian = "big")]
    pub fn is_re(&self) -> Option<bool> {
        Format::is_le(self)
    }
    /// Is format reverse of native endian?
    ///
    /// Returns `true` when the specified format is reverse endian, `false` when not. Returns `None`
    /// when endianness does not apply to the specified format, or endianness is unknown.
    #[cfg(target_endian = "little")]
    pub fn is_re(&self) -> Option<bool> {
        Format::is_be(self)
    }
}
