// ---------------------------------------
// IOracle hardware
// ---------------------------------------
use crate::iching::Hexagram;
use futures::channel::mpsc;
use rocket::tokio::time::{sleep, Duration};
use rppal::gpio::Gpio;
use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType};

// const MULTY: f32 = 1.0;
const BIAS: f32 = 500.0;
const THRESHOLD: f32 = 10.0;

const LINE_READING_SLEEP: u64 = 3;
// const PIP_READING_DURATION: u64 = 2;
const LEDS_IN_LINE: i32 = 144;
const RESTING_LI_COLOUR: [u8; 4] = [255, 2, 14, 0];
const RESTING_YAO_COLOUR: [u8; 4] = [255, 2, 14, 0];
const READING_LI_COLOUR: [u8; 4] = [38, 2, 255, 0];
const READING_YAO_COLOUR: [u8; 4] = [38, 2, 255, 0];
const DISPLAYING_LI_COLOUR: [u8; 4] = [121, 255, 0, 0];
const DISPLAYING_YAO_COLOUR: [u8; 4] = [121, 255, 0, 0];

// const DEFAULT_COLOUR: &str = "rgb(51, 0, 180)";
// const LI_COLOUR: &str = "rgb(230, 4, 211)";
// const HEAVEN_COLOUR: &str = "rgb(224, 4, 235)";
// const CLOUD_COLOUR: &str = "rgb(255, 2, 14)";
// const SUN_COLOUR: &str = "rgb(255, 109, 0)";
// const WIND_COLOUR: &str = "rgb(121, 255, 0)";
// const THUNDER_COLOUR: &str = "rgb(255, 53, 6)";
// const WATER_COLOUR: &str = "rgb(38, 2, 255)";
// const MOUNTAIN_COLOUR: &str = "rgb(14, 255, 232)";
// const EARTH_COLOUR: &str = "rgb(0, 0, 0)";

pub enum Command {
    Rest,
    Read,
    Display(String),
}

pub async fn hardware_controll(mut receiver: mpsc::UnboundedReceiver<Command>) {
    loop {
        sleep(Duration::from_secs(2)).await;

        // Set colours
        match receiver.try_next() {
            // Message is fetched.
            Ok(Some(t)) => match t {
                Command::Rest => {
                    // ???
                    println!("Resting");
                }
                Command::Read => {
                    // ???
                    println!("Reading");
                }
                Command::Display(h) => {
                    // ???
                    println!("Displaying {}", h);
                }
            },
            // Channel is closed and no messages left in the queue.
            Ok(None) => {
                println!("None");
            }
            // There are no messages available, but channel is not yet closed.
            Err(_e) => {
                // println!("No messages: {}", e);
            }
        }
    }
}

pub fn build_controller(brightness: u8) -> rs_ws281x::Result<Controller> {
    ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(12)
                .count(6 * LEDS_IN_LINE)
                .strip_type(StripType::Ws2811Rgb)
                .brightness(brightness)
                .build(),
        )
        .channel(
            1,
            ChannelBuilder::new()
                .pin(13)
                .count(3 * LEDS_IN_LINE)
                .strip_type(StripType::Ws2811Rgb)
                .brightness(brightness)
                .build(),
        )
        .build()
}

pub fn rest(command_sender: mpsc::UnboundedSender<Command>) {
    println!("Resting..");
    let _ = command_sender.unbounded_send(Command::Rest);

    if let Ok(mut controller) = build_controller(50) {
        let yao = controller.leds_mut(0);
        for num in 0..yao.len() {
            yao[num as usize] = RESTING_YAO_COLOUR;
        }

        let li = controller.leds_mut(1);
        for num in 0..li.len() {
            li[num as usize] = RESTING_LI_COLOUR;
        }

        if let Err(e) = controller.render() {
            println!("Resting render error: {:?}", e);
        }
    } else {
        println!("NO LED!");
    }
}

