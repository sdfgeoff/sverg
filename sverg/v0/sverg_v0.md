# Sverg v0 (Deprecated)
When I came up with the idea of sverg, I needed to de-risk it's implementation.
Are modern computers actually powerful enough that even a normal laptop can
run it performantly? To do so I developed a small editing program. This
editing program effectively could effectively dump it's memory to disk 
as binary in order to save files. (Cue horror noises). For a prototype
it worked, but it:
 - Is not extensible
 - Is not version-stable
 - Is not compiler-stable
 
etc. etc. 

One other issue with the v0 format was where various operations were
performed. When storing a stroke, v0 stored the path the stylus took 
and the pressure used (effectively storing the users input directly). 
This would mean that any non-trivial brush stroke information would 
have to be computed by the renderer based on the brush used to make the 
stroke. In turn, this means that any operation that requires noise (eg scatter,
variable opacity), would be dependant on the renderers implementation of a
noise algorithm, and minor changes to the renderer would risk changing the
generated image. The artist would then not be able to depend on his image
displaying the same across different versions and devices, even if the
file format could still be read.

But hey, v0 successfully de-risked the whole idea. It worked, and it could
be edited and rendered!
