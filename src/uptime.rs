use gdk::glib;
use gtk::prelude::LabelExt;
use gtk::Builder;
use sysinfo::{RefreshKind, System, SystemExt};

use crate::util::get_widget;
use crate::{SECONDS_IN_DAY, SECONDS_IN_HOUR, SECONDS_IN_MINUTE};

const UPTIME_UPDATE_INTERVAL: u32 = 60;

pub fn setup(builder: &Builder) {
    let label_uptime = get_widget("label_uptime", &builder);

    let sys = System::new_with_specifics(RefreshKind::new());

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
