use gdk;
use gtk::glib;
use gtk::prelude::*;

use sysinfo::{Disk, DiskExt, RefreshKind, System, SystemExt};

const UPTIME_UPDATE_INTERVAL: u32 = 60;
const MEMORY_UPDATE_INTERVAL: u32 = 1;
const DISK_UPDATE_INTERVAL: u32 = 5;

const SECONDS_IN_DAY: u64 = 86400;
const SECONDS_IN_HOUR: u64 = 3600;
const SECONDS_IN_MINUTE: u64 = 60;

// number of kibibytes in a gigabyte
const KIB_IN_GB: f64 = 1024_f64 * 1000_f64;

// number of bytes in a gigabyte
const B_IN_GB: f64 = 1000_f64 * 1000_f64 * 1000_f64;

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

    // ==========[ Kernel ]==========

    let sys = System::new_with_specifics(RefreshKind::new());

    let label_kernel_version: gtk::Label = builder
        .object("label_kernel_version")
        .expect("Couldn't get Kernel version label");

    let kernel_version = sys.kernel_version();
    let kernel_version = match kernel_version {
        None => "not available",
        Some(ref value) => value.as_str(),
    };
    label_kernel_version.set_text(kernel_version);

    // ==========[ Uptime ]==========

    let label_uptime: gtk::Label = builder
        .object("label_uptime")
        .expect("Couldn't get uptime label");

    update_uptime(&label_uptime, sys.uptime());

    // update every minute
    glib::timeout_add_seconds_local(UPTIME_UPDATE_INTERVAL, move || {
        update_uptime(&label_uptime, sys.uptime());

        return glib::Continue(true);
    });

    // ==========[ CPU ]==========

    // ==========[ Memory ]==========

    let mut sys = System::new_with_specifics(RefreshKind::new().with_memory());
    sys.refresh_memory();

    let label_memory_usage: gtk::Label = builder
        .object("label_memory_usage")
        .expect("Couldn't get memory usage label");
    let label_memory_free: gtk::Label = builder
        .object("label_memory_free")
        .expect("Couldn't get free memory label");

    update_memory_usage(&label_memory_usage, &sys);
    update_memory_free(&label_memory_free, &sys);

    glib::timeout_add_seconds_local(MEMORY_UPDATE_INTERVAL, move || {
        sys.refresh_memory();

        update_memory_usage(&label_memory_usage, &sys);
        update_memory_free(&label_memory_free, &sys);

        return glib::Continue(true);
    });

    // ==========[ Network ]==========

    // ==========[ Disk ]==========

    let label_disk_root: gtk::Label = builder
        .object("label_disk_root")
        .expect("Couldn't get root disk label");
    let label_disk_data: gtk::Label = builder
        .object("label_disk_data")
        .expect("Couldn't get data disk label");
    let label_disk_root_percent: gtk::Label = builder
        .object("label_disk_root_percent")
        .expect("Couldn't get root disk percent label");
    let label_disk_data_percent: gtk::Label = builder
        .object("label_disk_data_percent")
        .expect("Couldn't get data disk percent label");

    let mut sys = System::new_with_specifics(RefreshKind::new().with_disks().with_disks_list());
    sys.refresh_disks();
    update_disks(
        &sys,
        &label_disk_root,
        &label_disk_data,
        &label_disk_root_percent,
        &label_disk_data_percent,
    );

    let mut sys = System::new_with_specifics(RefreshKind::new().with_disks().with_disks_list());

    glib::timeout_add_seconds_local(DISK_UPDATE_INTERVAL, move || {
        sys.refresh_disks();
        update_disks(
            &sys,
            &label_disk_root,
            &label_disk_data,
            &label_disk_root_percent,
            &label_disk_data_percent,
        );

        return glib::Continue(true);
    });

    // ==========[ Show window ]==========

    window.show_all();
}

/// Converts kilobytes to gigabytes
fn kib_2_gb(kb: u64) -> f64 {
    return kb as f64 / KIB_IN_GB;
}

/// Converts bytes to gigabytes
fn b_2_gb(bytes: u64) -> f64 {
    return bytes as f64 / B_IN_GB;
}

fn update_disks(
    sys: &System,
    label_disk_root: &gtk::Label,
    label_disk_data: &gtk::Label,
    label_disk_root_percent: &gtk::Label,
    label_disk_data_percent: &gtk::Label,
) {
    for disk in sys.disks() {
        match disk.mount_point().to_str() {
            Some("/") => {
                update_disk(&label_disk_root, &label_disk_root_percent, disk);
            }

            Some("/media/pomp/data") => {
                update_disk(&label_disk_data, &label_disk_data_percent, disk);
            }

            Some(&_) => {}

            None => {}
        }
    }
}

fn update_disk(label: &gtk::Label, label_percent: &gtk::Label, disk: &Disk) {
    let total = disk.total_space();
    let used = total - disk.available_space();

    label.set_text(format!("{:.1}  /  {:.1}  GB", b_2_gb(used), b_2_gb(total)).as_str());
    label_percent.set_text(format!("{:.1}%", 100_f64 * used as f64 / total as f64).as_str());
}

fn update_memory_usage(label: &gtk::Label, sys: &System) {
    let mem_used = sys.used_memory();
    let mem_total = sys.total_memory();

    label.set_text(
        format!(
            "{:.1} GiB / {:.1} GiB ({:.1}%)",
            kib_2_gb(mem_used),
            kib_2_gb(mem_total),
            100 * mem_used / mem_total
        )
        .as_str(),
    );
}

fn update_memory_free(label: &gtk::Label, sys: &System) {
    label.set_text(format!("{:.1} GiB", kib_2_gb(sys.free_memory())).as_str());
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

    label.set_text(&result);
}
