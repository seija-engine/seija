using System.Runtime.InteropServices;
public static class libseija_core {

    [DllImport("lib_seija.dll")]
    public static extern void core_add_module(IntPtr app_ptr);

    [DllImport("lib_seija.dll")]
    public static extern float core_time_get_delta_seconds(IntPtr time_ptr);

    [DllImport("lib_seija.dll")]
    public static extern ulong core_time_get_frame(IntPtr time_ptr);

    [DllImport("lib_seija.dll")]
    public static extern IntPtr core_world_get_time(IntPtr world_ptr);


}