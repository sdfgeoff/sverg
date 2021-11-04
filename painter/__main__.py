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

import painter_core




def create_canvas():
    gl_area = Gtk.GLArea()
    gl_area.set_hexpand(True)
    gl_area.set_vexpand(True)
    return gl_area


class Painter():
    def __init__(self, app):
        self.window = Gtk.ApplicationWindow(application=app)
        self.overlay = Gtk.Overlay()

        # Create the area on the screen to paint in
        self.canvas = create_canvas()
        self.window.set_child(self.overlay)
        self.overlay.set_child(self.canvas)

        # Create the actual engine that paints on the canvas.
        # For it to grab a handle to openGL, the GLArea needs to be crrent and the window
        # needs to be show.
        self.canvas.make_current()
        self.window.present()
        self.core = painter_core.PainterCore()
        
        # Todo: should the core contain these by default rather than letting python create them?
        #self.renderer = painter_core.PainterRenderer()
        self.context = painter_core.EditContext()
        self.brush_tool = painter_core.BrushTool()
        self.renderer = None
        self.brush_tool.set_brush_id(self.context.image.brushes.list_ids()[0]) # TODO: Is there a better way to do this binding between tools and context?
        self.context.select_layer(self.context.image.layers.list_ids()[0]) # TODO: Is there a better way to select a layer?
        
        # Now that we have the core, we can bind things to it.
        self.toggle_ui_button = create_toggle_ui_button()
        self.top_bar = create_top_bar()
        
        self.toggle_ui_button.connect("clicked", lambda _: self.top_bar.hide() if self.top_bar.get_visible() else self.top_bar.show())
        
        self.overlay.add_overlay(self.toggle_ui_button)
        self.overlay.add_overlay(self.top_bar)
        
        self.setup_stylus()

        self.canvas.connect("render", self.render)
        self.window.present()

        
    def setup_stylus(self):
        gesture = Gtk.GestureStylus()
        self.canvas.add_controller(gesture)
        gesture.connect("down", self.stylus_down)
        gesture.connect("motion", self.stylus_move)
        gesture.connect("up", self.stylus_up)

    def stylus_down(self, event, x, y):
        pressure = event.get_axis(Gdk.AxisUse.PRESSURE).value
        self.brush_tool.start_stroke(self.context, x, y, pressure)
        self.canvas.queue_draw()
        
    def stylus_move(self, event, x, y):
        pressure = event.get_axis(Gdk.AxisUse.PRESSURE).value
        self.brush_tool.continue_stroke(self.context, x, y, pressure)
        self.canvas.queue_draw()
    
    def stylus_up(self, event, x, y):
        self.brush_tool.end_stroke()
        self.canvas.queue_draw()

    def render(self, area, ctx):
        ctx.make_current()
        if self.renderer is None:
            self.renderer = painter_core.PainterRenderer()
        self.renderer.render(self.context)
        return True


def create_toggle_ui_button():
    toggle_ui_button = Gtk.Button.new_with_label('UI')
    toggle_ui_button.set_halign(Gtk.Align.END)
    toggle_ui_button.set_hexpand(False)
    toggle_ui_button.set_valign(Gtk.Align.START)
    toggle_ui_button.set_vexpand(False)
    return toggle_ui_button


def create_top_bar():
    top_box = Gtk.Box()
    test_button1 = Gtk.Button.new_with_label('Test1')
    test_button2 = Gtk.Button.new_with_label('Test2')
    
    top_box.set_halign(Gtk.Align.CENTER)
    top_box.set_hexpand(False)
    top_box.set_valign(Gtk.Align.START)
    top_box.set_vexpand(False)
    
    top_box.append(test_button1)
    top_box.append(test_button2)
    
    return top_box
    




def start():
    app = Gtk.Application(application_id='com.example.GtkApplication')
    app.connect('activate', Painter)
    app.run(None)

if __name__ == "__main__":
    start()
