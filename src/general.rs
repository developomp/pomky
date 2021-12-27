use gdk::glib;
use gtk::prelude::LabelExt;
use gtk::Builder;
use sysinfo::{RefreshKind, System, SystemExt};

use crate::util::get_widget;
use crate::{SECONDS_IN_DAY, SECONDS_IN_HOUR, SECONDS_IN_MINUTE};

const UPTIME_UPDATE_INTERVAL: u32 = 60_000;

pub fn setup(builder: &Builder) {
    let label_kernel_version = get_widget::<gtk::Label>("label_kernel_version", &builder);
    let label_uptime = get_widget("label_uptime", &builder);

    let sys = System::new_with_specifics(RefreshKind::new());

    let kernel_version = sys.kernel_version();
    let kernel_version = match kernel_version {
        Some(ref value) => &value,

        None => "not available",
    };
    label_kernel_version.set_text(kernel_version);

    update(&sys, &label_uptime);

    // update every minute
    glib::timeout_add_seconds_local(UPTIME_UPDATE_INTERVAL / 1000, move || {
        update(&sys, &label_uptime);

        return glib::Continue(true);
    });
}

fn update(sys: &System, label_uptime: &gtk::Label) {
    let mut result = String::from("");

    let days: u64;
    let hours: u64;
    let minutes: u64;

    let mut uptime = sys.uptime();

    if uptime > SECONDS_IN_DAY {
        days = uptime / SECONDS_IN_DAY;
        uptime -= days * SECONDS_IN_DAY;

        result.push_str(&format!("{} days ", days));
    }

    if uptime > SECONDS_IN_HOUR {
        hours = uptime / SECONDS_IN_HOUR;
        uptime -= hours * SECONDS_IN_HOUR;

        result.push_str(&format!("{} hours ", hours));
    }

    minutes = uptime / SECONDS_IN_MINUTE;
    result.push_str(&format!("{} minutes", minutes));

    label_uptime.set_text(&result);
}
