use cfg_if::cfg_if;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
pub static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Conditional features setup
cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        pub fn init_panic() {
            use console_error_panic_hook;
            console_error_panic_hook::set_once();
        }
    } else {
        pub fn init_panic() {}
    }
}
cfg_if! {
    if #[cfg(feature = "console_log")] {
        pub fn init_log() {
            use log::Level;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        pub fn init_log() {}
    }
}
