mod cpu;
mod disk;
mod general;
mod image;
mod memory;
mod network;
mod processes;

mod custom_components;
mod util;

use gdk::Screen;
use gtk::{
    prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt},
    traits::{CssProviderExt, WidgetExt},
    CssProvider, StyleContext,
};

use util::get_widget;

const TOP_MARGIN: i32 = 40;
const RIGHT_MARGIN: i32 = 10;

fn main() {
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

    // move window to the top-right corner of the screen with margin (compensating for the GNOME top bar)
    unsafe {
        window.move_(
            gdk::ffi::gdk_screen_width() - window.default_width() - RIGHT_MARGIN,
            TOP_MARGIN,
        );
    }

    // =====[ Setup Stats ]=====

    general::setup(&builder);
    processes::setup(&builder);
    cpu::setup(&builder);
    memory::setup(&builder);
    network::setup(&builder);
    disk::setup(&builder);

    window.show();
}
