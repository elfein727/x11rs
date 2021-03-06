#[repr(u8)]
pub enum OpCode {
   CreateWindow = 1,
   ChangeWindowAttributes = 2,
   GetWindowAttributes = 3,
   DestroyWindow = 4,
   DestrroySubwindows = 5,
   ChangeSaveSet = 6,
   ReparentWindow = 7,
   MapWindow = 8,
   MapSubwindows = 9,
   UnmapWindow = 10,
   UnmapSubwindows = 11,
   ConfigureWindow = 12,
   CirculateWindow = 13,
   GetGeometry = 14,
   QueryTree = 15,
   InternAtom = 16,
   GetAtomName = 17,
   ChangeProperty = 18,
   DeleteProperty = 19,
   GetProperty = 20,
   ListProperties = 21,
   SetSelectionOwner = 22,
   GetSelectionOwner = 23,
   ConvertSelection = 24,
   SendEvent = 25,
   GrabPointer = 26,
   UngrabPointer = 27,
   GrabButton = 28,
   UngrabButton = 29,
   ChangeActivePointerGrab = 30,
   GrabKeyboard = 31,
   UngrabKeyboard = 32,
   GrabKey = 33,
   UngrabKey = 34,
   AllowEvents = 35,
   GrabServer = 36,
   UngrabServer = 37,
   QueryPointer = 38,
   GetMotionEvents = 39,
   TranslateCoordinates = 40,
   WarpPointer = 41,
   SetInputFocus = 42,
   GetInputFocus = 43,
   QueryKeymap = 44,
   OpenFont = 45,
   CloseFont = 46,
   QueryFont = 47,
   QueryTextExtents = 48,
   ListFonts = 49,
   ListFontsWithInfo = 50,
   CreatePixmap = 53,
   CreateGC = 55,
   ChangeGC = 56,
   CopyGC = 57,
   SetDashes = 58,
   SetClipRectangles = 59,
   FreeGC = 60,
   ClearArea = 61,
   CopyArea = 62,
   CopyPlane = 63,
   PolyPoint = 64,
   PolyLine = 65,
   PolySegment = 66,
   PolyRectangle = 67,
   PolyArc = 68,
   FillPoly = 69,
   PolyFillRectangle = 70,
   PolyFillArc = 71,
   PutImage = 72,
   GetImage = 73,
   PolyText8 = 74,
   PolyText16 = 75,
   ImageText8 = 76,
   ImageText16 = 77,
   CreateColormap = 78,
   FreeColormap = 79,
   CopyColormapAndFree = 80,
   InstallColormap = 81,
   UninstallColormap = 82,
   ListInstalledColormaps = 83,
   AllocColor = 84,
   AllocNamedColor = 85,
   AllocColorCells = 86,
   AllocColorPlanes = 87,
   FreeColors = 88,
   StoreColors = 89,
   StoreNamedColor = 90,
   QueryColors = 91,
   LookupColor = 92,
   CreateCursor = 93,
   CreateGlyphCursor = 94,
   FreeCursor = 95,
   RecolorCursor = 96,
   QueryBestSize = 97,
   QueryExtension = 98,
   ListExtensions = 99,
   ChangeKeyboardMapping = 100,
   GetKeyboardMapping = 101,
   ChangeKeyboardControl = 102,
   GetKeyboardControl = 103,
   Bell = 104,
   ChangePointerControl = 105,
   GetPointerControl = 106,
   SetScreenSaver = 107,
   GetScreenSaver = 108,
   ChangeHosts = 109,
   ListHosts = 110,
   SetAccessControl = 111,
   KillClient = 113,
   RotateProperties = 114,
   ForceScreenSaver = 115,
   SetPointerMapping = 116,
   GetPointerMapping = 117,
   SetModifierMapping = 118,
   GetModifierMapping = 119,
   NoOperation = 127,
}

