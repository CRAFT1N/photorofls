# Photorofls
Simple program to do some manipulations with images:
1. Upscale image by int number
2. Downscale image by int number
3. Apply Floyd-Steinberg dithering on image
4. Invert image colors
5. Apply sine function on image(idk really)
6. Apply Sobel filter on image
## Examples
1. Upscale `./photorofls -f mgy.jpg -u 2` ![](img/mgy.jpg_2x.png)
2. Downscale `./photorofls -f mgy.jpg -l 2` ![](img/mgy.jpg_2x_c.png)
3. Floyd-Steinberg `./photorofls -f mgy.jpg -d` ![](img/mgy.jpg_d.png)
4. Invert `./photorofls -f mgy.jpg -i` ![](img/mgy.jpg_i.png)
5. Sine `./photorofls -f mgy.jpg -t` ![](img/mgy.jpg_t.png)
6. Sobel filter `./photorofls -f mgy.jpg -s` ![](img/mgy.jpg_s.png)