mod base;
mod example;

pub use self::{base::*, example::*};

use godot::{classes::Engine, prelude::*};

struct Extension;

const SINGLETON_NAME: &str = "GodotRust";

#[gdextension]
unsafe impl ExtensionLibrary for Extension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            ok!(init());

            info!("init");

            let mut engine = Engine::singleton();

            engine.register_singleton(SINGLETON_NAME, &Singleton::new_alloc());
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            info!("deinit");

            let mut engine = Engine::singleton();

            let singleton = ok!({
                engine
                    .get_singleton(SINGLETON_NAME)
                    .ok_or(anyhow!("unregister: {SINGLETON_NAME}"))
            });
            engine.unregister_singleton(SINGLETON_NAME);
            singleton.free();
        }
    }
}
