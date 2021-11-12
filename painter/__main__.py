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
import time



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
        self.top_bar = create_top_bar(self)
        
        self.toggle_ui_button.connect("clicked", lambda _: self.top_bar.hide() if self.top_bar.get_visible() else self.top_bar.show())
        
        self.overlay.add_overlay(self.toggle_ui_button)
        self.overlay.add_overlay(self.top_bar)
        
        self.setup_canvas_events()

        # TODO: find a better place to store this
        self.zoom_at_gesture_start = None
        self.angle_at_gesture_start = None
        self.translation_at_gesture_start = None
        self.time_at_stroke_start = None

        
        self.window.present()

        
    def setup_canvas_events(self):
        stylus = Gtk.GestureStylus()
        self.canvas.add_controller(stylus)
        stylus.connect("down", self.stylus_down)
        stylus.connect("motion", self.stylus_move)
        stylus.connect("up", self.stylus_up)

        zoom = Gtk.GestureZoom()
        self.canvas.add_controller(zoom)
        zoom.connect("begin", self.scale_changed_start)
        zoom.connect("end", self.scale_changed_end)
        zoom.connect("scale-changed", self.scale_changed)

        rotate = Gtk.GestureRotate()
        self.canvas.add_controller(rotate)
        rotate.connect("begin", self.angle_changed_start)
        rotate.connect("end", self.angle_changed_end)
        rotate.connect("angle-changed", self.angle_changed)

        drag = Gtk.GestureDrag()
        self.canvas.add_controller(drag)
        drag.connect("begin", self.drag_start)
        drag.connect("end", self.drag_end)
        drag.connect("drag-update", self.drag_changed)
        drag.set_touch_only(True)

        self.canvas.connect("render", self.render)

        # TODO: Consider using get_last_event() so we don't need to store the *_at_gesture_start

    def drag_start(self, event, z):
        if self.translation_at_gesture_start is not None:
            print("Angle event starting twice!")
        self.translation_at_gesture_start = self.context.canvas_transform.translation

    def drag_end(self, event, z):
        if self.translation_at_gesture_start is None:
            print("Angle event ending twice!")
        self.translation_at_gesture_start = None

    def drag_changed(self, event, new_x, new_y):
        if self.translation_at_gesture_start is None:
            print("Angle event did not start properly")
        x, y = self.translation_at_gesture_start
        alloc = self.canvas.get_allocation()
        dx = 2 * new_x / alloc.width
        dy = -2 * new_y / alloc.height
        self.context.manipulate_canvas(
            self.context.canvas_transform.zoom,
            self.context.canvas_transform.angle,
            [x + dx, y + dy]
        )
        self.canvas.queue_draw()
    
    
    def angle_changed_start(self, event, z):
        if self.angle_at_gesture_start is not None:
            print("Angle event starting twice!")
        self.angle_at_gesture_start = self.context.canvas_transform.angle

    def angle_changed_end(self, event, z):
        if self.angle_at_gesture_start is None:
            print("Angle event ending twice!")
        self.angle_at_gesture_start = None

    def angle_changed(self, event, _angle, angle_delta):
        if self.angle_at_gesture_start is None:
            print("Angle event did not start properly")
        angle_delta *= -1  # We define our coordinate system as counter-clockwide positive, GTK has clockwise-positive
        self.context.manipulate_canvas(
            self.context.canvas_transform.zoom,
            self.angle_at_gesture_start + angle_delta,
            self.context.canvas_transform.translation
        )
        self.canvas.queue_draw()

    def scale_changed_start(self, event, z):
        if self.zoom_at_gesture_start is not None:
            print("Zoom event starting twice!")
        self.zoom_at_gesture_start = self.context.canvas_transform.zoom

    def scale_changed_end(self, event, z):
        if self.zoom_at_gesture_start is None:
            print("Zoom event ending twice!")
        self.zoom_at_gesture_start = None

    def scale_changed(self, event, zoom):
        if self.zoom_at_gesture_start is None:
            print("Zoom event did not start properly")
        self.context.manipulate_canvas(
            self.zoom_at_gesture_start * zoom,
            self.context.canvas_transform.angle,
            self.context.canvas_transform.translation
        )
        self.canvas.queue_draw()

    def stylus_down(self, event, x, y):
        pressure = event.get_axis(Gdk.AxisUse.PRESSURE).value
        alloc = self.canvas.get_allocation()
        x = 2 * x / alloc.width - 1
        y = -2 * y / alloc.height + 1
        self.time_at_stroke_start = time.time()
        self.brush_tool.start_stroke(self.context, x, y, pressure)
        self.canvas.queue_draw()
        
    def stylus_move(self, event, x, y):
        pressure = event.get_axis(Gdk.AxisUse.PRESSURE).value
        alloc = self.canvas.get_allocation()
        x = 2 * x / alloc.width - 1
        y = -2 * y / alloc.height + 1
        self.brush_tool.continue_stroke(self.context, x, y, pressure, time.time() - self.time_at_stroke_start)
        self.canvas.queue_draw()
    
    def stylus_up(self, event, x, y):
        self.brush_tool.end_stroke()
        self.time_at_stroke_start = 0.0
        self.canvas.queue_draw()

    def render(self, area, ctx):
        ctx.make_current()
        if self.renderer is None:
            self.renderer = painter_core.PainterRenderer()
        self.renderer.render(self.context)
        return True


def create_toggle_ui_button():
    toggle_ui_button = Gtk.Button.new_with_label('◯')
    toggle_ui_button.set_halign(Gtk.Align.END)
    toggle_ui_button.set_hexpand(False)
    toggle_ui_button.set_valign(Gtk.Align.START)
    toggle_ui_button.set_vexpand(False)
    return toggle_ui_button


def create_top_bar(painter):
    top_box = Gtk.Box()
    save_button = Gtk.Button.new_with_label('Save')
    load_button = Gtk.Button.new_with_label('Open')

    def save(_):
        print("Saving")
        painter.core.save(painter.context, "test.sveg")
        painter.canvas.queue_draw()
    
    def load(_):
        print("Loading")
        painter.context = painter.core.load("test.sveg")

        painter.brush_tool.set_brush_id(painter.context.image.brushes.list_ids()[0]) # TODO: Is there a better way to do this binding between tools and context?
        painter.context.select_layer(painter.context.image.layers.list_ids()[0]) # TODO: Is there a better way to select a layer?
        

        painter.canvas.queue_draw()


    save_button.connect("clicked", save)
    load_button.connect("clicked", load)
    top_box.set_halign(Gtk.Align.CENTER)
    top_box.set_hexpand(False)
    top_box.set_valign(Gtk.Align.START)
    top_box.set_vexpand(False)
    
    top_box.append(save_button)
    top_box.append(load_button)
    
    return top_box
    


def start():
    app = Gtk.Application(application_id='com.example.GtkApplication')
    app.connect('activate', Painter)
    app.run(None)

if __name__ == "__main__":
    start()