impl From<OpCode> for u8 {
   fn from(v: OpCode) -> u8 {
      match v {
         OpCode::CreateWindow => 1,
         OpCode::ChangeWindowAttributes => 2,
         OpCode::GetWindowAttributes => 3,
         OpCode::DestroyWindow => 4,
         OpCode::DestrroySubwindows => 5,
         OpCode::ChangeSaveSet => 6,
         OpCode::ReparentWindow => 7,
         OpCode::MapWindow => 8,
         OpCode::MapSubwindows => 9,
         OpCode::UnmapWindow => 10,
         OpCode::UnmapSubwindows => 11,
         OpCode::ConfigureWindow => 12,
         OpCode::CirculateWindow => 13,
         OpCode::GetGeometry => 14,
         OpCode::QueryTree => 15,
         OpCode::InternAtom => 16,
         OpCode::GetAtomName => 17,
         OpCode::ChangeProperty => 18,
         OpCode::DeleteProperty => 19,
         OpCode::GetProperty => 20,
         OpCode::ListProperties => 21,
         OpCode::SetSelectionOwner => 22,
         OpCode::GetSelectionOwner => 23,
         OpCode::ConvertSelection => 24,
         OpCode::SendEvent => 25,
         OpCode::GrabPointer => 26,
         OpCode::UngrabPointer => 27,
         OpCode::GrabButton => 28,
         OpCode::UngrabButton => 29,
         OpCode::ChangeActivePointerGrab => 30,
         OpCode::GrabKeyboard => 31,
         OpCode::UngrabKeyboard => 32,
         OpCode::GrabKey => 33,
         OpCode::UngrabKey => 34,
         OpCode::AllowEvents => 35,
         OpCode::GrabServer => 36,
         OpCode::UngrabServer => 37,
         OpCode::QueryPointer => 38,
         OpCode::GetMotionEvents => 39,
         OpCode::TranslateCoordinates => 40,
         OpCode::WarpPointer => 41,
         OpCode::SetInputFocus => 42,
         OpCode::GetInputFocus => 43,
         OpCode::QueryKeymap => 44,
         OpCode::OpenFont => 45,
         OpCode::CloseFont => 46,
         OpCode::QueryFont => 47,
         OpCode::QueryTextExtents => 48,
         OpCode::ListFonts => 49,
         OpCode::ListFontsWithInfo => 50,
         OpCode::CreatePixmap => 53,
         OpCode::CreateGC => 55,
         OpCode::ChangeGC => 56,
         OpCode::CopyGC => 57,
         OpCode::SetDashes => 58,
         OpCode::SetClipRectangles => 59,
         OpCode::FreeGC => 60,
         OpCode::ClearArea => 61,
         OpCode::CopyArea => 62,
         OpCode::CopyPlane => 63,
         OpCode::PolyPoint => 64,
         OpCode::PolyLine => 65,
         OpCode::PolySegment => 66,
         OpCode::PolyRectangle => 67,
         OpCode::PolyArc => 68,
         OpCode::FillPoly => 69,
         OpCode::PolyFillRectangle => 70,
         OpCode::PolyFillArc => 71,
         OpCode::PutImage => 72,
         OpCode::GetImage => 73,
         OpCode::PolyText8 => 74,
         OpCode::PolyText16 => 75,
         OpCode::ImageText8 => 76,
         OpCode::ImageText16 => 77,
         OpCode::CreateColormap => 78,
         OpCode::FreeColormap => 79,
         OpCode::CopyColormapAndFree => 80,
         OpCode::InstallColormap => 81,
         OpCode::UninstallColormap => 82,
         OpCode::ListInstalledColormaps => 83,
         OpCode::AllocColor => 84,
         OpCode::AllocNamedColor => 85,
         OpCode::AllocColorCells => 86,
         OpCode::AllocColorPlanes => 87,
         OpCode::FreeColors => 88,
         OpCode::StoreColors => 89,
         OpCode::StoreNamedColor => 90,
         OpCode::QueryColors => 91,
         OpCode::LookupColor => 92,
         OpCode::CreateCursor => 93,
         OpCode::CreateGlyphCursor => 94,
         OpCode::FreeCursor => 95,
         OpCode::RecolorCursor => 96,
         OpCode::QueryBestSize => 97,
         OpCode::QueryExtension => 98,
         OpCode::ListExtensions => 99,
         OpCode::ChangeKeyboardMapping => 100,
         OpCode::GetKeyboardMapping => 101,
         OpCode::ChangeKeyboardControl => 102,
         OpCode::GetKeyboardControl => 103,
         OpCode::Bell => 104,
         OpCode::ChangePointerControl => 105,
         OpCode::GetPointerControl => 106,
         OpCode::SetScreenSaver => 107,
         OpCode::GetScreenSaver => 108,
         OpCode::ChangeHosts => 109,
         OpCode::ListHosts => 110,
         OpCode::SetAccessControl => 111,
         OpCode::KillClient => 113,
         OpCode::RotateProperties => 114,
         OpCode::ForceScreenSaver => 115,
         OpCode::SetPointerMapping => 116,
         OpCode::GetPointerMapping => 117,
         OpCode::SetModifierMapping => 118,
         OpCode::GetModifierMapping => 119,
         OpCode::NoOperation => 127,
      }
   }
}

