use std::process::exit;

slint::include_modules!();

fn main() {
    use slint::Model;

    let main_window = MainWindow::new().unwrap();
    main_window.on_close(|| exit(0));

    main_window.run().unwrap();
}
