## Raytracer

A **Raytracer** implementation in **`Rust`** based on the tutorial: [Ray Tracing in One Weekend.](https://raytracing.github.io/books/RayTracingInOneWeekend.html)  
Also used [snarkyboojum's video tutorial series](https://www.youtube.com/watch?v=_5hD0gxRzzg) for guideance.

### Test generated image

### write image test
![](example_renders/my_image.01.test_write_image.jpg)

### render bacgkground
![](example_renders/my_image.02.render_bg.jpg)

### hitting a sphere
![](example_renders/my_image.03.hitting_sphere.jpg)

### added normals
![](example_renders/my_image.04.added_normal.jpg)

### Hittable trait & multiple objects
![](example_renders/my_image.05.hittable_trait.jpg)

### Aliased image without randomised rays
![](example_renders/my_image.06a.without_random_rays.jpg)

### Antialiasing with randomised rays
![](example_renders/my_image.06b.with_random_rays.jpg)

### A simple diffuse material
![](example_renders/my_image.08.simple_diffuse_material.jpg)

### Diffuse, metal & reflection
- ray per pixel: 128
- *max colour calc recursion: 512*
- **Pentium Silver N5000, 4 Gb => ~ 20.0 secs**

![](example_renders/my_image.09.diff_metal_refl.jpg)

### Added fuzziness to metal
![](example_renders/my_image.09b.fuzziness.jpg)
