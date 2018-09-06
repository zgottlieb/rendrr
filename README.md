# rendrr

rendrr is a toy rendering engine that can parse and render HTML, CSS, and text to a window in your OS. The project is intended to be used as a tool for hands-on exploration of how browsers work; it exists as a sandbox in which to add, change, remove features over time and to dive into the concepts and technologies that make up a browser.

The code for rendrr is largely based on https://github.com/mbrubeck/robinson and the accompanying tutorial, though many parts are written from scratch and/or extend past the robinson code. Any use of code from robinson was done as a temporary solution to allow for focus on building out other features.

Current working features of rendrr include:
- parse HTML and CSS
- basic implementation of CSS box model (block/inline, border, padding, margin)
- render text and color (borders, font colors, background colors)
- render to OS window (using SDL2)









