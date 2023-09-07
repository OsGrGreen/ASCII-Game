# Idea for shader

If resolution of screen is a by b pixels then (given we use 8 x 8 chars), we create two textures.

These textures are a/8 x b/8 pixels big. These textures are for storing colour (colour_texture) and the shape (shape_texture).

The colour_texture is self explanatory, since it just stores the colour the given ASCII char should be coloured.
The shape_texture is a little bit more complicated. For simplicity lets limit the number of chars to 256 (one colour) (dont know how floats will work with this :/ )

Each char is represented by one number, for example { '.'=1, ','=2, ';'=3, '=' = 4, '#' = 5, '@' = 6}

These numbers we store in any of the colour channels, lets say green, because green looks good.

then if we want to render the following string: "###@@@", then the texture will contain "555666"

Then we will have another texture (font_texture) where the binary data of all these ascii chars will be. For example the har "#" will look like something like:
01100110
11111111
11111111
01100110
01100110
11111111
11111111
01100110

Where 1 is white and black is 0. Then we create a function shapeOf(i, x, y), which returns the pixel of shape i (i e {1,2,3,4,5,6,...}), at the posistion x%8, y%8. Mod is used for getting the posisiton of the pixel, in the char, and not on screen.

function shapeOf(i, x, y):
    is_lit = font_texture(i*(x%8),i*(y%8))
    return is_lit;

When rendering we create a new texture and do the following (maybe creating a new texture is unneccesary):
    for each pixel(x,y):
        colour = colour_texture(x/8,y/8).rgba
        //Should it be lit?
        shape = shape_texture(x/8, y/8).r
        colour = colour*shapeOf(shape, x, y)
        return colour



## Response:
In summary, your proposed approach may work for rendering ASCII characters on the GPU, but it may not be the most efficient or scalable solution for a game. Depending on your specific requirements and performance constraints, you may want to explore alternative methods such as bitmap fonts, vector fonts, or GPU-based text rendering libraries designed for games.