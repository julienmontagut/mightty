use std::sync::{Arc, Mutex};
use vte::{Params, Parser, Perform};

#[derive(Debug)]
pub enum TerminalEvent {
    Print(char),
    Execute(u8),
    Hook(Vec<i16>),
    Put(u8),
    Unhook,
    OscDispatch(Vec<Vec<u8>>),
    CsiDispatch(Vec<i16>, Vec<u8>, bool, char),
    EscDispatch(Vec<u8>, bool, u8),
}

pub struct VteParser {
    parser: Parser,
    events: Arc<Mutex<Vec<TerminalEvent>>>,
}

impl Default for VteParser {
    fn default() -> Self {
        Self::new()
    }
}

impl VteParser {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn parse(&mut self, data: &[u8]) -> Vec<TerminalEvent> {
        let mut performer = TerminalPerformer {
            events: self.events.clone(),
        };

        self.parser.advance(&mut performer, data);

        let mut events = self.events.lock().unwrap();
        let result = events.drain(..).collect();
        result
    }
}

struct TerminalPerformer {
    events: Arc<Mutex<Vec<TerminalEvent>>>,
}

impl Perform for TerminalPerformer {
    fn print(&mut self, c: char) {
        self.events.lock().unwrap().push(TerminalEvent::Print(c));
    }

    fn execute(&mut self, byte: u8) {
        self.events
            .lock()
            .unwrap()
            .push(TerminalEvent::Execute(byte));
    }

    fn hook(&mut self, params: &Params, _intermediates: &[u8], _ignore: bool, _c: char) {
        let params_vec: Vec<i16> = params
            .iter()
            .flat_map(|p| p.iter().map(|&x| x as i16))
            .collect();
        self.events
            .lock()
            .unwrap()
            .push(TerminalEvent::Hook(params_vec));
    }

    fn put(&mut self, byte: u8) {
        self.events.lock().unwrap().push(TerminalEvent::Put(byte));
    }

    fn unhook(&mut self) {
        self.events.lock().unwrap().push(TerminalEvent::Unhook);
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], bell_terminated: bool) {
        let _ = bell_terminated;
        let params_vec = params.iter().map(|p| p.to_vec()).collect();
        self.events
            .lock()
            .unwrap()
            .push(TerminalEvent::OscDispatch(params_vec));
    }

    fn csi_dispatch(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: char) {
        let params_vec: Vec<i16> = params
            .iter()
            .flat_map(|p| p.iter().map(|&x| x as i16))
            .collect();
        let intermediates_vec = intermediates.to_vec();
        self.events.lock().unwrap().push(TerminalEvent::CsiDispatch(
            params_vec,
            intermediates_vec,
            ignore,
            c,
        ));
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, byte: u8) {
        let intermediates_vec = intermediates.to_vec();
        self.events.lock().unwrap().push(TerminalEvent::EscDispatch(
            intermediates_vec,
            ignore,
            byte,
        ));
    }
}
