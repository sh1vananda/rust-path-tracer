DATA FLOW:

1. Configuration Setup  
   - `RenderConfig::new()` → image size, samples, quality settings

2. Scene Creation  
   - `create_scene()` → HittableList with spheres and materials

3. Camera Setup  
   - `Camera::new()` → position, orientation, depth of field

4. Parallel Pixel Generation  
   - For each pixel (i, j):
     - Generate multiple sample rays with random offsets
     - For each sample ray:
       - `cam.get_ray()` → Ray from camera through pixel
       - `ray_color()` → Recursive light transport
         - `world.hit()` → Find closest intersection
         - `material.scatter()` → New ray direction + attenuation
         - Recursive call with depth-1
       - Accumulate color contribution
     - Average all samples for final pixel color

5. Color Output  
   - Apply gamma correction  
   - Convert to 8-bit RGB  
   - Write PPM format
