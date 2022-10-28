using System.Runtime.InteropServices;
public static class libseija_winit {

    [DllImport("lib_seija.dll")]
    public static extern void winit_add_module(IntPtr app_ptr,IntPtr config_ptr);

    [DllImport("lib_seija.dll")]
    public static extern IntPtr winit_new_windowconfig();

    [DllImport("lib_seija.dll")]
    public static extern void winit_windowconfig_set_title(IntPtr config_ptr,[MarshalAs(UnmanagedType.LPUTF8Str)] string title);


}