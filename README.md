# huerotate-like-webkit-rust


* Usage:
`hue-rotate 180 some-image-file.jpg some-output-file.png`


```
# c++
mojos-MacBook-Pro:huerotatewebkit mojomacssd$ time bin/huerotate 180 /Users/mojomacssd/Pictures/out.png  test3.png
INFO:       FreeImage version: 3.17.0
LOADING:    /Users/mojomacssd/Pictures/out.png
DIMENSIONS: 2605x2624
SUCCESS:    Image successfully processed to test3.png

real	0m1.753s
user	0m1.698s
sys	0m0.037s


# rust
mojos-MacBook-Pro:hue-rotate mojomacssd$ time target/release/hue-rotate 180 /Users/mojomacssd/Pictures/out.png  test3.png
saved to: "test3.png"
PT0.411342066S seconds for whatever you did.

real	0m0.426s
user	0m0.384s
sys	0m0.035s
```
