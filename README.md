# Hedron
An all-in-one 3D modelling & procedural generation crate

## 🚧 Disclaimer 🚧
Hedron is currently in an very early stage. 
I don't recommend you use this crate quite yet, as it is not at all stable or complete, 
but hopefully some of code provided can help you nonetheless!

## Use cases:
- **Basic Modelling**: Hedron could be used to develop a 3D modelling tool, a very basic `blender` clone.

- **Procedural Geometry / 'Parametric Design'**: Hedron is intended for parametric modelling, akin to what can be done with Rhino & Grasshopper. 

- **Web Geometry Processing**: The crate can be complied to WebAssembly, allowing these operations to be used on the web.

## What it contains / will contain:
Hedron strifes for feature parody with the parametric design tool called Grasshopper. 
In addition, any tools aiding procedural modelling like perlin noise generation, or Wave Function Collapse, will also be added.
The crate intends to be your one-stop shop with regards to procedural modelling!

It includes:

- **A variety of geometry models**:
  - planar: `Line`, `Circle`, `Triangle`, `Polygon` ...
  - curves: `Polyline`, `Bezier`, `Spline`, ...
  - surfaces: `BezierSrf`, `SplineSrf` (TODO), ...
  - solids: `Mesh`, `Polyhedron`, `Brep` (TODO), ... 

- **Operators on those models**: 
  - Point / Vector tools (Closest point, ...) 
  - Transformations between the various models (triangulate  brep, ...)  
  - Modelling operations (`loft`, `inset`, `extrude`, `split`, `subdivide`) 
  - Intersections (intersect mesh with plane)
  - Boolean operators (join two solids) (TODO)

- **Importers & Exporters**:
  - Direct [bevy](https://bevyengine.org/) integration: `--features bevy` 
  - Obj + Mtl exporting 
  - Svg, Stl, Gltf importing & exporting (TODO): `--features svg stl gltf`
  - Various mathematical tools to support the operations mentioned above.

- **Various extra procedural tools**:
  - Marching cubes
  - Quadrilateral mesh deformation 
  - Perlin Noise generation
  - Wave Function Collapse (TODO)


# Used in:
> ![Some cute graphics](./LOGO.PNG)
> [My upcoming game](https://twitter.com/i_am_feenster/status/1622708645606703104)


# Design 
Hedron is designed to strike a balance between usability and expressiveness. It sacrifices A CGAL-level of expressivenes, in favor of a more simple and predictable API. 
 
