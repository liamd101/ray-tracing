# Rust Ray Tracer

A physically-based renderer implemented in Rust, based on Peter Shirley's "Ray Tracing in a Weekend" and "Ray Tracing: The Next Week" book series.

<img src="images/bouncing-spheres.ppm" alt="Bouncing Spheres" width="400"/>

## Overview

This project implements a ray tracer that simulates how light interacts with objects in a 3D scene. It features:

- Path tracing with Monte Carlo sampling for global illumination
- Various material types (Lambertian diffuse, metal, dielectric/glass)
- Texture mapping (solid colors, checkered patterns, procedural noise)
- Light sources (diffuse area lights)
- Geometric primitives (spheres, quadrilaterals, boxes)
- Motion blur for moving objects
- Depth of field effects through a thin-lens camera model
- Bounding volume hierarchy (BVH) for spatial acceleration
- Anti-aliasing through multi-sampling
- Linear-to-gamma correction for proper color display

## Usage

The application provides several built-in scene examples:

```
USAGE:
    ray_tracer [SUBCOMMAND]

SUBCOMMANDS:
    BouncingSpheres    Renders a scene with dynamically moving spheres
    CheckeredSpheres   Renders a scene with checkered texture spheres
    SimpleLight        Renders a simple light emission demo
    Quads              Renders a scene with various quadrilateral surfaces
    CornellBox         Renders the classic Cornell Box scene
    PerlinSpheres      Renders a scene with Perlin noise textured spheres
```

To render a scene, run:

```bash
cargo run --release -- CornellBox > cornell_box.ppm
```

This will output a PPM image file which can be converted to PNG or other formats using tools like ImageMagick.

## Project Structure

The project is organized into several modules:

- `core/`: Core components including vectors, rays, materials, and the camera
- `shape/`: Geometric primitives like spheres and quadrilaterals
- `volume/`: Spatial acceleration structures (AABB and BVH)
- `materials/`: Material definitions (Lambertian, Metal, Dielectric, DiffuseLight)
- `textures/`: Texture definitions (SolidColor, Checkerboard, PerlinNoise)

## Implementation Details

### Camera

The camera implementation supports:
- Configurable position and orientation (look_from, look_at)
- Field of view control
- Aspect ratio adjustment
- Depth of field with configurable focus distance and aperture

### Materials

The project implements several material types:
- **Lambertian**: Diffuse surfaces with albedo control
- **Metal**: Reflective surfaces with configurable fuzziness
- **Dielectric**: Transparent materials like glass with refraction
- **DiffuseLight**: Light-emitting surfaces for illumination

### Scene Construction

Scenes are constructed by creating geometric primitives with associated materials and adding them to a world object. The world is then wrapped in a BVH for efficient ray-scene intersection.

### Rendering

The renderer uses Monte Carlo path tracing with multiple samples per pixel to produce realistic global illumination. The ray color function recursively traces rays through the scene, accumulating light contributions up to a maximum recursion depth.

## Performance

The ray tracer uses several optimizations:
- Bounding Volume Hierarchy (BVH) for faster ray-object intersection tests
- Multi-sample anti-aliasing
- Efficient vector operations

For best performance, compile and run in release mode.

## Examples

Check the `main.rs` file for several example scenes that demonstrate different features of the ray tracer.

## License

This project is open source and available under the MIT License.

## Acknowledgments

- Peter Shirley for his excellent "Ray Tracing in One Weekend" book series
- The Rust community for providing a fast and safe systems programming language
