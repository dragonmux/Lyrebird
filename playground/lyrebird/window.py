from urwid import *
from .widgets import LyrebirdButton

__all__ = ('mainWindow',)

class MainWindow(Frame):
	def __init__(self):
		self._header = AttrMap(
			Columns(
				(
					Text(('headerEntry', ' Lyrebird'), wrap = 'clip'),
					LyrebirdButton([('headerNumber', '1'), ('headerEntry', ' Tree')]),
					LyrebirdButton([('headerNumber', '2'), ('headerEntry', ' Artists')]),
					LyrebirdButton([('headerNumber', '3'), ('headerEntry', ' Albums')]),
					LyrebirdButton([('headerNumber', '4'), ('headerEntry', ' Options')]),
					LyrebirdButton([('headerNumber', '5'), ('headerEntry', ' Playlist')]),
				),
				dividechars = 1),
			'header'
		)

		self._footer = AttrMap(
			Columns(
				(
					('weight', 6, Text(' Nothing playing', wrap = 'ellipsis')),
					('weight', 1, Text('??:??/??:??', align = 'center', wrap = 'clip')),
					('weight', 5, Text('No errors', wrap = 'ellipsis')),
				),
				dividechars = 1),
			'footer'
		)

		super().__init__(LineBox(Columns(())), header = self.header, footer = self.footer)
