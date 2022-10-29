

typedef void (*WorldFN)(World *world);

void app_set_on_start(App *app_ptr, WorldFN start_fn);

void app_set_on_update(App *app_ptr, WorldFN update_fn);

void core_add_module(uint8_t *app_ptr);

float core_time_get_delta_seconds(const uint8_t *time_ptr);

uint64_t core_time_get_frame(const uint8_t *time_ptr);

const uint8_t *core_world_get_time(uint8_t *world_ptr);
