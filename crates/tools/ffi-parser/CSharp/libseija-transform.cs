public struct TransformMatrix {
    public Vec3  scale;
    public Quat  rotation;
    public Vec3  position;
};

public struct Transform {
    public TransformMatrix  local;
    public TransformMatrix  global;
};

class libseija_transform {

    [DllImport("lib_seija.dll")]
    public static extern void tranrform_add_module();

    [DllImport("lib_seija.dll")]
    public static extern void transform_world_entity_add();

    [DllImport("lib_seija.dll")]
    public static extern IntPtr transform_world_entity_get();


}