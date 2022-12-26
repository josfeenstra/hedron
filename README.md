# Hedron
An all-in-one 3D modelling crate, written in Rust!

## What it contains (or will contain)
- A full set of **geometry** models:
  - planar: `Lines`, `Circles`, `Triangles`, ...
  - curves: `Polylines`, `Bezier`, `Spline`, ...
  - surfaces: `BezierSrf` (TODO), ...
  - solids: `Mesh`, `Polyhedron` (WIP), `Solid`, ... 

- **operators** on those models: 
  - Point / Vector tools (Closest point) (TODO)
  - Transformations between the various models  
  - Modelling operations (`loft`, `inset`, `extrude`, `split`, `subdivide`) (TODO) 
  - Interpolation / Extrapolation
  - Intersections (intersect mesh with line) (TODO)
  - Boolean operators (join two solids) (TODO)

- Various extra **features**:
  - Direct [bevy](https://bevyengine.org/) integration: `--features bevy` 
  - Obj / Mtl exporting 
  - Svg exporting (TODO)
  - Various mathematical tools to support the operations mentioned above


## Use cases
- **Basic Modelling**: Hedron could be used to develop a 3D modelling tool, a very basic `blender` clone.

- **Procedural Geometry / 'Parametric Design'**: Hedron is intended for parametric modelling, akin to what can be done with Rhino & Grasshopper. 

- **Web Geometry Processing**: The crate can be complied to WebAssembly, allowing these operations to be used on the web.

## What it is not 
- **No GIS support** : The crate offers no tooling to load and process large geographical datasets. 
- **Not BIM ready** : Currently, the crate does not support IFC models.
- **Not CAD ready** : The crate does not support common CAD file types such as STL. It also does not contain constructive solid geometry (CSG) models.


# Design 
Hedron is designed to strike a balance between usability and expressiveness. It is not as expressive as something like CGAL, but it is more simple and straight- forward to use. 


# Stage: Pre Alpha
Hedron is currently in an very early stage. 
I don't recommend you use this library quite yet, but hopefully some of the models and operations presented can help you nonetheless!
