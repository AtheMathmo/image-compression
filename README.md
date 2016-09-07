# Image Compression

With rusty-machine!

This project has no purpose beyond showing off some functionality
of rusty-machine (and [clap](https://github.com/kbknapp/clap-rs)/[image](https://github.com/PistonDevelopers/image)).

---

This project provides a toy implementation of image compression using rusty-machine.
We use the [K-Means](https://en.wikipedia.org/wiki/K-means_clustering) algorithm to cluster
the colours in an image and produce a new image made up of only the mean of each cluster.

Original | Compressed
:-------:|:-----------:
![Original image](./img/flowers.jpg) | ![Compressed image](./img/compressed_10.jpg)

The compressed image above contains only 10 unique colours.

## Usage

The output below is thanks to [clap](https://github.com/kbknapp/clap-rs)!

```
./target/release/image-compression --help
Image Compression 0.1.0
James Lucas
Uses rusty-machine's K-Means model to compress images

USAGE:
    image-compression --input <FILE> --output <FILE> -c <INTEGER>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c <INTEGER>           The number of colors present in the output image
    -i, --input <FILE>     The input image path
    -o, --output <FILE>    The output image path
```

I'd recommend running this in `release` mode otherwise it will be pretty slow.
This is **not** an efficient compression algorithm and will take a while to run on large images.

Finally this doesn't really _compress_ the image at all! The output image is saved in whichever format
is specified by the `output` argument - but it _should_ be smaller in size.
