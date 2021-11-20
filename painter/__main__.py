"""
I wanted to get input from my laptops active stylus (Wacom I think).
To do so took an hour or so of trawling documentation and random
source code snippets.

Eventually I discovered that only certain widgets report the pressure
correctly. For example a button will only give a single value for
pressure - the pressure when the button was initially clicked.

So anyway, here is the most trivial app that reads the pressure from a
graphics tablet using python and GTK4.
"""
# Load Gtk
import gi
gi.require_version('Gtk', '4.0')
from gi.repository import Gtk, Gdk
import os

import painter_core
import time

from top_bar import create_top_bar
from canvas import create_canvas
from right_side_bar import create_right_bar

CSS_FILE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "style.css")

class Painter():
    def __init__(self, app):
        self.window = Gtk.ApplicationWindow(application=app)
        self.overlay = Gtk.Overlay()

        # Create the area on the screen to paint in
        self.canvas = create_canvas(self)
        self.window.set_child(self.overlay)
        self.overlay.set_child(self.canvas)

        # Create the actual engine that paints on the canvas.
        # For it to grab a handle to openGL, the GLArea needs to be crrent and the window
        # needs to be show.
        self.canvas.make_current()
        self.window.present()
        self.core = painter_core.PainterCore()
        
        self.context = painter_core.EditContext()
        self.brush_tool = painter_core.BrushTool()

        self.toggle_ui_button = create_toggle_ui_button()
        self.top_bar = create_top_bar(self)
        self.right_bar = create_right_bar(self)
        
        self.ui_visible = True
        self.toggle_ui_button.connect("clicked", lambda _: self.toggle_ui())
        
        self.overlay.add_overlay(self.top_bar)
        self.overlay.add_overlay(self.right_bar)
        self.overlay.add_overlay(self.toggle_ui_button) # Should come last so it is always on top
        
        self._context_changed()
        self.window.present()
    
    def new_image(self):
        self.context = self.core.new_image()
        self._context_changed()

    def save_image(self):
        print("Saving")
        self.core.save(self.context, "test.sveg")
    
    def load_image(self):
        print("Loading")
        self.context = self.core.load("test.sveg")
        self._context_changed()
    
    def _context_changed(self):
        self.brush_tool.set_brush_id(self.context.image.brushes.list_ids()[0]) # TODO: Is there a better way to do this binding between tools and context?
        self.context.select_layer(self.context.image.layers.list_ids()[0]) # TODO: Is there a better way to select a layer?
        self.canvas.queue_draw()

    def toggle_ui(self):
        self.ui_visible = not self.ui_visible
        if self.ui_visible:
            self.top_bar.show()
            self.right_bar.show()
        else:
            self.top_bar.hide()
            self.right_bar.hide()
        


def create_toggle_ui_button():
    toggle_ui_button = Gtk.Button.new_with_label('◯')
    toggle_ui_button.set_halign(Gtk.Align.END)
    toggle_ui_button.set_hexpand(False)
    toggle_ui_button.set_valign(Gtk.Align.START)
    toggle_ui_button.set_vexpand(False)
    return toggle_ui_button
    


def start():
    app = Gtk.Application(application_id='com.example.GtkApplication')
    app.connect('activate', Painter)
    app.run(None)

if __name__ == "__main__":
    start()
