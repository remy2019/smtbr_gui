slint::include_modules!();

fn main() {
    use slint::Model;

    let main_window = MainWindow::new().unwrap();

    main_window.run().unwrap();
}
