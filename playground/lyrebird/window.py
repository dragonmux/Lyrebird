from urwid import *
from .widgets import LyrebirdButton
from .panes import *

__all__ = ('mainWindow',)

class MainWindow(Frame):
	playIcon = '\u25B6'
	pauseIcon = '\u23F8'
	stopIcon = '\u23F9'

	def __init__(self):
		self._header = AttrMap(
			Columns(
				(
					Text(('headerEntry', ' Lyrebird'), wrap = 'clip'),
					('fixed', 1, Text('\u2502')),
					LyrebirdButton([('headerNumber', '1'), ('headerEntry', ' Tree')]),
					('fixed', 1, Text('\u2502')),
					LyrebirdButton([('headerNumber', '2'), ('headerEntry', ' Artists')]),
					('fixed', 1, Text('\u2502')),
					LyrebirdButton([('headerNumber', '3'), ('headerEntry', ' Albums')]),
					('fixed', 1, Text('\u2502')),
					LyrebirdButton([('headerNumber', '4'), ('headerEntry', ' Options')]),
					('fixed', 1, Text('\u2502')),
					LyrebirdButton([('headerNumber', '5'), ('headerEntry', ' Playlist')]),
				),
				dividechars = 0),
			'header'
		)

		self._footer = AttrMap(
			Columns(
				(
					('weight', 6, Text(' Nothing playing', wrap = 'ellipsis')),
					('fixed', 1, Text('\u2502')),
					('weight', 1, Text('??:??/??:??', align = 'center', wrap = 'clip')),
					('fixed', 1, Text('\u2502')),
					('weight', 5, Text('No errors', wrap = 'ellipsis')),
				),
				dividechars = 0),
			'footer'
		)

		self._panes = {
			'tree': LibraryTree()
		}

		super().__init__(self._panes['tree'], header = self.header, footer = self.footer)
