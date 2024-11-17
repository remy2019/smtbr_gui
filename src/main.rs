pub mod serialize;
pub mod types;

use std::process::exit;
use std::time;

slint::include_modules!();

fn main() {
    use slint::Model;
    let exe = std::env::current_exe().unwrap();
    let dir = exe.parent().expect("Executable must be in some directory");
    let test_yml = dir.clone().join("test.yml");
    let black_yml = dir.join("blacklist.yml");

    let main_window = MainWindow::new().unwrap();
    let main_window_weak = main_window.as_weak();

    // check test.yml exists
    let exe = std::env::current_exe().unwrap();
    let dir = exe.parent().expect("Executable must be in some directory");
    let yml = dir.join("test.yml");
    match std::fs::metadata(yml) {
        Ok(_) => main_window.set_page_enum(1),
        Err(_) => main_window.set_page_enum(0),
    }

    // terminate app
    main_window.on_close(|| exit(0));

    // create test.yml
    main_window.on_serialize(move || {
        let main_window = main_window_weak.unwrap();
        slint::Timer::single_shot(time::Duration::from_secs(3), move || {
            match serialize::serialize_problems() {
                Ok(_) => {
                    main_window.set_processing(false);
                    main_window.set_is_yaml(true);
                }
                Err(_) => {
                    main_window.set_processing(false);
                    main_window.set_is_error(true);
                }
            }
        })
    });

    // remove test.yml
    main_window.on_reset(move || {
        _ = std::fs::remove_file(&test_yml);
        _ = std::fs::remove_file(&black_yml);
    });

    main_window.run().unwrap();
}
