/*
    fanboy_exporter
    (c) Alexander Koch <mail@alexanderkoch.net
*/

// SPDX-License-Identifier: MIT

use std::time::Duration;
use serialport::SerialPort;
use log::error;
use byteorder::{
    LittleEndian,
    ReadBytesExt,
};

pub const NUM_FANS: usize = 4;
pub const NUM_TEMP: usize = 2;

const MSG_SOF: u8 = 0x42;
const MSG_CMD_STATUS: u8 = 0x01;

pub struct FanBoy {
    pub temp: [f32; NUM_TEMP],
    pub rpm: [u16; NUM_FANS],
    pub duty: [u8; NUM_FANS],
    port: Box<dyn SerialPort>,
}

pub fn fanboy_init(device: &str) -> Result<FanBoy, serialport::Error> {
    let p = match serialport::new(device, 115_200)
        .timeout(Duration::from_millis(500))
        .open() {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
    let f = FanBoy {
        temp: [0.0, 0.0],
        rpm: [0, 0, 0, 0],
        duty: [0, 0, 0, 0],
        port: p,
    };

    return Ok(f);
}

impl FanBoy {
    pub fn update(&mut self) {
        let query: [u8; 2] = [MSG_SOF, MSG_CMD_STATUS];
        self.port.write(&query).expect("failed to send query");

        let mut response: Vec<u8> = vec![0;18];
        let nread = match self.port.read(response.as_mut_slice()) {
            Ok(n) => n,
            Err(_e) => 0,
        };
        if nread < response.len() {
            error!("failed to receive status reply");
            return;
        }

        let mut current = &response[..];
        if current.read_u8().unwrap() != MSG_SOF ||
                current.read_u8().unwrap() != MSG_CMD_STATUS {
            error!("invalid SOF or CMD byte in reply");
            return;
        }

        for i in 0..NUM_FANS {
            self.duty[i] = current.read_u8().unwrap();
            self.rpm[i] = current.read_u16::<LittleEndian>().unwrap();
            if self.rpm[i] == u16::MAX {
                self.rpm[i] = 0;
            }
        }
        for i in 0..NUM_TEMP {
            let t = current.read_u16::<LittleEndian>().unwrap();
            self.temp[i] = t as f32 / 100.0;
        }
    }
}