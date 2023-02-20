use pad_motion::protocol::*;
use pad_motion::server::*;
use std::io::{self};
use std::time::{Instant};
use vigem::types::button::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use vigem::*;

fn str_2_float(x: &str) -> f32{
    x.parse::<f32>().unwrap()
}
fn str_2_u8(x: &str) ->u8{
    x.parse::<u8>().unwrap()
}
fn main() {
    let mut gyro_list = Vec::new();
    let running = Arc::new(AtomicBool::new(true));

    {
        let running = running.clone();
        ctrlc::set_handler(move || {
            running.store(false, Ordering::SeqCst);
        })
            .expect("Error setting Ctrl-C handler");
    }
    let server = Arc::new(Server::new(None, None).unwrap());
    let server_thread_join_handle = {
        let server = server.clone();
        server.start(running.clone())
    };

    let controller_info = ControllerInfo {
        slot_state: SlotState::Connected,
        device_type: DeviceType::FullGyro,
        connection_type: ConnectionType::USB,
        ..Default::default()
    };
    server.update_controller_info(controller_info);
    let mut read_dat = String::new();
    // controller side
    let mut vig = Vigem::new();
    vig.connect().unwrap();
    let mut target = Target::new(TargetType:: Xbox360);
    vig.target_add(&mut target).unwrap();
    //serial port
    let mut port = serialport::new("COM12", 115200).open().expect("Failed to open port");
    let mut serial_buf: Vec<u8> = vec![0; 32];
    let mut report = XUSBReport {
        w_buttons: XButton::B | XButton::DpadDown,
        b_right_trigger: 100,
        s_thumb_lx: 32000,
        ..XUSBReport::default()
    };
    let now = Instant::now();
    while running.load(Ordering::SeqCst) {
        match port.read(serial_buf.as_mut_slice()) {
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
            _ => ()
        }
        let  dat = serial_buf.as_mut_slice();
        let dat = std::str::from_utf8(&dat).unwrap();
        read_dat += dat;
        if read_dat.len() < 64*3{
            continue;
        }
        if read_dat.len() > 16384{
            read_dat.clear();
            continue
        }
        let  dat_list: Vec<&str> = read_dat.split("\r\n").collect();
        let  length_of_data = dat_list.len();
        let  controller_data: Vec<&str> = dat_list[length_of_data-2].split(' ').collect();

        if controller_data.len() != 7 {
            continue;
        }

        if controller_data[0] == "@"{
            for i in 1..7{
                let data = controller_data[i];
                gyro_list.push(str_2_float(data));
            }
            println!("{:?}",gyro_list);
            let controller_data = {
                ControllerData {
                    connected: true,
                    motion_data_timestamp: now.elapsed().as_micros() as u64,
                    accelerometer_x: gyro_list[0]/1638.4,
                    accelerometer_y:gyro_list[1]/1638.4,
                    accelerometer_z:-gyro_list[2]/1638.4,
                    gyroscope_pitch: gyro_list[4]/131.0,
                    gyroscope_roll:gyro_list[3]/131.0,
                    gyroscope_yaw:-gyro_list[5]/131.0,
                    ..Default::default()
                }
            };
            server.update_controller_data(0, controller_data);
            gyro_list.clear();
        }else {

            let button_dat = controller_data[6];


            let mut input = XButton::Nothing;
            let  left_trigger:u8 = str_2_u8(controller_data[5]);
            let  right_trigger:u8 = str_2_u8(controller_data[4]);
            for (i, c) in button_dat.chars().enumerate() {
                if c == '1' {
                    match i{
                        0 => input |= XButton::RightShoulder,
                        1 => input |= XButton::DpadUp,
                        2 => input |= XButton::DpadLeft,
                        3 => input |= XButton::DpadRight,
                        4 => input |= XButton::DpadDown,
                        5 => input |= XButton::X,
                        6 => input |= XButton::A,
                        7 => input |= XButton::B,
                        8 => input |= XButton::Y,
                        9 => input |= XButton::LeftShoulder,
                        10 => input |= XButton::Back,
                        11 => input |= XButton::Start,
                        12 => input |= XButton::RightThumb,
                        13 => input |= XButton::LeftThumb,
                        _ => ()
                    }
                }
            }

            report.w_buttons = input;
            report.b_left_trigger = left_trigger;
            report.b_right_trigger = right_trigger;
            let new_rx: i16 = controller_data[0].parse().unwrap();
            report.s_thumb_ry = (511-new_rx) * 64;
            let new_ry: i16 = controller_data[1].parse().unwrap();
            report.s_thumb_rx = (new_ry-512) * 64;
            let new_lx: i16 = controller_data[2].parse().unwrap();
            report.s_thumb_ly = (511-new_lx) * 64;
            let new_ly: i16 = controller_data[3].parse().unwrap();
            report.s_thumb_lx = ( new_ly-512) * 64;

            //update controller
            vig.update(&target, &report).unwrap();

        }

    }
    server_thread_join_handle.join().unwrap();

}

