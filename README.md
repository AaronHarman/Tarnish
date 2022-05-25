# Tarnish
A command-line tool for ruining your images.

Written in Rust, this tool takes in an image, performs a modification based on the command you give it, and saves the resulting image to a prescribed location, either creating or overwriting that image.

The command format is as follows:
**tarnish \<source file> \<destination file> \<command>** *[arguments]*

The number of arguments varies depending on the command.

## Commands
- **copy** : Simply copies the source image to the target image.
- **huerotate \<degrees>** : Rotates the hue of the image by the given number of degrees.
- **rgbreplace \<r> \<g> \<b>** : Takes in three hex-format colors for R, G, and B, and then replaces the RGB in the image with those three colors. You can think of it like developing a picture with different colors than red, green, and blue.
- **mosaic \<pieces>** : Generates a mosaic effect on the image, breaking it up into the given number of single-color cells. The cells are randomly positioned, so they should usually be placed pretty uniformly. The random distribution also means that if you get a result you do not like, you can call this again with the same number of pieces and get a different resulting image.

There are 2 test commands that don't do anything particularly useful to an end user.
- **errortest** : Intentionally throws an error, displaying an error message.
- **argerrortest** : Intentionally throws an invalid arguments error, displaying an error message.
