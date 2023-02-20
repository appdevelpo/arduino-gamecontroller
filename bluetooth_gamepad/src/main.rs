use std::io::{self};
use std::{thread, time};
use vigem::types::button::*;

use vigem::*;
fn str_2_u8(x: &str) ->u8{
    x.parse::<u8>().unwrap()
}
fn main() {

    thread::sleep(time::Duration::from_millis(1000));
    let mut read_dat = String::new();
    // controller side
    let mut vig = Vigem::new();
    vig.connect().unwrap();
    let mut target = types::target::Target::new(TargetType:: Xbox360);
    vig.target_add(&mut target).unwrap();
    //serial port
    let mut port = serialport::new("COM12", 115200).open().expect("Failed to open port");
    let mut serial_buf: Vec<u8> = vec![0; 64];
    let mut report = XUSBReport {
        w_buttons: XButton::B | XButton::DpadDown,
        b_right_trigger: 100,
        s_thumb_lx: 32000,
        ..XUSBReport::default()
    };

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) =>
                print!(""),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
        let  dat = serial_buf.as_mut_slice();
        let dat = std::str::from_utf8(&dat).unwrap();
        read_dat += dat;
        let  dat_list: Vec<&str> = read_dat.split("\r\n").collect();
        if read_dat.len() < 64*3{
            continue;
        }
        let  length_of_data = dat_list.len();
        let  controller_data: Vec<&str> = dat_list[length_of_data-2].split(' ').collect();
        if read_dat.len() > 16384{
            read_dat.clear();
            continue
        }

        if controller_data.len() != 7 {
                continue;
            }
            let button_dat = controller_data[6];


            println!("{:?}",button_dat);
            let mut input = XButton::Nothing;
            let  left_trigger:u8 = str_2_u8(controller_data[5]);
            let  right_trigger:u8 = str_2_u8((controller_data[4]));
            for (i, c) in button_dat.chars().enumerate() {
                if c == '1' {
                    match i {
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
                }else{

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

