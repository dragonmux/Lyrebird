from urwid import SelectableIcon, WidgetWrap, FLOW, ACTIVATE, connect_signal
from urwid.util import is_mouse_press

__all__ = ('LyrebirdButton',)

class LyrebirdButton(WidgetWrap):
	def sizing(self):
		return frozenset([FLOW])

	signals = ["click"]

	def __init__(self, label, on_press = None, user_data = None):
		self._label = SelectableIcon("", 0)
		super().__init__(self._label)

		# The old way of listening for a change was to pass the callback
		# in to the constructor.  Just convert it to the new way:
		if on_press:
			connect_signal(self, 'click', on_press, user_data)
		self.set_label(label)

	def set_label(self, label):
		self._label.set_text(label)

	def get_label(self):
		return self._label.text
	label = property(get_label)

	def keypress(self, size, key):
		if self._command_map[key] != ACTIVATE:
			return key
		self._emit('click')

	def mouse_event(self, size, event, button, x, y, focus):
		if button != 1 or not is_mouse_press(event):
			return False
		self._emit('click')
		return True
