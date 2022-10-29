using System.Runtime.InteropServices;
public struct TransformMatrix {
    public Vec3  scale;
    public Quat  rotation;
    public Vec3  position;
};

public struct Transform {
    public TransformMatrix  local;
    public TransformMatrix  global;
};

public static class libseija_transform {

    [DllImport("lib_seija")]
    public static extern void tranrform_add_module(IntPtr app_ptr);

    [DllImport("lib_seija")]
    public static extern void transform_world_entity_add(IntPtr world,uint eid,IntPtr t);

    [DllImport("lib_seija")]
    public static extern IntPtr transform_world_entity_get(IntPtr world,uint eid);


}