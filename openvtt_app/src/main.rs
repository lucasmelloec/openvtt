fn main() {
    #[cfg(target_arch = "wasm32")]
    openvtt_app::web::setup_log();

    openvtt_app::run()
}
