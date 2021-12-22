use gdk;
use gtk::glib;
use gtk::prelude::*;

use sysinfo;
use sysinfo::SystemExt;

const SECONDS_IN_DAY: u64 = 86400;
const SECONDS_IN_HOUR: u64 = 3600;
const SECONDS_IN_MINUTE: u64 = 60;

fn main() {
    let application = gtk::Application::new(Some("com.developomp.pomky"), Default::default());

    // only here to prevent warning
    application.connect_activate(|_| {});

    application.connect_startup(|app| {
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

        // We build the application UI.
        build_ui(app);
    });

    application.run();
}

fn set_visual(window: &gtk::ApplicationWindow, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = window.screen() {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual)); // crucial for transparency
        }
    }
}

fn build_ui(application: &gtk::Application) {
    let mut sys = sysinfo::System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // load design.ui
    let builder = gtk::Builder::from_string(include_str!("design.ui"));

    // ==========[ Window ]==========

    let window: gtk::ApplicationWindow = builder
        .object("window_main")
        .expect("Couldn't get window_main");
    window.set_application(Some(application));

    window.connect_screen_changed(set_visual);
    window.connect_draw(|_window, ctx| {
        ctx.set_source_rgba(0.0, 0.0, 0.0, 50.0 / 255.0);
        ctx.paint().expect("Failed to paint background");

        return Inhibit(false);
    });

    set_visual(&window, None);

    // move window to upper right corner
    unsafe {
        window.move_(
            gdk::ffi::gdk_screen_width() - window.default_width() - 10,
            40,
        );
    }

    // ==========[ Kernel label ]==========

    let label_kernel_version: gtk::Label = builder
        .object("label_kernel_version")
        .expect("Couldn't get Kernel version label");

    let kernel_version = sys.kernel_version();
    let kernel_version = match kernel_version {
        None => "not available",
        Some(ref value) => value.as_str(),
    };
    label_kernel_version.set_text(format!("Kernel: {}", kernel_version).as_str());

    // ==========[ Uptime ]==========

    let label_uptime: gtk::Label = builder
        .object("label_uptime")
        .expect("Couldn't get uptime label");

    update_uptime(&label_uptime, sys.uptime());

    // update every minute
    glib::timeout_add_seconds_local(60, move || {
        update_uptime(&label_uptime, sys.uptime());
        return glib::Continue(true);
    });

    // ==========[ Show window ]==========

    window.show_all();
}

fn update_uptime(label: &gtk::Label, mut uptime: u64) {
    let mut result = String::from("");
    let days: u64;
    let hours: u64;
    let minutes: u64;

    if uptime > SECONDS_IN_DAY {
        days = uptime / SECONDS_IN_DAY;
        uptime -= days * SECONDS_IN_DAY;

        result.push_str(format!("{} days ", days).as_str());
    }

    if uptime > SECONDS_IN_HOUR {
        hours = uptime / SECONDS_IN_HOUR;
        uptime -= hours * SECONDS_IN_HOUR;

        result.push_str(format!("{} hours ", hours).as_str());
    }

    minutes = uptime / SECONDS_IN_MINUTE;
    result.push_str(format!("{} minutes", minutes).as_str());

    label.set_text(format!("Uptime: {}", result).as_str());
}
