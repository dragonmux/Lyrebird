from urwid import MainLoop, ExitMainLoop
from .window import MainWindow

def quitHandler(key):
	if key in ('q', 'Q'):
		raise ExitMainLoop()

lyrebirdPalette = (
	('header', 'dark blue', 'black'),
	('headerEntry', 'dark blue', 'black', None, 'dark green', 'black'),
	('headerNumber', 'light blue', 'black'),

	('footer', 'dark blue', 'black'),
)

mainWindow = MainWindow()
MainLoop(
	mainWindow,
	palette = lyrebirdPalette,
	unhandled_input = quitHandler
).run()
