fn main() {
    embed_resource::compile("app.rc", embed_resource::NONE)
        .manifest_optional()
        .expect("failed to embed application icon resource");
}
