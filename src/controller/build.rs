fn main() {
    slint_build::compile("ui/main.slint").expect("failed to compile ui/main.slint");
    embed_resource::compile("app.rc", embed_resource::NONE)
        .manifest_optional()
        .expect("failed to embed application icon resource");
}
