use std::{env, fs, path::PathBuf, process::Command, rc::Rc};

use adw::{Application, ApplicationWindow};
use directories_next as dirs;
use gtk::{
    gdk::Key,
    gio::{self, SimpleAction},
    glib::{self, clone, ExitCode},
    prelude::{ActionMapExt, ApplicationExt, ApplicationExtManual},
    traits::{BoxExt, ButtonExt, GestureSingleExt, GtkWindowExt, WidgetExt},
    Button,
};

const APP_ID: &'static str = "org.dashie.OxiShut";

fn main() -> glib::ExitCode {
    let mut css_string = "".to_string();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut argiter = args.iter();
        argiter.next().unwrap();
        match argiter.next().unwrap().as_str() {
            "--css" => {
                let next = argiter.next();
                if next.is_some() {
                    css_string = next.unwrap().clone();
                }
            }
            _ => {
                print!(
                    "usage:
    --css: use a specific path to load a css style sheet.
    --help: show this message.\n"
                );
                return ExitCode::FAILURE;
            }
        }
    } else {
        css_string = create_config_dir().to_str().unwrap().into();
    }

    gio::resources_register_include!("src.templates.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(move |_| {
        adw::init().unwrap();
        load_css(&css_string);
    });

    app.connect_activate(build_ui);
    app.run_with_args(&[""])
}

fn build_ui(app: &Application) {
    let mainbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    mainbox.set_css_classes(&[&"mainbox"]);
    mainbox.set_halign(gtk::Align::Fill);
    mainbox.set_homogeneous(true);

    let window = ApplicationWindow::builder()
        .application(app)
        .name("mainwindow")
        .content(&mainbox)
        .build();
    window.set_vexpand(false);
    window.set_default_size(800, 350);

    let action_shutdown = SimpleAction::new("shutdown", None);
    let action_reboot = SimpleAction::new("reboot", None);
    let action_sleep = SimpleAction::new("sleep", None);

    let button_shutdown = Button::new();
    button_shutdown.connect_clicked(|button| {
        button.activate_action("win.shutdown", None).expect("");
    });
    button_shutdown.set_icon_name("shutdown");
    button_shutdown.set_css_classes(&[&"button_shutdown", &"button"]);

    let button_reboot = Button::new();
    button_reboot.connect_clicked(|button| {
        button.activate_action("win.reboot", None).expect("");
    });
    button_reboot.set_icon_name("reboot");
    button_reboot.set_css_classes(&[&"button_reboot", &"button"]);

    let button_sleep = Button::new();
    button_sleep.connect_clicked(|button| {
        button.activate_action("win.sleep", None).expect("");
    });
    button_sleep.set_icon_name("sleep");
    button_sleep.set_css_classes(&[&"button_sleep", &"button"]);

    mainbox.append(&button_shutdown);
    mainbox.append(&button_reboot);
    mainbox.append(&button_sleep);

    action_shutdown.connect_activate(clone!(@weak window => move |_, _| {
        Command::new("shutdown")
            .arg("now")
            .spawn()
            .expect("No shutdown process available?");
        window.close();
    }));

    action_reboot.connect_activate(clone!(@weak window => move |_, _| {
        Command::new("reboot")
            .spawn()
            .expect("No reboot available?");
        window.close();
    }));

    action_sleep.connect_activate(clone!(@weak window => move |_, _| {
        Command::new("playerctl")
            .arg("-a")
            .arg("pause")
            .spawn()
            .expect("No playerctl available?");
        Command::new("swaylock")
            .arg("-c")
            .arg("000000")
            .spawn()
            .expect("No swaylock available?");
        Command::new("systemctl")
            .arg("suspend")
            .spawn()
            .expect("No soystemd available?");
        window.close();
    }));

    window.add_action(&action_shutdown);
    window.add_action(&action_reboot);
    window.add_action(&action_sleep);

    gtk4_layer_shell::init_for_window(&window);
    gtk4_layer_shell::set_keyboard_mode(&window, gtk4_layer_shell::KeyboardMode::Exclusive);
    gtk4_layer_shell::auto_exclusive_zone_enable(&window);
    gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Overlay);

    let windowrc = Rc::new(window.clone());
    let windowrc2 = windowrc.clone();

    let focus_event_controller = gtk::EventControllerFocus::new();
    focus_event_controller.connect_leave(move |_| {
        windowrc.close();
    });

    let gesture = gtk::GestureClick::new();
    gesture.set_button(gtk::gdk::ffi::GDK_BUTTON_PRIMARY as u32);

    gesture.connect_pressed(move |_gesture, _, _, _| {});

    let key_event_controller = gtk::EventControllerKey::new();
    key_event_controller.connect_key_pressed(move |_controller, key, _keycode, _state| match key {
        Key::_1 => {
            windowrc2.activate_action("win.shutdown", None).expect("");
            windowrc2.close();
            gtk::Inhibit(true)
        }
        Key::_2 => {
            windowrc2.activate_action("win.reboot", None).expect("");
            windowrc2.close();
            gtk::Inhibit(true)
        }
        Key::_3 => {
            windowrc2.activate_action("win.sleep", None).expect("");
            windowrc2.close();
            gtk::Inhibit(true)
        }
        Key::Escape => {
            windowrc2.close();
            gtk::Inhibit(true)
        }
        Key::Super_L => {
            windowrc2.close();
            gtk::Inhibit(true)
        }
        _ => gtk::Inhibit(false),
    });

    window.add_controller(key_event_controller);
    window.add_controller(focus_event_controller);
    window.add_controller(gesture);
    window.present();
}

fn load_css(css_string: &String) {
    let context_provider = gtk::CssProvider::new();
    if css_string != "" {
        context_provider.load_from_path(css_string);
    }

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(),
        &context_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn create_config_dir() -> PathBuf {
    let maybe_config_dir = dirs::ProjectDirs::from("com", "dashie", "oxishut");
    if maybe_config_dir.is_none() {
        panic!("Could not get config directory");
    }
    let config = maybe_config_dir.unwrap();
    let config_dir = config.config_dir();
    if !config_dir.exists() {
        fs::create_dir(config_dir).expect("Could not create config directory");
    }
    let file_path = config_dir.join("style.css");
    if !file_path.exists() {
        fs::File::create(&file_path).expect("Could not create css config file");
        fs::write(
            &file_path,
            "#MainWindow {
                border-radius: 10px;
            }",
        )
        .expect("Could not write default values");
    }
    file_path
}
