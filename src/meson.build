# SPDX-License-Identifier: BSD-3-Clause

lyrebirdSrcs = structured_sources(
	'main.rs'
)

# libAudio = subproject(
# 	'libAudio',
# 	default_options: [
# 		'default_library=static',
# 	],
# ).get_variable(
# 	'libAudio_dep'
# )

lyrebird = executable(
	'lyrebird',
	lyrebirdSrcs,
	# dependencies: [libAudio],
)
