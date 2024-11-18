pub mod deserialize;
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
    match std::fs::metadata(test_yml.clone()) {
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
                    // create default blacklist
                    let exe = std::env::current_exe().unwrap();
                    let dir = exe.parent().expect("Executable must be in some directory");
                    let yml = dir.join("test.yml");
                    let data_string = &std::fs::read_to_string(yml).unwrap();
                    let data = deserialize::deserialize(data_string).unwrap();

                    let mut selected_data: Vec<(u32, u32, bool)> = vec![];
                    for problem in data {
                        selected_data.push((problem.chapter, problem.prob_num_rel, false));
                    }
                    let black_yml = dir.join("blacklist.yml");

                    let f = std::fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(black_yml)
                        .expect("Couldn't open file");
                    serde_yaml::to_writer(f, &selected_data).unwrap();

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
