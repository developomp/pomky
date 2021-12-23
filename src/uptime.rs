use gdk::glib;
use gtk::prelude::{BuilderExtManual, LabelExt};
use gtk::Builder;
use sysinfo::{RefreshKind, System, SystemExt};

use crate::{SECONDS_IN_DAY, SECONDS_IN_HOUR, SECONDS_IN_MINUTE, UPTIME_UPDATE_INTERVAL};

pub fn setup(builder: &Builder) {
    let sys = System::new_with_specifics(RefreshKind::new());

    let label_uptime: gtk::Label = builder
        .object("label_uptime")
        .expect("Couldn't get uptime label");

    update_uptime(&label_uptime, sys.uptime());

    // update every minute
    glib::timeout_add_seconds_local(UPTIME_UPDATE_INTERVAL, move || {
        update_uptime(&label_uptime, sys.uptime());

        return glib::Continue(true);
    });
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
