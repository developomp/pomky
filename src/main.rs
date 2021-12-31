mod cpu;
mod disk;
mod general;
mod image;
mod memory;
mod network;

mod custom_components;
mod util;

use gtk::prelude::{ApplicationExt, ApplicationExtManual, CssProviderExt, GtkWindowExt, WidgetExt};

use util::get_widget;

const TOP_MARGIN: i32 = 40;
const RIGHT_MARGIN: i32 = 10;

fn main() {
    let application = gtk::Application::new(Some("com.developomp.pomky"), Default::default());

    // only here to prevent warning
    application.connect_activate(|_| {});

    application.connect_startup(|app| {
        setup_css();
        build_ui(app);
    });

    application.run();
}

fn setup_css() {
    let provider = gtk::CssProvider::new();

    provider
        .load_from_data(include_bytes!("style.css"))
        .expect("Failed to load CSS");

    // We give the CssProvided to the default screen so the CSS rules we added
    // can be applied to our window.
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error initializing gtk css provider."),
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

    let window = get_widget::<gtk::ApplicationWindow>("window_main", &builder);
    window.set_application(Some(application));

    window.connect_screen_changed(set_visual);
    window.connect_draw(|_, ctx| {
        // set transparent window background
        ctx.set_source_rgba(0.15, 0.15, 0.15, 0.5);
        ctx.paint().expect("Failed to paint background");

        return gtk::Inhibit(false);
    });

    set_visual(&window, None);

    // window positioning
    unsafe {
        window.move_(
            gdk::ffi::gdk_screen_width() - window.default_width() - RIGHT_MARGIN,
            TOP_MARGIN,
        );
    }

    // =====[ Setup Stats ]=====

    general::setup(&builder);
    cpu::setup(&builder);
    memory::setup(&builder);
    network::setup(&builder);
    disk::setup(&builder);

    window.show_all();
}
