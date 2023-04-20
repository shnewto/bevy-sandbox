# bevy-sandbox
sandbox for working on bevy things out in the open. mostly just examples of specific things I want to remember how to do.

## mirror

A 3D scene where a rotated cube is "reflected in a mirror", i.e. a second camera's view is
projected onto a plane. Code borrowed and revised from bevy examples/3d/3d_scene.rs and
examples/3d/render_to_texture.rs

---

![brown cube on a black background, reflected in a gray mirror](/img/mirror.png)

## gltf-mesh-png-material

A 3D scene displaying a rotated cube built from a gltf named mesh and png material

---

![purple and gray grid textured cube on a black background](/img/gltf-png.png)

## save

Sample bevy app that uses moonshine_save to save object data for an object in the scene.
This example corresponds to the "load" example that loads the data this one saves

---

![emissive green cube on a purple background](/img/save-load.png)

## load

Sample bevy app that uses moonshine_save to load object data for an object in the scene.
This example corresponds to the "save" example that saves the data this one loads

---

![emissive green cube on a purple background](/img/save-load.png)

## license
all code in this repository is dual-licensed under either:
- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. 
