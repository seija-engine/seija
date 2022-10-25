class libseija_app {

    [DllImport("lib_seija.dll")]
    public static extern IntPtr app_new();

    [DllImport("lib_seija.dll")]
    public static extern void app_run();

    [DllImport("lib_seija.dll")]
    public static extern void app_set_fps();

    [DllImport("lib_seija.dll")]
    public static extern void app_start();


}