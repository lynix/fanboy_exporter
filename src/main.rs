/*
    fanboy_exporter
    (c) Alexander Koch <mail@alexanderkoch.net
*/

// SPDX-License-Identifier: MIT

use clap::{self, App, Arg};
use git_version::git_version;
use env_logger::{
    Builder,
    Env,
};
use prometheus_exporter::prometheus::{
    register_gauge_vec,
    register_int_gauge_vec,
};
use prometheus::{
    GaugeVec,
    core::GenericGaugeVec,
    core::AtomicI64,
};
use std::{
    str::FromStr,
    net::{IpAddr, SocketAddr},
};
use crate::fanboy::{
    FanBoy,
    fanboy_init,
    NUM_FANS,
    NUM_TEMP,
};

const ARG_ADDR: &str = "bind";
const ARG_PORT: &str = "port";
const ARG_INT:  &str = "interval";
const ARG_DEV:  &str = "device";
const DEF_ADDR: &str = "0.0.0.0";
const DEF_PORT: &str = "9184";
const DEF_INT:  &str = "10";
const DEF_DEV:  &str = "/dev/ttyACM0";

mod fanboy;

fn update(fanboy: &mut FanBoy, temps: &mut GaugeVec,
        rpms: &mut GenericGaugeVec<AtomicI64>,
        duties: &mut GenericGaugeVec<AtomicI64>) {
    fanboy.update();

    for n in 0..NUM_FANS {
        let label = format!("FAN{}", n);
        rpms.with_label_values(&[&label]).set(fanboy.rpm[n].into());
        duties.with_label_values(&[&label]).set(fanboy.duty[n].into());
    }
    for n in 0..NUM_TEMP {
        let label = format!("TEMP{}", n);
        temps.with_label_values(&[&label]).set(fanboy.temp[0].into());
    }
}

fn main() {
    let matches = App::new("fanboy_exporter")
        .version(git_version!())
        .about("Export FanBoy sensor data to Prometheus")
        .arg(Arg::with_name(ARG_ADDR)
             .short("b")
             .value_name("ADDR")
             .help(&format!("Listen address (default: {})", DEF_ADDR))
             .takes_value(true))
        .arg(Arg::with_name(ARG_PORT)
             .short("p")
             .value_name("PORT")
             .help(&format!("TCP port (default: {})", DEF_PORT))
             .takes_value(true))
        .arg(Arg::with_name(ARG_INT)
             .short("i")
             .value_name("INT")
             .help(&format!("Update interval in seconds (default: {})", DEF_INT))
             .takes_value(true))
        .arg(Arg::with_name(ARG_DEV)
             .short("d")
             .value_name("DEV")
             .help(&format!("Device (default: {})", DEF_DEV))
             .takes_value(true))
        .get_matches();

    let addr = matches.value_of(ARG_ADDR).unwrap_or(DEF_ADDR);
    let port = matches.value_of(ARG_PORT).unwrap_or(DEF_PORT);
    let interval = matches.value_of(ARG_INT).unwrap_or(DEF_INT);
    let device = matches.value_of(ARG_DEV).unwrap_or(DEF_DEV);

    let ia: IpAddr = addr.parse().expect("invalid listen addr");
    let sa = SocketAddr::new(ia, port.parse()
        .expect("invalid listen addr or port"));
    let duration = std::time::Duration::from_secs(u64::from_str(interval)
        .expect("invalid interval"));

    // setup logger for messages from 'prometheus' module
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut fanboy: FanBoy = fanboy_init(device).expect("failed to open serial port");

    let exporter = prometheus_exporter::start(sa)
        .expect("can not start exporter");
    let mut temps = register_gauge_vec!("fanboy_temp", "FanBoy temperatures",
        &["sensor"]).expect("failed to create sensor gauge");
    let mut rpms = register_int_gauge_vec!("fanboy_rpm", "FanBoy RPM values",
        &["fan"]).expect("failed to create rpm gauge");
    let mut duties = register_int_gauge_vec!("fanboy_duty",
        "FanBoy duty values", &["fan"]).expect("failed to create duty gauge");

    update(&mut fanboy, &mut temps, &mut rpms, &mut duties);
    loop {
        let _guard = exporter.wait_duration(duration);
        update(&mut fanboy, &mut temps, &mut rpms, &mut duties);
    }
}
