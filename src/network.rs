use gdk::glib;
use gtk::prelude::LabelExt;
use sysinfo::{NetworkExt, RefreshKind, System, SystemExt};

use crate::util::{get_widget, msec_2_duration, pretty_bytes};

const NETWORK_UPDATE_INTERVAL: u32 = 1000;

pub fn setup(builder: &gtk::Builder) {
    let label_ethernet_up_speed = get_widget("label_ethernet_up_speed", &builder);
    let label_ethernet_down_speed = get_widget("label_ethernet_down_speed", &builder);
    let label_wifi_up_speed = get_widget("label_wifi_up_speed", &builder);
    let label_wifi_down_speed = get_widget("label_wifi_down_speed", &builder);

    let mut sys =
        System::new_with_specifics(RefreshKind::new().with_networks().with_networks_list());

    update(
        &mut sys,
        &label_ethernet_up_speed,
        &label_ethernet_down_speed,
        &label_wifi_up_speed,
        &label_wifi_down_speed,
    );

    glib::timeout_add_local(msec_2_duration(NETWORK_UPDATE_INTERVAL), move || {
        update(
            &mut sys,
            &label_ethernet_up_speed,
            &label_ethernet_down_speed,
            &label_wifi_up_speed,
            &label_wifi_down_speed,
        );

        return glib::Continue(true);
    });
}

fn update(
    sys: &mut System,
    label_ethernet_up_speed: &gtk::Label,
    label_ethernet_down_speed: &gtk::Label,
    label_wifi_up_speed: &gtk::Label,
    label_wifi_down_speed: &gtk::Label,
) {
    sys.refresh_networks();

    for (interface_name, data) in sys.networks() {
        match interface_name.as_str() {
            "enp3s0" => {
                label_ethernet_up_speed.set_text(&pretty_bytes(data.transmitted()));
                label_ethernet_down_speed.set_text(&pretty_bytes(data.received()));
            }

            "wlp5s0" => {
                label_wifi_up_speed.set_text(&pretty_bytes(data.transmitted()));
                label_wifi_down_speed.set_text(&pretty_bytes(data.received()));
            }

            _ => {}
        }
    }
}
