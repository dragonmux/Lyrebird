// SPDX-License-Identifier: BSD-3-Clause

use std::ffi::{c_char, c_float, c_int, c_void};

#[repr(C)]
pub struct FileInfo
{
	__private: c_void,
}

#[repr(u32)]
pub enum AudioType
{
	OggVorbis = 1,
	FLAC = 2,
	Wave = 3,
	M4A = 4,
	AAC = 5,
	MP3 = 6,
	ImpulseTracker = 7,
	MusePack = 8,
	WavPack = 9,
	OptimFROG = 10,
	RealAudio = 11,
	WMA = 12,
	MOD = 13,
	S3M = 14,
	STM = 15,
	AON = 16,
	FC1x = 17,
	OggOpus = 18,
	SNDH = 19,
}

extern "C"
{
	// General API functions
	pub fn audioCloseFile(audioFile: *mut c_void) -> c_int;
	pub fn isAudio(fileName: *const c_char) -> bool;

	// Read (decode) API functions
	pub fn audioOpenR(fileName: *const c_char) -> *mut c_void;
	pub fn audioGetFileInfo(audioFile: *mut c_void) -> *const FileInfo;
	pub fn audioFillBuffer(audioFile: *mut c_void, buffer: *mut c_void, length: u32) -> i64;

	// Playback API functions
	pub fn audioPlay(audioFile: *mut c_void) -> c_void;
	pub fn audioPause(audioFile: *mut c_void) -> c_void;
	pub fn audioStop(audioFile: *mut c_void) -> c_void;

	pub fn audioDefaultLevel(level: c_float) -> c_void;

	// Write (encode) API functions
	pub fn audioOpenW(fileName: *const c_char, audioType: AudioType) -> *mut c_void;
	pub fn audioSetFileInfo(audioFile: *mut c_void, fileInfo: *const FileInfo) -> bool;
	pub fn audioWriteBuffer(audioFile: *mut c_void, buffer: *const c_void, length: i64) -> i64;

	// File information API functions
	pub fn audioFileTotalTime(fileInfo: *const FileInfo) -> u64;
	pub fn audioFileBitsPerSample(fileInfo: *const FileInfo) -> u32;
	pub fn audioFileBitRate(fileInfo: *const FileInfo) -> u32;
	pub fn audioFileChannels(fileInfo: *const FileInfo) -> u8;
	pub fn audioFileTitle(fileInfo: *const FileInfo) -> *const c_char;
	pub fn audioFileArtist(fileInfo: *const FileInfo) -> *const c_char;
	pub fn audioFileAlbum(fileInfo: *const FileInfo) -> *const c_char;
	pub fn audioFileOtherCommentsCount(fileInfo: *const FileInfo) -> usize;
	pub fn audioFileOtherComment(fileInfo: *const FileInfo) -> *const c_char;

	// pub ExternalPlayback: u8;
	// pub ToPlayback: u8;
}
