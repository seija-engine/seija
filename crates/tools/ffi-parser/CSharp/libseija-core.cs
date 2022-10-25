public enum WindowMode {
    Windowed,
    BorderlessFullscreen,
    Fullscreen,
 };

public struct WindowConfig {
    public float  width;
    public float  height;
    public WindowMode  mode;
    public bool  vsync;
    public string  title;
};

class libseija_core {

    [DllImport("lib_seija.dll")]
    public static extern void core_add_module();

    [DllImport("lib_seija.dll")]
    public static extern IntPtr core_new_windowconfig();

    [DllImport("lib_seija.dll")]
    public static extern float core_time_get_delta_seconds();

    [DllImport("lib_seija.dll")]
    public static extern ulong core_time_get_frame();

    [DllImport("lib_seija.dll")]
    public static extern void core_windowconfig_set_title();

    [DllImport("lib_seija.dll")]
    public static extern IntPtr core_world_get_time();


}