pub async fn read(command_sender: mpsc::UnboundedSender<Command>) -> (Hexagram, Hexagram) {
    println!("Reading..");
    let _ = command_sender.unbounded_send(Command::Read);

    if let Ok(mut controller) = build_controller(100) {
        let yao = controller.leds_mut(0);
        for num in 0..yao.len() {
            yao[num as usize] = READING_YAO_COLOUR;
        }

        let li = controller.leds_mut(1);
        for num in 0..li.len() {
            li[num as usize] = READING_LI_COLOUR;
        }

        if let Err(e) = controller.render() {
            println!("Reading render error: {:?}", e);
        }
    } else {
        println!("NO LED!");
    }

    let mut hexagram = "".to_string();
    let mut tmp_hexagram = "".to_string();

    // Get first trigram
    for i in 1..4 {
        let line = read_line();
        hexagram = format!("{}{}", hexagram, line);
        println!("Line{}: {}", i, line);
        sleep(Duration::from_secs(LINE_READING_SLEEP)).await;
    }

    // React
    // TODO: react(hexagram);

    // Get related lines
    for i in 1..4 {
        let line = read_line();
        tmp_hexagram = format!("{}{}", tmp_hexagram, line);
        println!("Related line{}: {}", i, line);
        sleep(Duration::from_secs(LINE_READING_SLEEP)).await;
    }

    // Stop reaction
    // TODO: drop_pins();

    // Get second trigram
    for i in 4..7 {
        let line = read_line();
        hexagram = format!("{}{}", hexagram, line);
        println!("Line{}: {}", i, line);
        sleep(Duration::from_secs(LINE_READING_SLEEP)).await;
    }

    // React
    // TODO: react(hexagram);

    // Get next related lines
    for i in 4..7 {
        let line = read_line();
        tmp_hexagram = format!("{}{}", tmp_hexagram, line);
        println!("Related line{}: {}", i, line);
        sleep(Duration::from_secs(LINE_READING_SLEEP)).await;
    }

    // Stop reaction
    // TODO: drop_pins();

    // Calculate related hexagram
    let mut r_hexagram = "".to_string();
    for (x, y) in hexagram.chars().zip(tmp_hexagram.chars()) {
        if x.eq(&y) {
            if x.eq(&'0') {
                r_hexagram = format!("{}1", r_hexagram);
            } else {
                r_hexagram = format!("{}0", r_hexagram);
            }
        } else {
            r_hexagram = format!("{}{}", r_hexagram, x);
        }
    }
    println!("Hexagram: {}, R_Hexagram: {}", hexagram, r_hexagram);

    (hexagram, r_hexagram)
}

pub fn display(command_sender: mpsc::UnboundedSender<Command>, hexagram: Hexagram) {
    println!("Displaying..");
    let _ = command_sender.unbounded_send(Command::Display(hexagram));

    // TODO: parse hexagram, get colours

    if let Ok(mut controller) = build_controller(100) {
        let yao = controller.leds_mut(0);
        for num in 0..yao.len() {
            yao[num as usize] = DISPLAYING_YAO_COLOUR;
        }

        let li = controller.leds_mut(1);
        for num in 0..li.len() {
            li[num as usize] = DISPLAYING_LI_COLOUR;
        }

        if let Err(e) = controller.render() {
            println!("Displaying render error: {:?}", e);
        }
    } else {
        println!("NO LED!");
    }
}

//----------------------------------------------------------------------
// render the line
//----------------------------------------------------------------------
// pub fn render(l: u8, line_num: i32, controller: &mut Controller, colour: &String) {
//     match l {
//         1 => render_yang(line_num, controller, colour),
//         _ => render_yin(line_num, controller, colour),
//     }
// }
//
// pub fn render_yin(line_num: i32, controller: &mut Controller, colour: &String) {
//     let leds = controller.leds_mut(0);
//     let (a, b, c) = parse_colour(colour);
//
//     let part = LEDS_IN_LINE / 3;
//     let position = LEDS_IN_LINE * (line_num - 1);
//     for num in position..position + LEDS_IN_LINE {
//         if num > position + part && num < position + part * 2 {
//             leds[num as usize] = [0, 0, 0, 0];
//         } else {
//             leds[num as usize] = [c, a, b, 0];
//         }
//     }
//
//     if let Err(e) = controller.render() {
//         println!("{:?}", e);
//     };
// }
//
// pub fn render_yang(line_num: i32, controller: &mut Controller, colour: &String) {
//     let leds = controller.leds_mut(0);
//     let (a, b, c) = parse_colour(colour);
//
//     let position = LEDS_IN_LINE * (line_num - 1);
//     for num in position..position + LEDS_IN_LINE {
//         leds[num as usize] = [c, a, b, 0];
//     }
//
//     if let Err(e) = controller.render() {
//         println!("{:?}", e);
//     };
// }
//----------------------------------------------------------------------

