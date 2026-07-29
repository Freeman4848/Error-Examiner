fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut resource = winresource::WindowsResource::new();
        resource.set_icon("assets/app-icon.ico");
        resource.set("ProductName", "Error Explainer");
        resource.set("FileDescription", "Local AI error and incident explainer");
        resource.set("OriginalFilename", "Error-Explainer.exe");
        resource
            .compile()
            .expect("Windows resources should compile");
    }
}
