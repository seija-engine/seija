

typedef struct TransformMatrix {
  Vec3 scale;
  Quat rotation;
  Vec3 position;
} TransformMatrix;

typedef struct Transform {
  struct TransformMatrix local;
  struct TransformMatrix global;
} Transform;

void tranrform_add_module(App *app_ptr);

void transform_world_entity_add(World *world, uint32_t eid, const struct Transform *t);

struct Transform *transform_world_entity_get(World *world, uint32_t eid);
