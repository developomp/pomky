use sysinfo::{ProcessorExt, RefreshKind, System, SystemExt};

use crate::bar::build_bar;

const CPU_UPDATE_INTERVAL: u32 = 500;

pub fn setup(builder: &gtk::Builder) {
    build_bar(
        builder,
        "drawing_area_cpu_percent_bar",
        500,
        6,
        CPU_UPDATE_INTERVAL as u64,
        || {
            return System::new_with_specifics(RefreshKind::new());
        },
        |sys| {
            sys.refresh_cpu();

            let mut total_percent = 0.0;

            for processor in sys.processors().into_iter() {
                total_percent += processor.cpu_usage();
            }

            return (total_percent / 800.0) as f64;
        },
    );

    for i in 0..8 {
        build_bar(
            builder,
            format!("drawing_area_cpu{}_percent", i).as_str(),
            120,
            6,
            CPU_UPDATE_INTERVAL as u64,
            || {
                return System::new_with_specifics(RefreshKind::new());
            },
            move |sys| {
                sys.refresh_cpu();

                return sys.processors()[i].cpu_usage() as f64 / 100.0;
            },
        );
    }
}
