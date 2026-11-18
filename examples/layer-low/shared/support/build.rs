fn main() {
    #[cfg(feature = "opengl")]
    {
        use std::{env, fs::File, path::Path};

        use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};

        let dest = env::var("OUT_DIR").unwrap();
        let mut file = File::create(Path::new(&dest).join("gl_bindings.rs")).unwrap();

        Registry::new(Api::Gl, (4, 5), Profile::Compatibility, Fallbacks::All, [])
            .write_bindings(GlobalGenerator, &mut file)
            .unwrap();
    }
}
