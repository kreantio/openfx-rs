#![allow(warnings)]

/// This should only be used directly to access constants from the outside.
pub mod inner {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

macro_rules! export_functions {
    ($($name:ident),*) => {
        $(pub use inner::$name;)*

        pub fn missing_functions() -> Vec<&'static str> {
            let mut missing = vec![];
            $(
                if !$name::is_loaded() {
                    missing.push(stringify!($name));
                }
            )*
            missing
        }
    };
}

export_functions!(
    PushAttrib,
    PopAttrib,
    Enable,
    Disable,
    Begin,
    End,
    BindTexture,
    TexParameteri,
    TexEnvf,
    TexCoord2f,
    Vertex2f,
    Color3f,
    Color4f
);
