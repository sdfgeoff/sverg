# Load Gtk
import gi
gi.require_version('Gtk', '4.0')
from gi.repository import Gtk, Gdk

import painter_core
import time


class Canvas(Gtk.GLArea):
    def __init__(self, painter):
        super().__init__()
        
        self.painter = painter
        self.set_hexpand(True)
        self.set_vexpand(True)
        
        self.zoom_at_gesture_start = None
        self.angle_at_gesture_start = None
        self.translation_at_gesture_start = None
        self.time_at_stroke_start = None
        
        self.renderer = None
        
        stylus = Gtk.GestureStylus()
        self.add_controller(stylus)
        stylus.connect("down", self.stylus_down)
        stylus.connect("motion", self.stylus_move)
        stylus.connect("up", self.stylus_up)

        zoom = Gtk.GestureZoom()
        self.add_controller(zoom)
        zoom.connect("begin", self.scale_changed_start)
        zoom.connect("end", self.scale_changed_end)
        zoom.connect("scale-changed", self.scale_changed)

        rotate = Gtk.GestureRotate()
        self.add_controller(rotate)
        rotate.connect("begin", self.angle_changed_start)
        rotate.connect("end", self.angle_changed_end)
        rotate.connect("angle-changed", self.angle_changed)

        drag = Gtk.GestureDrag()
        self.add_controller(drag)
        drag.connect("begin", self.drag_start)
        drag.connect("end", self.drag_end)
        drag.connect("drag-update", self.drag_changed)
        drag.set_touch_only(True)

        self.connect("render", self.render)


    def drag_start(self, event, z):
        if self.translation_at_gesture_start is not None:
            print("Angle event starting twice!")
        self.translation_at_gesture_start = self.painter.context.canvas_transform.translation

    def drag_end(self, event, z):
        if self.translation_at_gesture_start is None:
            print("Angle event ending twice!")
        self.translation_at_gesture_start = None

    def drag_changed(self, event, new_x, new_y):
        if self.translation_at_gesture_start is None:
            print("Angle event did not start properly")
        x, y = self.translation_at_gesture_start
        alloc = self.get_allocation()
        dx = 2 * new_x / alloc.width
        dy = -2 * new_y / alloc.height
        dx *= alloc.width / alloc.height
        self.painter.context.manipulate_canvas(
            self.painter.context.canvas_transform.zoom,
            self.painter.context.canvas_transform.angle,
            [x + dx, y + dy]
        )
        self.queue_draw()
    
    
    def angle_changed_start(self, event, z):
        if self.angle_at_gesture_start is not None:
            print("Angle event starting twice!")
        self.angle_at_gesture_start = self.painter.context.canvas_transform.angle

    def angle_changed_end(self, event, z):
        if self.angle_at_gesture_start is None:
            print("Angle event ending twice!")
        self.angle_at_gesture_start = None

    def angle_changed(self, event, _angle, angle_delta):
        if self.angle_at_gesture_start is None:
            print("Angle event did not start properly")
        angle_delta *= -1  # We define our coordinate system as counter-clockwide positive, GTK has clockwise-positive
        self.painter.context.manipulate_canvas(
            self.painter.context.canvas_transform.zoom,
            self.angle_at_gesture_start + angle_delta,
            self.painter.context.canvas_transform.translation
        )
        self.queue_draw()

    def scale_changed_start(self, event, z):
        if self.zoom_at_gesture_start is not None:
            print("Zoom event starting twice!")
        self.zoom_at_gesture_start = self.painter.context.canvas_transform.zoom

    def scale_changed_end(self, event, z):
        if self.zoom_at_gesture_start is None:
            print("Zoom event ending twice!")
        self.zoom_at_gesture_start = None

    def scale_changed(self, event, zoom):
        if self.zoom_at_gesture_start is None:
            print("Zoom event did not start properly")
        self.painter.context.manipulate_canvas(
            self.zoom_at_gesture_start * zoom,
            self.painter.context.canvas_transform.angle,
            self.painter.context.canvas_transform.translation
        )
        self.queue_draw()

    def stylus_down(self, event, x, y):
        pressure = event.get_axis(Gdk.AxisUse.PRESSURE).value
        alloc = self.get_allocation()
        x = 2 * x / alloc.width - 1
        y = -2 * y / alloc.height + 1
        x *= alloc.width / alloc.height
        x, y = self.painter.context.screen_coords_to_canvas_coords(x, y)
        self.painter.brush_tool.start_stroke(self.painter.context, x, y, pressure)
        self.time_at_stroke_start = time.time()
        self.queue_draw()
        
    def stylus_move(self, event, x, y):
        if self.time_at_stroke_start == None:
            return
        pressure = event.get_axis(Gdk.AxisUse.PRESSURE).value
        alloc = self.get_allocation()
        x = 2 * x / alloc.width - 1
        y = -2 * y / alloc.height + 1
        x *= alloc.width / alloc.height
        x, y = self.painter.context.screen_coords_to_canvas_coords(x, y)
        self.painter.brush_tool.continue_stroke(self.painter.context, x, y, pressure, time.time() - self.time_at_stroke_start)
        self.queue_draw()
    
    def stylus_up(self, event, x, y):
        self.painter.brush_tool.end_stroke()
        self.time_at_stroke_start = None
        self.queue_draw()

    def render(self, area, ctx):
        ctx.make_current()
        if self.renderer is None:
            self.renderer = painter_core.PainterRenderer()
        self.renderer.render(self.painter.context)
        return True



def create_canvas(painter):
    return Canvas(painter)
