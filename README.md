# Sverg

Sverg is an experimental image editor that leverages the power of modern
hardware so that you can create images as if they were raster
images (eg painting with a brush), but edit them as if they were
vector images.

## Current Status

Functional but Useless. You can draw on a canvas with a single brush with configurable
color/size. The brush is fixed as a spiral shape, and it can only save/load
to a hard-coded filepath.

Currently performance is pretty poor (on my laptop with an iGPU) because:
 - The depgraph doesn't cache anything between frames - so it re-renders the entire 
   image from scratch each frame
 - There are lots of texture copies. Every brush stroke copies the entire canvas texture 
   (gasp). This makes the depgraph execution very safe, but means we are doing lots 
   of extra work 
     - Currently the default canvas resolution is 3840 x 2160. If this can be somewhat performant
       on my laptop, then ... yay!

See the TODO file for more details


## License.
Undecided. Probably MIT or something like that.


### The very initial plan:
Python provides the UI and feeds events into the application. This makes
it very easy to customize for different displays etc.

The rust library provides the API's for saving, loading, drawing etc.

There are some edge cases about custom widgets (eg gradient tools), but to me these are part of
the GUI and should reside in python.


1920 * 1080 * 4 * 4 = 33.2 Mb
Compared to modern VRAM, this is tiny, we can probably store some 60 layers
even on low end hardware.

The plan:
 -> A fixed number of "canvas-bitmaps" in the GPU, determined by some setting (or perhaps based on measuring VRAM?)
 -> The dependency Tree figures out how best to utilize these "canvas-bitmaps" by placing them at critical junctions
