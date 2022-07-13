use actix::prelude::*;
use serialport as sp;
use std::time::Duration;

const ARDUINO_VID: u16 = 0x2341;
const ARDUINO_PID: u16 = 0x8037; 

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub msg: String,
}


pub struct SerialActor {
    // Chucks data onto the heap as SerialPort has an undefined
    // size at compile time
    port: Box<dyn sp::SerialPort>,
}
impl Actor for SerialActor {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Self::Context){}
}
impl Default for SerialActor {
    fn default() -> SerialActor {
        // CBA to find the port for both windows & linux
        // Hence I connect to arduino via PID and VID
        let ports = sp::available_ports().unwrap();
        let arduino_port = ports.iter()
        .find_map(|p| match &p.port_type {
            sp::SerialPortType::UsbPort(u) 
            if u.vid == ARDUINO_VID && u.pid == ARDUINO_PID => Some(&p.port_name),
            _ => None,
        });
        SerialActor {
            port: sp::new(arduino_port.unwrap(), 9600).timeout(Duration::from_millis(10))
            .open().expect("Failed to open port"),
        }
    }
}

impl Handler<ClientActorMessage> for SerialActor{
    type Result = ();
    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Self::Context) -> Self::Result {
        // Arduino hex no likey # so go bye bye
        match self.port.write(msg.msg[1..].to_string().as_bytes()) {
            Ok(_) => {
                println!("{}", msg.msg);
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }
}

