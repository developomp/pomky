mod config;
mod custom_components;
mod image;
mod stats;
mod util;

use config::CONFIG_LOCK;
use gdk::Screen;
use gtk::{
    prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt},
    traits::{CssProviderExt, WidgetExt},
    CssProvider, StyleContext,
};
use util::get_widget;

fn main() {
    // Terminate if configuration fails to load
    if let Err(error_msg) = config::Config::load() {
        println!("{}", error_msg);
        std::process::exit(-1);
    }

    let application = gtk::Application::new(Some("com.developomp.pomky"), Default::default());

    // https://lazka.github.io/pgi-docs/Gio-2.0/classes/Application.html#Gio.Application.signals.startup
    application.connect_startup(|_| {
        setup_css();
    });

    // https://lazka.github.io/pgi-docs/Gio-2.0/classes/Application.html#Gio.Application.signals.activate
    application.connect_activate(build_ui);

    application.run();
}

fn setup_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();

    provider
        .load_from_data(include_bytes!("style.css"))
        .expect("Failed to load CSS data");

    // Apply CSS
    StyleContext::add_provider_for_screen(
        &Screen::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn set_visual(window: &gtk::ApplicationWindow, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = window.screen() {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual)); // crucial for transparency
        }
    }
}

fn build_ui(application: &gtk::Application) {
    // load design.ui
    let builder: gtk::Builder = gtk::Builder::from_string(include_str!("design.ui"));

    // ==========[ Window ]==========

    let window: gtk::ApplicationWindow = get_widget("window_main", &builder);
    window.set_application(Some(application));

    window.connect_screen_changed(set_visual);
    window.connect_draw(|_, ctx| {
        // set transparent window background
        ctx.set_source_rgba(0.15, 0.15, 0.15, 0.5);
        ctx.paint().expect("Failed to paint background");

        return gtk::Inhibit(false);
    });

    set_visual(&window, None);

    let (x, y) = CONFIG_LOCK.read().unwrap().calculate_position(&window);
    window.move_(x, y);

    // =====[ Setup Stats ]=====

    stats::general::setup(&builder);
    stats::processes::setup(&builder);
    stats::cpu::setup(&builder);
    stats::memory::setup(&builder);
    stats::network::setup(&builder);
    stats::disk::setup(&builder);

    window.show();
}
