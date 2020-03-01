
mod others {
    #[link(name="SDL")]
    extern {}
}

mod alsa {
    #[link(name="asound")]
    extern {}
}


pub mod internal {
    #![allow(non_camel_case_types)]

    use libc::{c_int, c_uint};
    use std::os::raw::{c_void, c_char};

    pub type SDL_errorcode = c_int;

    pub const SDL_ENOMEM: SDL_errorcode = 0;
    pub const SDL_EFREAD: SDL_errorcode = 1;
    pub const SDL_EFWRITE: SDL_errorcode = 2;
    pub const SDL_EFSEEK: SDL_errorcode = 3;
    pub const SDL_UNSUPPORTED: SDL_errorcode = 4;
    pub const SDL_LASTERROR: SDL_errorcode = 5;

    pub type SDL_InitFlag = u32;

    pub const SDL_INIT_TIMER: SDL_InitFlag = 0x00000001;
    pub const SDL_INIT_AUDIO: SDL_InitFlag = 0x00000010;
    pub const SDL_INIT_VIDEO: SDL_InitFlag = 0x00000020;
    pub const SDL_INIT_CDROM: SDL_InitFlag = 0x00000100;
    pub const SDL_INIT_JOYSTICK: SDL_InitFlag = 0x00000200;
    pub const SDL_INIT_NOPARACHUTE: SDL_InitFlag = 0x00100000;
    pub const SDL_INIT_EVENTTHREAD: SDL_InitFlag = 0x01000000;
    pub const SDL_INIT_EVERYTHING: SDL_InitFlag = 0x0000FFFF;

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum SurfaceFlag {
        SWSurface = 0x00000000,
        HWSurface = 0x00000001,
        AsyncBlit = 0x00000004,
        SrcColorKey = 0x00001000,
        SrcAlpha = 0x00010000,
        RLEAccel = 0x00004000,
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum VideoFlag {
        AnyFormat = 0x10000000,
        HWPalette = 0x20000000,
        DoubleBuf = 0x40000000,
        Fullscreen = 0x80000000usize as isize,
        // 0x80000000 > INT_MAX on i686
        OpenGL = 0x00000002,
        OpenGLBlit = 0x0000000A,
        Resizable = 0x00000010,
        NoFrame = 0x00000020,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_Palette {
        pub ncolors: c_int,
        pub colors: *mut SDL_Color,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub unused: u8,
    }

    #[allow(non_snake_case)]
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_PixelFormat {
        pub palette: *mut SDL_Palette,
        pub BitsPerPixel: u8,
        pub BytesPerPixel: u8,
        pub Rloss: u8,
        pub Gloss: u8,
        pub Bloss: u8,
        pub Aloss: u8,
        pub Rshift: u8,
        pub Gshift: u8,
        pub Bshift: u8,
        pub Ashift: u8,
        pub Rmask: u32,
        pub Gmask: u32,
        pub Bmask: u32,
        pub Amask: u32,
        pub colorkey: u32,
        pub alpha: u8,
    }

    #[repr(C)]
    #[derive(PartialEq, Copy, Clone)]
    pub struct SDL_Rect {
        pub x: i16,
        pub y: i16,
        pub w: u16,
        pub h: u16,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_Surface {
        pub flags: u32,
        pub format: *mut SDL_PixelFormat,
        pub w: c_int,
        pub h: c_int,
        pub pitch: u16,
        pub pixels: *mut c_void,
        pub offset: c_int,
        pub hwdata: *mut c_void,
        pub clip_rect: SDL_Rect,
        pub unused1: u32,
        pub locked: u32,
        pub map: *mut c_void,
        pub format_version: c_uint,
        pub refcount: c_int,
    }

    extern "C" {
        pub fn SDL_ClearError();
        pub fn SDL_Error(code: SDL_errorcode);
        pub fn SDL_SetError(fmt: *const c_char);
        pub fn SDL_GetError() -> *const c_char;
        pub fn SDL_Quit();
        pub fn SDL_QuitSubSystem(flags: SDL_InitFlag);
        pub fn SDL_Init(flags: u32) -> c_int;
        pub fn SDL_InitSubSystem(flags: SDL_InitFlag) -> c_int;
        pub fn SDL_WasInit(flags: SDL_InitFlag) -> SDL_InitFlag;
        pub fn SDL_GetTicks() -> u32;

        pub fn SDL_FreeSurface(surface: *mut SDL_Surface);
        pub fn SDL_SetVideoMode(width: c_int, height: c_int, bpp: c_int, flags: u32)
                                -> *mut SDL_Surface;
    }
}

pub mod sdl {
    use std::iter::Iterator;
    use crate::sdl::internal::{SDL_errorcode, SDL_INIT_VIDEO, SDL_Init, SurfaceFlag, VideoFlag, SDL_Quit, SDL_Surface, SDL_FreeSurface, SDL_SetVideoMode, SDL_GetError};
    use std::os::raw::c_int;
    use std::ffi::CStr;

    pub fn init() -> SDL_errorcode {
        unsafe {
            SDL_Init(SDL_INIT_VIDEO)
        }
    }

    pub fn quit() {
        unsafe {
            SDL_Quit()
        }
    }

    #[derive(PartialEq)]
    pub struct Surface {
        pub raw: *mut SDL_Surface,
        pub owned: bool,
    }

    fn wrap_surface(raw: *mut SDL_Surface, owned: bool) -> Surface {
        Surface { raw: raw, owned: owned }
    }

    impl Drop for Surface {
        fn drop(&mut self) {
            unsafe {
                if self.owned {
                    SDL_FreeSurface(self.raw);
                }
            }
        }
    }

    pub fn get_error() -> String {
        unsafe {
            let cstr = SDL_GetError();
            let slice = CStr::from_ptr(cstr).to_bytes();

            std::str::from_utf8(slice).unwrap().to_string()
        }
    }

    pub fn set_video_mode(w: isize, h: isize, bpp: isize,
                          surface_flags: &[SurfaceFlag],
                          video_flags: &[VideoFlag]) -> Result<Surface, String> {
        let flags = surface_flags.iter().fold(0u32, |flags, &flag| {
            flags | flag as u32
        });
        let flags = video_flags.iter().fold(flags, |flags, &flag| {
            flags | flag as u32
        });

        unsafe {
            let raw = SDL_SetVideoMode(w as c_int, h as c_int,
                                       bpp as c_int, flags);

            if raw.is_null() { Err(get_error()) } else { Ok(wrap_surface(raw, false)) }
        }
    }
}
