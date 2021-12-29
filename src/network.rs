use gdk::glib::Sender;
use gdk::prelude::Continue;
use gtk::glib;
use gtk::prelude::LabelExt;
use gtk::Label;

use sysinfo::{NetworkExt, RefreshKind, System, SystemExt};

use crate::custom_components::graph::build_graph;
use crate::util::{get_widget, pretty_bytes};

// must be set to 1000 to keep the calculation somewhat accurate
const NETWORK_UPDATE_INTERVAL: u32 = 1000;

pub const DEVICE_ETHERNET: &str = "enp3s0";
pub const DEVICE_WIFI: &str = "wlp5s0";

pub fn setup(builder: &gtk::Builder) {
    let label_ethernet_upload_speed = get_widget::<Label>("label_ethernet_up_speed", &builder);
    let label_ethernet_download_speed = get_widget::<Label>("label_ethernet_down_speed", &builder);
    let label_wifi_upload_speed = get_widget::<Label>("label_wifi_up_speed", &builder);
    let label_wifi_download_speed = get_widget::<Label>("label_wifi_down_speed", &builder);

    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (ethernet_up_tx, ethernet_up_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let mut sys =
        System::new_with_specifics(RefreshKind::new().with_networks().with_networks_list());

    build_graph(
        get_widget("drawing_area_ethernet_upload", &builder),
        250,
        100,
        ethernet_up_rx,
    );

    rx.attach(None, move |(device_type, transmitted, received)| {
        match device_type {
            DEVICE_ETHERNET => {
                update_label(&label_ethernet_upload_speed, transmitted);
                update_label(&label_ethernet_download_speed, received);

                ethernet_up_tx.send(transmitted).unwrap();
            }

            DEVICE_WIFI => {
                update_label(&label_wifi_upload_speed, transmitted);
                update_label(&label_wifi_download_speed, received);
            }

            _ => {}
        };

        return Continue(true);
    });

    update(&mut sys, &tx);
    glib::timeout_add_seconds_local(NETWORK_UPDATE_INTERVAL / 1000, move || {
        update(&mut sys, &tx);

        return glib::Continue(true);
    });
}

fn update(sys: &mut System, tx: &Sender<(&str, u64, u64)>) {
    sys.refresh_networks();

    for (_, (interface_name, data)) in sys.networks().into_iter().enumerate() {
        let transmitted = data.transmitted();
        let received = data.received();

        match interface_name.as_str() {
            DEVICE_ETHERNET => {
                tx.send((DEVICE_ETHERNET, transmitted, received)).unwrap();
            }

            DEVICE_WIFI => {
                tx.send((DEVICE_WIFI, transmitted, received)).unwrap();
            }

            _ => {}
        }
    }
}

fn update_label(label: &Label, value: u64) {
    label.set_text(&pretty_bytes(value));
}
