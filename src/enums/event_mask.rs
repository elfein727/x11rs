#[repr(u32)]
#[derive(Copy, Clone)]
pub enum EventMask {
   None = 0,
   KeyPress = 0x00000001,
   KeyRelease = 0x00000002,
   ButtonPress = 0x00000004,
   ButtonRelease = 0x00000008,
   EnterWindow = 0x00000010,
   LeaveWindow = 0x00000020,
   PointerMotion = 0x00000040,
   PointerMotionHint = 0x00000080,
   Button1Motion = 0x00000100,
   Button2Motion = 0x00000200,
   Button3Motion = 0x00000400,
   Button4Motion = 0x00000800,
   Button5Motion = 0x00001000,
   ButtonMotion = 0x00002000,
   KeymapState = 0x00004000,
   Exposure = 0x00008000,
   VisibilityChange = 0x00010000,
   StructureNotify = 0x00020000,
   ResizeRedirect = 0x00040000,
   SubstructureNotify = 0x00080000,
   SubstructureRedirect = 0x00100000,
   FocusChange = 0x00200000,
   PropertyChange = 0x00400000,
   ColormapChange = 0x00800000,
   OwnerGrabButton = 0x01000000,
   UnusedMask = 0xFE000000,
}

impl Default for EventMask {
   fn default() -> Self {
      Self::None
   }
}