impl From<u8> for OpCode {
   fn from(v: u8) -> OpCode {
      match v {
         1 => Self::CreateWindow,
         2 => Self::ChangeWindowAttributes,
         3 => Self::GetWindowAttributes,
         4 => Self::DestroyWindow,
         5 => Self::DestrroySubwindows,
         6 => Self::ChangeSaveSet,
         7 => Self::ReparentWindow,
         8 => Self::MapWindow,
         9 => Self::MapSubwindows,
         10 => Self::UnmapWindow,
         11 => Self::UnmapSubwindows,
         12 => Self::ConfigureWindow,
         13 => Self::CirculateWindow,
         14 => Self::GetGeometry,
         15 => Self::QueryTree,
         16 => Self::InternAtom,
         17 => Self::GetAtomName,
         18 => Self::ChangeProperty,
         19 => Self::DeleteProperty,
         20 => Self::GetProperty,
         21 => Self::ListProperties,
         22 => Self::SetSelectionOwner,
         23 => Self::GetSelectionOwner,
         24 => Self::ConvertSelection,
         25 => Self::SendEvent,
         26 => Self::GrabPointer,
         27 => Self::UngrabPointer,
         28 => Self::GrabButton,
         29 => Self::UngrabButton,
         30 => Self::ChangeActivePointerGrab,
         31 => Self::GrabKeyboard,
         32 => Self::UngrabKeyboard,
         33 => Self::GrabKey,
         34 => Self::UngrabKey,
         35 => Self::AllowEvents,
         36 => Self::GrabServer,
         37 => Self::UngrabServer,
         38 => Self::QueryPointer,
         39 => Self::GetMotionEvents,
         40 => Self::TranslateCoordinates,
         41 => Self::WarpPointer,
         42 => Self::SetInputFocus,
         43 => Self::GetInputFocus,
         44 => Self::QueryKeymap,
         45 => Self::OpenFont,
         46 => Self::CloseFont,
         47 => Self::QueryFont,
         48 => Self::QueryTextExtents,
         49 => Self::ListFonts,
         50 => Self::ListFontsWithInfo,
         53 => Self::CreatePixmap,
         55 => Self::CreateGC,
         56 => Self::ChangeGC,
         57 => Self::CopyGC,
         58 => Self::SetDashes,
         59 => Self::SetClipRectangles,
         60 => Self::FreeGC,
         61 => Self::ClearArea,
         62 => Self::CopyArea,
         63 => Self::CopyPlane,
         64 => Self::PolyPoint,
         65 => Self::PolyLine,
         66 => Self::PolySegment,
         67 => Self::PolyRectangle,
         68 => Self::PolyArc,
         69 => Self::FillPoly,
         70 => Self::PolyFillRectangle,
         71 => Self::PolyFillArc,
         72 => Self::PutImage,
         73 => Self::GetImage,
         74 => Self::PolyText8,
         75 => Self::PolyText16,
         76 => Self::ImageText8,
         77 => Self::ImageText16,
         78 => Self::CreateColormap,
         79 => Self::FreeColormap,
         80 => Self::CopyColormapAndFree,
         81 => Self::InstallColormap,
         82 => Self::UninstallColormap,
         83 => Self::ListInstalledColormaps,
         84 => Self::AllocColor,
         85 => Self::AllocNamedColor,
         86 => Self::AllocColorCells,
         87 => Self::AllocColorPlanes,
         88 => Self::FreeColors,
         89 => Self::StoreColors,
         90 => Self::StoreNamedColor,
         91 => Self::QueryColors,
         92 => Self::LookupColor,
         93 => Self::CreateCursor,
         94 => Self::CreateGlyphCursor,
         95 => Self::FreeCursor,
         96 => Self::RecolorCursor,
         97 => Self::QueryBestSize,
         98 => Self::QueryExtension,
         99 => Self::ListExtensions,
         100 => Self::ChangeKeyboardMapping,
         101 => Self::GetKeyboardMapping,
         102 => Self::ChangeKeyboardControl,
         103 => Self::GetKeyboardControl,
         104 => Self::Bell,
         105 => Self::ChangePointerControl,
         106 => Self::GetPointerControl,
         107 => Self::SetScreenSaver,
         108 => Self::GetScreenSaver,
         109 => Self::ChangeHosts,
         110 => Self::ListHosts,
         111 => Self::SetAccessControl,
         113 => Self::KillClient,
         114 => Self::RotateProperties,
         115 => Self::ForceScreenSaver,
         116 => Self::SetPointerMapping,
         117 => Self::GetPointerMapping,
         118 => Self::SetModifierMapping,
         119 => Self::GetModifierMapping,
         127 => Self::NoOperation,
         _ => Self::NoOperation, // Ideally an extension ,probably
      }
   }
}