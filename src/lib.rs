#![feature(type_alias_impl_trait, const_async_blocks)]
#![no_std]

use asr::{ timer::{self, TimerState}, future::next_tick, settings::Gui, Process, PointerSize, game_engine::unity::il2cpp};
use asr::game_engine::unity::il2cpp::Version;

asr::panic_handler!();
asr::async_main!(nightly);

#[derive(Gui)]
struct Settings {
    /// Enable Load Remover
    #[default = true]
    load_remover: bool,
}

async fn main() {
    // TODO: Set up some general state and settings.
    let mut settings = Settings::register();

    asr::print_message("Hello, World!");

    loop {
        let process = Process::wait_attach("EiyudenChronicle.exe").await;
        let module = il2cpp::Module::wait_attach(&process, Version::V2020).await;
        let image = module.wait_get_default_image(&process).await;
        let game_manager_class = image.wait_get_class(&process, &module, "GameManager").await;
        let game_manager_parent = game_manager_class.wait_get_parent(&process, &module).await;
        let instance = game_manager_parent.wait_get_static_instance(&process, &module, "instance").await;
        let ui_manager_offset = game_manager_class.wait_get_field_offset(&process, &module, "<UIManager>k__BackingField").await;
       
        process
            .until_closes(async {
                loop {
                    settings.update();
                    let loading_value = match process.read_pointer_path::<bool>(
                        instance,
                        PointerSize::Bit64,
                        &[ui_manager_offset as u64, 0x88, 0x61],
                    ) {
                        Ok(val) => Some(val),
                        Err(_e) => Some(false),
                    };

                    if settings.load_remover {
                        match timer::state() {
                            TimerState::Running => {
                                if let Some(true) = loading_value {
                                    timer::pause_game_time()
                                } else {
                                    timer::resume_game_time()
                                }
                            },
                            _ => ()
                        }
                    }
                    next_tick().await;
                }
            })
            .await;
    }
}
