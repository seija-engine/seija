using System.Runtime.InteropServices;
public static class libseija_app {

    [DllImport("lib_seija")]
    public static extern IntPtr app_new();

    [DllImport("lib_seija")]
    public static extern void app_run(IntPtr app_ptr);

    [DllImport("lib_seija")]
    public static extern void app_set_fps(IntPtr app_ptr,uint fps);

    [DllImport("lib_seija")]
    public static extern void app_start(IntPtr app_ptr);


}