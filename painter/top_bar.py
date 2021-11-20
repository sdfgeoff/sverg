import gi
gi.require_version('Gtk', '4.0')
from gi.repository import Gtk, Gdk



class TopBar(Gtk.Box):
    def __init__(self, painter):
        super().__init__()
        self.painter = painter
        
        
        load_button = Gtk.Button.new_with_label('Open')
        save_button = Gtk.Button.new_with_label('Save')
        new_button = Gtk.Button.new_with_label('New')
        
        new_button.connect("clicked", lambda _: painter.new_image())
        save_button.connect("clicked", lambda _: painter.save_image())
        load_button.connect("clicked", lambda _: painter.load_image())
        self.set_halign(Gtk.Align.CENTER)
        self.set_hexpand(False)
        self.set_valign(Gtk.Align.START)
        self.set_vexpand(False)
        
        self.append(save_button)
        self.append(load_button)
        self.append(new_button)
        
        

def create_top_bar(painter):
    return TopBar(painter)
