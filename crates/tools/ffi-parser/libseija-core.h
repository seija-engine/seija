

enum WindowMode {
  Windowed,
  BorderlessFullscreen,
  Fullscreen,
};
typedef uint32_t WindowMode;

typedef struct String String;

typedef struct WindowConfig {
  float width;
  float height;
  WindowMode mode;
  bool vsync;
  struct String title;
} WindowConfig;

void core_add_module(uint8_t *app_ptr);

struct WindowConfig *core_new_windowconfig(void);

float core_time_get_delta_seconds(const uint8_t *time_ptr);

uint64_t core_time_get_frame(const uint8_t *time_ptr);

void core_windowconfig_set_title(struct WindowConfig *config_ptr, const int8_t *title);

const uint8_t *core_world_get_time(uint8_t *world_ptr);
