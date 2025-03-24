from urwid import *

class LibraryTree(Columns):
	treeNodeIcon = '\u2514'
	treeLeafIcon = '\u251C'

	def __init__(self):
		self._dirWalker = SimpleFocusListWalker([
			Text(('activeEntry', f'   {self.treeLeafIcon} Chip')),
			Text(f'   {self.treeLeafIcon} Data_Airlines-The_Kife_(DATA013)'),
		])
		self._dirPath = Text(f' {self.treeNodeIcon} /pool/shiro/Music', wrap = 'ellipsis')
		self._dirPane = Frame(ListBox(self._dirWalker), header = self._dirPath)
		self._fileWalker = SimpleFocusListWalker([])

		super().__init__(
			(
				('weight', 1, LineBox(self._dirPane, title = 'Directory Tree', title_align = 'left')),
				('weight', 2, LineBox(ListBox(self._fileWalker), title = 'Files', title_align = 'left'))
			),
			dividechars = 0
		)
