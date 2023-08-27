use std::io::{stdin, stdout, Write};
use midir::{MidiInput, Ignore, MidiInputConnection};
use midi_msg::*;
use std::error::Error;


//ch1_note : &'static mut i32, ch1_velocity: &'static mut i32

pub fn listen<F>( mut func: F) ->  Result<MidiInputConnection<()>, Box<dyn Error>>
    where F : FnMut(usize, u32, u32) + Send + 'static // index, note and velocity
{
    let mut midi_in;
    let in_port;

    let mut ctx = ReceiverContext::new();

    midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();

    in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!("Choosing the only available input port: {}", midi_in.port_name(&in_ports[0]).unwrap());
            &in_ports[0]
        },
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports.get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };

    println!("\nOpening connection");
    let _in_port_name = midi_in.port_name(in_port)?;

    let _conn_in = midi_in.connect(in_port, "midir-read-input", move |stamp, midi_bytes, _| {
        //println!("{}: {:?} (len = {})", stamp, message, message.len());
        let (msg, _len) = MidiMsg::from_midi_with_context(&midi_bytes, &mut ctx).expect("Not an error");
        // Print everything but spammy clock messages.
        if let MidiMsg::SystemRealTime{ msg: SystemRealTimeMsg::TimingClock } = msg {
            // no-op
        } else {
            println!("{}: {:?}", stamp, msg);

            match msg {
                MidiMsg::ChannelVoice {channel, msg} => {
                    match msg {
                        ChannelVoiceMsg::NoteOn {note, velocity} => {
                            func(channel as usize, note as u32, velocity as u32);
                        }
                        ChannelVoiceMsg::NoteOff {note: _, velocity: _} => {
                            func(channel as usize, 0, 0);
                        }
                        _ => println!("nothing")
                    }
                }
                _ => println!("nothing")
            }
        }
    },())?;
    return Ok(_conn_in);
}