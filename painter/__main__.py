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



STROKE = []

def stylus_down(event, x, y):
    STROKE.append(True)
    print(event.get_axis(Gdk.AxisUse.PRESSURE), x, y)
    print(len(STROKE))
    


def on_activate(app):
    window = Gtk.ApplicationWindow(application=app)
    overlay = Gtk.Overlay()

    gl_area = Gtk.GLArea()
    gl_area.set_hexpand(True)
    gl_area.set_vexpand(True)

    gesture = Gtk.GestureStylus()
    gl_area.add_controller(gesture)
    gesture.connect("motion", stylus_down)
    
    toggle_ui_button = create_toggle_ui_button()
    top_bar = create_top_bar()
    
    toggle_ui_button.connect("clicked", lambda _: top_bar.hide() if top_bar.get_visible() else top_bar.show())
    
    window.set_child(overlay)
    overlay.set_child(gl_area)
    overlay.add_overlay(toggle_ui_button)
    overlay.add_overlay(top_bar)
    window.present()
    

    gl_area.make_current()
    core = painter_core.PainterCore()

    def render (area, ctx):
        print("Rendering (python)")
        ctx.make_current()
        core.render()
        return True
    gl_area.connect ("render", render)

    window.present()


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
    app.connect('activate', on_activate)
    app.run(None)

if __name__ == "__main__":
    start()
