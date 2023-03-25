# Maph
## Overview
Maph is a personal library I've developed mostly for use with computer graphics, although I ended up including a number of components unrelated
to this, mostly for fun! Right now there's three major components - a geometry library with generic matrix and vector types as well as implementing
Quaternions and Dual Quaternions and several other helper functions for calculating transformations, a numerical library that implements its own
rational number type, a number of handy factorisaton helper functions and surds, for no real good reason except that I could. Finally there's a 
short cg library using bytemuck to easily convert the matrix and vector types from the geometry library into repr(C) useable types for rendering.