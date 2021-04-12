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

rSON = dependency(
	'rSON',
	required: true,
	version: '>=0.4.1',
	fallback: ['rSON', 'rSON_dep']
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
	dependencies: [substrate, rSON, libAudio],
	gnu_symbol_visibility: 'inlineshidden',
	implicit_include_directories: false,
	pic: true,
	install: true
)
