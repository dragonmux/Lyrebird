# SPDX-License-Identifier: BSD-3-Clause

lyrebirdSrcs = [
	'lyrebird.cxx'
]

substrate = dependency(
	'substrate',
	required: true,
	version: '>=0.0.1',
	fallback: ['substrate', 'substrate_dep'],
	default_options: [
		'build_tests=false'
	]
)

libAudio = dependency(
	'libAudio',
	required: true,
	version: '>=0.5.2',
	fallback: ['libAudio', 'libAudio']
)

lyrebird = executable(
	'lyrebird',
	lyrebirdSrcs,
	include_directories: lyrebirdInclude,
	dependencies: [substrate, libAudio]
)
