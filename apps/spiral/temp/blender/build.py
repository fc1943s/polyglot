import os
import bpy

# 1) Clear default scene
bpy.ops.wm.read_factory_settings(use_empty=True)

# 2) Create a cube
bpy.ops.mesh.primitive_cube_add(size=2, location=(0, 0, 0))
cube_obj = bpy.context.active_object

# Add a subdivision surface modifier for extra detail
subsurf_mod = cube_obj.modifiers.new(name="Subsurf", type='SUBSURF')
subsurf_mod.levels = 2
bpy.ops.object.modifier_apply(modifier=subsurf_mod.name)

# Switch to Edit Mode to extrude faces
bpy.ops.object.mode_set(mode='EDIT')
bpy.ops.mesh.select_all(action='SELECT')

# Extrude all faces out a bit
bpy.ops.mesh.extrude_faces_move(
    TRANSFORM_OT_shrink_fatten={"value": 0.2}
)

# Switch back to Object Mode
bpy.ops.object.mode_set(mode='OBJECT')

# 3) Create a new material
material = bpy.data.materials.new(name="SampleMaterial")
material.use_nodes = True
bsdf = material.node_tree.nodes["Principled BSDF"]
# Give it a bright orange color
bsdf.inputs["Base Color"].default_value = (1, 0.3, 0.1, 1)  # RGBA
cube_obj.data.materials.append(material)

# 4) Create a camera
bpy.ops.object.camera_add(location=(6, -6, 5), rotation=(1.1, 0, 0.8))
camera_obj = bpy.context.active_object
bpy.context.scene.camera = camera_obj

# Create a sun light
bpy.ops.object.light_add(type='SUN', location=(10, 10, 10))
light_obj = bpy.context.active_object
light_obj.data.energy = 6
light_obj.data.use_shadow = False

# 5) Set rendering options (Cycles, resolution)
bpy.context.scene.render.engine = 'CYCLES'
bpy.context.scene.render.resolution_x = 1024
bpy.context.scene.render.resolution_y = 1024

# Render and save
bpy.context.scene.render.filepath = os.path.join(".", f"spiral_blender.png")
bpy.ops.render.render(write_still=True)
