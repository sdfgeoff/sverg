import gi
gi.require_version('Gtk', '4.0')
from gi.repository import Gtk, Gdk
import os

ICON_BRUSH_SIZE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "icons/BrushSize.svg")
ICON_BRUSH_OPACITY = os.path.join(os.path.dirname(os.path.abspath(__file__)), "icons/BrushOpacity.svg")
ICON_BRUSH_COLOR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "icons/BrushColor.svg")

class RightBar(Gtk.Box):
    def __init__(self, painter):
        super().__init__()
        self.painter = painter
        
        top_spacing = Gtk.Separator.new(Gtk.Orientation.HORIZONTAL)
        top_spacing.set_property('height-request', 50.0)
        self.append(top_spacing)

        self.color_button = Gtk.Button.new()
        color_icon = Gtk.Image.new_from_file(ICON_BRUSH_COLOR)
        self.append(self.color_button)
        self.color_button.set_child(color_icon)
        self.color_button.connect('clicked', lambda _: self.open_color_popup())

        self.color_selector_popover = Gtk.Popover.new()
        self.append(self.color_selector_popover)
        
        self.color_chooser = Gtk.ColorChooserWidget.new()
        self.color_chooser.set_property('show-editor', True)
        self.color_chooser.set_property('use-alpha', False)
        self.color_chooser.connect('notify::rgba', self.set_color)
        self.color_selector_popover.set_child(self.color_chooser)

        self.size_slider = Gtk.Scale.new_with_range(Gtk.Orientation.VERTICAL, 0.0, 1.0, 0.001)
        self.size_slider.set_vexpand(True)
        self.size_slider.set_inverted(True)
        self.size_slider.set_valign(Gtk.Align.FILL)
        self.size_slider.connect('change-value', self.set_size)
        self.append(self.size_slider)

        size_icon = Gtk.Image.new_from_file(ICON_BRUSH_SIZE)
        self.append(size_icon)

        self.opacity_slider = Gtk.Scale.new_with_range(Gtk.Orientation.VERTICAL, 0.0, 1.0, 0.001)
        self.opacity_slider.set_vexpand(True)
        self.opacity_slider.set_inverted(True)
        self.opacity_slider.set_valign(Gtk.Align.FILL)
        self.opacity_slider.connect('change-value', self.set_alpha)
        self.append(self.opacity_slider)

        opacity_icon = Gtk.Image.new_from_file(ICON_BRUSH_OPACITY)
        self.append(opacity_icon)
        
        self.set_halign(Gtk.Align.END)
        self.set_hexpand(False)
        self.set_valign(Gtk.Align.FILL)
        self.set_vexpand(True)
        self.set_orientation(Gtk.Orientation.VERTICAL)
        
        
        
        
    def open_color_popup(self):
        self.color_selector_popover.set_pointing_to(self.color_button.get_allocation())
        self.color_selector_popover.popup()

    def set_color(self, *args):
        new_color = self.color_chooser.get_rgba()

        self.painter.context.set_primary_color(
            new_color.red,
            new_color.green,
            new_color.blue,
            self.painter.context.color.a,
        )

    def set_alpha(self, *args):
        alpha = self.opacity_slider.get_value()
        self.painter.context.set_primary_color(
            self.painter.context.color.r,
            self.painter.context.color.g,
            self.painter.context.color.b,
            alpha,
        )

    def set_size(self, *args):
        size = self.size_slider.get_value()
        self.painter.brush_tool.size = size
        

        
        
        

def create_right_bar(painter):
    return RightBar(painter)
