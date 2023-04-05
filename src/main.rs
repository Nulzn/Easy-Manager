use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Orientation};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {

    let button_increase = Button::builder()
        .label("Refresh")
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    button_increase.connect_clicked(|button_increase| {
        button_increase.set_label("Woo!");
    });

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);

    let window = ApplicationWindow::builder()
    .application(app)
    .title("Easy Manager")
    .child(&gtk_box)
    .build();

    window.present();
}
