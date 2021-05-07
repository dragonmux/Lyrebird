from urwid import *

__all__ = ('mainWindow',)

header = AttrMap(
	Columns(
		(
			Text(('headerEntry', ' Lyrebird'), wrap = 'clip'),
			Button([('headerNumber', '1'), ('headerEntry', ' Tree')]),
			Button([('headerNumber', '2'), ('headerEntry', ' Artists')]),
			Button([('headerNumber', '3'), ('headerEntry', ' Albums')]),
			Button([('headerNumber', '4'), ('headerEntry', ' Options')]),
			Button([('headerNumber', '5'), ('headerEntry', ' Playlist')]),
		),
		dividechars = 1),
	'header'
)

footer = AttrMap(
	Columns(
		(
			Text(' Nothing playing', wrap = 'ellipsis'),
			Text('??:??/??:??', align = 'center', wrap = 'clip'),
			Text('No errors', wrap = 'ellipsis'),
		)
	),
	'footer'
)

mainWindow = Frame(LineBox(Columns(())), header = header, footer = footer)