fn read_line() -> u8 {
    let data = read_the_pip();

    // let mut min = 0;
    // if let Some(m) = data.iter().min() {
    //     min = *m;
    // };

    // let mut max = 0;
    // if let Some(m) = data.iter().max() {
    //     max = *m;
    // };

    let n_data = data.iter().map(|&i| i as f32 - BIAS).collect::<Vec<f32>>();

    let mut mins: Vec<f32> = vec![];
    let mut maxs: Vec<f32> = vec![];
    for i in n_data.windows(3) {
        if i[1] > i[0] && i[1] > i[2] && i[1] > THRESHOLD {
            // println!("local max extremum = {:?}", i[1]);
            maxs.push(i[1]);
        }
        if i[1] < i[0] && i[1] < i[2] && i[1].abs() > THRESHOLD {
            // println!("local min extremum = {:?}", i[1]);
            mins.push(i[1]);
        }
        // println!("windows iter = {:?}", i);
    }

    // println!("mins = {:?}", mins);
    // println!("mins len = {:?}", mins.len());
    // println!("maxs = {:?}", maxs);
    // println!("maxs len = {:?}", maxs.len());

    if maxs.len() > mins.len() {
        1
    } else {
        0
    }
}

// read pip data from the serial port
// install arduino ide + teense support to read from serial port on rpi
pub fn read_the_pip() -> Vec<i32> {
    let data = vec![1, 3, 2];
    // let s = SerialPortSettings {
    //     baud_rate: 9600,
    //     data_bits: DataBits::Eight,
    //     flow_control: FlowControl::None,
    //     parity: Parity::None,
    //     stop_bits: StopBits::One,
    //     timeout: Duration::from_secs(1),
    // };
    //
    // let mut data: Vec<i32> = vec![];
    // if let Ok(mut port) = serialport::open_with_settings("/dev/ttyACM0", &s) {
    //     let mut serial_buf: Vec<u8> = vec![0; 512];
    //     let start = SystemTime::now();
    //     loop {
    //         if let Ok(d) = start.elapsed() {
    //             if d > Duration::from_secs(delta) {
    //                 break;
    //             };
    //         }
    //         match port.read(serial_buf.as_mut_slice()) {
    //             Ok(t) => {
    //                 // println!("Pip val: {}", get_val(&serial_buf[..t]));
    //                 data.push(get_val(&serial_buf[..t]));
    //             }
    //             Err(e) => eprintln!("{:?}", e),
    //         }
    //     }
    // }

    data
}

pub fn get_val(buf: &[u8]) -> i32 {
    let mut output = 0;
    let serial_data = std::str::from_utf8(buf).unwrap();
    if let Some(i) = serial_data.find("PiPVal: ") {
        let s = &serial_data[i + 8..];
        if let Some(j) = s.find("\r") {
            let str_value = &s[..j];
            if let Ok(value) = str_value.parse::<i32>() {
                output = value;
            }
        }
    }

    output
}

pub fn pin_on(pin: u8) {
    println!("Pin number {} is on now.", pin);
    if let Ok(gpio) = Gpio::new() {
        if let Ok(pin) = gpio.get(pin) {
            let mut pin = pin.into_output();
            pin.set_high();
        }
    }
}

pub fn pin_off(pin: u8) {
    println!("Pin number {} is off now.", pin);
    if let Ok(gpio) = Gpio::new() {
        if let Ok(pin) = gpio.get(pin) {
            let mut pin = pin.into_output();
            pin.set_low();
        }
    }
}
