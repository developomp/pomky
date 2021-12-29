use gdk::glib::Sender;
use gdk::prelude::Continue;
use gtk::glib;
use gtk::prelude::LabelExt;
use gtk::Label;

use sysinfo::{NetworkExt, RefreshKind, System, SystemExt};

use crate::custom_components::graph::build_graph;
use crate::util::{get_widget, pretty_bytes};

// interval fixed at 1 sec to keep speed data accurate

pub const DEVICE_ETHERNET: &str = "enp3s0";
pub const DEVICE_WIFI: &str = "wlp5s0";

const GRAPH_WIDTH: i32 = 248;
const GRAPH_HEIGHT: i32 = 50;

pub fn setup(builder: &gtk::Builder) {
    let label_ethernet_upload_speed = get_widget::<Label>("label_ethernet_up_speed", &builder);
    let label_ethernet_download_speed = get_widget::<Label>("label_ethernet_down_speed", &builder);
    let label_wifi_upload_speed = get_widget::<Label>("label_wifi_up_speed", &builder);
    let label_wifi_download_speed = get_widget::<Label>("label_wifi_down_speed", &builder);

    let mut sys =
        System::new_with_specifics(RefreshKind::new().with_networks().with_networks_list());

    let (data_tx, data_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (ethernet_up_tx, ethernet_up_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (ethernet_down_tx, ethernet_down_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    data_rx.attach(None, move |(device_type, transmitted, received)| {
        match device_type {
            DEVICE_ETHERNET => {
                update_label(&label_ethernet_upload_speed, transmitted);
                update_label(&label_ethernet_download_speed, received);

                ethernet_up_tx.send(transmitted).unwrap();
                ethernet_down_tx.send(received).unwrap();
            }

            DEVICE_WIFI => {
                update_label(&label_wifi_upload_speed, transmitted);
                update_label(&label_wifi_download_speed, received);
            }

            _ => {}
        };

        return Continue(true);
    });

    build_graph(
        get_widget("drawing_area_ethernet_upload", &builder),
        GRAPH_WIDTH,
        GRAPH_HEIGHT,
        ethernet_up_rx,
        None,
    );

    build_graph(
        get_widget("drawing_area_ethernet_download", &builder),
        GRAPH_WIDTH,
        GRAPH_HEIGHT,
        ethernet_down_rx,
        None,
    );

    update(&mut sys, &data_tx);
    glib::timeout_add_seconds_local(1, move || {
        update(&mut sys, &data_tx);

        return glib::Continue(true);
    });
}

fn update(sys: &mut System, data_tx: &Sender<(&str, u64, u64)>) {
    sys.refresh_networks();

    for (_, (interface_name, data)) in sys.networks().into_iter().enumerate() {
        let transmitted = data.transmitted();
        let received = data.received();

        match interface_name.as_str() {
            DEVICE_ETHERNET => {
                data_tx
                    .send((DEVICE_ETHERNET, transmitted, received))
                    .unwrap();
            }

            DEVICE_WIFI => {
                data_tx.send((DEVICE_WIFI, transmitted, received)).unwrap();
            }

            _ => {}
        }
    }
}

fn update_label(label: &Label, value: u64) {
    label.set_text(&pretty_bytes(value));
}
