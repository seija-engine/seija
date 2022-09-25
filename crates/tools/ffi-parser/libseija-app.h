

typedef struct App App;

uint8_t *app_new(void);

void app_run(struct App *app_ptr);

void app_set_fps(struct App *app_ptr, uint32_t fps);

void app_start(struct App *app_ptr);
