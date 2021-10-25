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
