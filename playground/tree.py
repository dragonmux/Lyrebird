from urwid import *

files = (
	('clcollab - Ice cleam (WIP)', '4m39s'),
	('Feel Good Inc (Chiptune Cover)', '3m36s'),
	('When I\'m With You 2015', '3m58s')
)

directories = (
	'Anthrax',
	'Asira',
	'Atrocity',
	'AViVA',
	'Burn In Silence',
	'Chip',
	'Cradle of Filth',
	'Data_Airlines-The_Kife_(DATA013)',
	'Disturbed Discography',
	'Dream Evil',
	'EVE',
	'Five Finger Death Punch',
	'Gorillaz',
	'Hazel',
	'Rammstein'
)

dirWalker = SimpleFocusListWalker([
	Text(f'\u251C {directory}', wrap = 'ellipsis')
	for directory in directories
])
dirPath = Text('/pool/shiro/Music', wrap = 'ellipsis')
dirPane = Frame(ListBox(dirWalker), header = dirPath)

fileWalker = SimpleFocusListWalker([
	Columns((
		Text(title, wrap = 'ellipsis'),
		Text(length, wrap = 'clip')
	), dividechars = 1)
	for title, length in files
])

body = Columns((dirPane, ListBox(fileWalker)), dividechars = 1)

header = Columns((
	Text('Lyrebird', wrap = 'clip'),
	Text([('light blue', '1'), ' Tree'], wrap = 'clip'),
	Text([('light blue', '2'), ' Artists'], wrap = 'clip'),
	Text([('light blue', '3'), ' Albums'], wrap = 'clip'),
	Text([('light blue', '4'), ' Playlist'], wrap = 'clip')
))

footer = Columns((
	Text('\u23F8 clcollab - Ice cleam (WIP)', wrap = 'ellipsis'),
	Text('01:58/04:39', align = 'center', wrap = 'clip'),
	Text('No errors', align = 'right', wrap = 'ellipsis')
))

mainWindow = Frame(body, header = header, footer = footer)
loop = MainLoop(mainWindow)
loop.run()
