

typedef struct Transform Transform;

typedef struct FFIV3 {
  float x;
  float y;
  float z;
} FFIV3;

void tranrform_add_module(App *app_ptr);

struct Transform *transform_new(struct FFIV3 pos);

struct Transform *transform_world_entity_get(World *world, uint32_t eid);

