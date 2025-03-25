# SPDX-License-Identifier: BSD-3-Clause

libAudioRustSrcs = structured_sources(
	'lib.rs'
)

libAudio = subproject(
	'libAudio',
	default_options: [
		'default_library=static',
	],
)# .get_variable(
# 	'libAudio_dep'
# )

libAudioRust = static_library(
	'lyrebird',
	libAudioRustSrcs,
	# dependencies: [libAudio],
)
