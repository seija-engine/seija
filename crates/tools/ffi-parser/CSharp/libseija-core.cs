using System.Runtime.InteropServices;
public static class libseija_core {

    public delegate void WorldFN(IntPtr world);
    [DllImport("lib_seija")]
    public static extern void app_set_on_start(IntPtr app_ptr,WorldFN start_fn);

    [DllImport("lib_seija")]
    public static extern void app_set_on_update(IntPtr app_ptr,WorldFN update_fn);

    [DllImport("lib_seija")]
    public static extern void core_add_module(IntPtr app_ptr);

    [DllImport("lib_seija")]
    public static extern float core_time_get_delta_seconds(IntPtr time_ptr);

    [DllImport("lib_seija")]
    public static extern ulong core_time_get_frame(IntPtr time_ptr);

    [DllImport("lib_seija")]
    public static extern IntPtr core_world_get_time(IntPtr world_ptr);


}