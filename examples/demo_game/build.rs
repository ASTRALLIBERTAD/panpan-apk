fn main() {
    // Link against Android EGL and OpenGL ES libraries when building for Android
    #[cfg(target_os = "android")]
    {
        println!("cargo:rustc-link-lib=EGL");
        println!("cargo:rustc-link-lib=GLESv2");
        println!("cargo:rustc-link-lib=log");
    }
}
