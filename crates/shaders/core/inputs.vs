layout(location = LOCATION_POSITION) in vec4 mesh_position;

#if defined(HAS_ATTRIBUTE_TANGENTS)
layout(location = LOCATION_TANGENTS) in vec4 mesh_tangents;
#endif

#if defined(HAS_ATTRIBUTE_COLOR)
layout(location = LOCATION_COLOR) in vec4 mesh_color;
#endif

#if defined(HAS_ATTRIBUTE_UV0)
layout(location = LOCATION_UV0) in vec2 mesh_uv0;
#endif

#if defined(HAS_ATTRIBUTE_UV1)
layout(location = LOCATION_UV1) in vec2 mesh_uv1;
#endif