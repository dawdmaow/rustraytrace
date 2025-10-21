Path tracing renderer, based on Peter Shirley's "Ray Tracing in One Weekend".

- Materials: Lambertian (diffuse), Metal (reflective), and Dielectric (glass/refractive)
- Depth of field (defocus blur)
- Anti-aliasing: Multiple samples per pixel for smooth edges
- Scene: spheres with various materials
- PPM image format

## Running
```bash
cargo run --release > image.ppm
```

The program outputs a PPM image to stdout, so make sure to redirect it to a file.

## Viewing the Output

Convert PPM to PNG if PPM is not easily viewable (ImageMagick):
```bash
convert image.ppm image.png
```