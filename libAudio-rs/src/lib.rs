// SPDX-License-Identifier: BSD-3-Clause
#![warn(clippy::unwrap_used)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_snake_case)]

use bindings::audioDefaultLevel;

pub mod audioFile;
mod bindings;
pub mod fileInfo;

pub fn setVolumeLevel(level: f32)
{
	unsafe { audioDefaultLevel(level) };
}
