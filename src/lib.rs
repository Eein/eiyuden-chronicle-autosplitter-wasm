#![feature(type_alias_impl_trait, const_async_blocks)]

use asr::{ timer::{self, TimerState}, future::next_tick, settings::Gui, Process, PointerSize, game_engine::unity::il2cpp};
use asr::game_engine::unity::il2cpp::Version;


asr::async_main!(nightly);

#[derive(Gui)]
struct Settings {
    /// My Setting
    #[default = true]
    my_setting: bool,
    // TODO: Change these settings.
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

        asr::print_message(&instance.to_string());
        let ui_manager_offset = game_manager_class.wait_get_field_offset(&process, &module, "<UIManager>k__BackingField").await;
        
        asr::print_message(&format!("{:x}", &ui_manager_offset));

        // let (main_module_base, _main_module_size) = process
        //     .wait_module_range("EiyudenChronicle.exe")
        // .await;
        //
       
        process
            .until_closes(async {
                // TODO: Load some initial information from the process.
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

                    if let Some(true) = loading_value {
                        match timer::state() {
                            TimerState::NotRunning => {
                                timer::start()
                            },
                            TimerState::Ended=> {
                                timer::start()
                            },
                            TimerState::Paused => {
                                timer::split()
                            },
                            _ => ()
                        }
                        timer::resume_game_time()
                    } else {
                        timer::pause_game_time()
                    }

                    // TODO: Do something on every tick.
                    next_tick().await;
                }
            })
            .await;
    }
}
