use anyhow::Result;
use portable_pty::{Child, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

pub struct Pty {
    master_pty: Box<dyn MasterPty + Send>,
    child_process: Box<dyn Child + Send + Sync>,
    input_tx: Sender<String>,
    output_rx: Receiver<Vec<u8>>,
    process_alive: bool,
}

impl Pty {
    pub fn new(size: PtySize) -> Result<Self> {
        let native_pty = portable_pty::native_pty_system();
        let pty_pair = native_pty.openpty(size)?;

        let command = CommandBuilder::new("bash");
        let child_process = pty_pair.slave.spawn_command(command)?;

        let (input_tx, input_rx) = mpsc::channel::<String>();
        let (output_tx, output_rx) = mpsc::channel::<Vec<u8>>();

        // Clone the master for the input thread
        let master_writer = pty_pair.master.take_writer()?;
        let master_reader = pty_pair.master.try_clone_reader()?;

        // Input handling thread
        let input_master = master_writer;
        thread::spawn(move || {
            let mut writer = input_master;
            for input in input_rx {
                if let Err(e) = writer.write_all(input.as_bytes()) {
                    eprintln!("PTY input thread: Failed to write to PTY: {}", e);
                    break;
                }
                if let Err(e) = writer.flush() {
                    eprintln!("PTY input thread: Failed to flush PTY: {}", e);
                    break;
                }
            }
            eprintln!("PTY input thread: Exiting");
        });

        // Output handling thread
        let output_sender = output_tx;
        thread::spawn(move || {
            let mut reader = master_reader;
            let mut buffer = [0u8; 1024];

            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => {
                        eprintln!("PTY output thread: EOF received, process likely exited");
                        break;
                    }
                    Ok(n) => {
                        let data = buffer[..n].to_vec();
                        if output_sender.send(data).is_err() {
                            eprintln!("PTY output thread: Failed to send data to main thread");
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("PTY output thread: Failed to read from PTY: {}", e);
                        break;
                    }
                }
            }
            eprintln!("PTY output thread: Exiting");
        });

        Ok(Self {
            master_pty: pty_pair.master,
            child_process,
            input_tx,
            output_rx,
            process_alive: true,
        })
    }

    pub fn write_input(&self, input: &str) -> Result<()> {
        self.input_tx
            .send(input.to_string())
            .map_err(|e| anyhow::anyhow!("Failed to send input: {}", e))?;
        Ok(())
    }

    pub fn try_read_output(&mut self) -> Option<Vec<u8>> {
        self.output_rx.try_recv().ok()
    }

    pub fn resize(&self, size: PtySize) -> Result<()> {
        self.master_pty
            .resize(size)
            .map_err(|e| anyhow::anyhow!("Failed to resize PTY: {}", e))
    }

    pub fn is_alive(&mut self) -> bool {
        if !self.process_alive {
            return false;
        }

        match self.child_process.try_wait() {
            Ok(Some(exit_status)) => {
                eprintln!("Shell process exited with status: {:?}", exit_status);
                self.process_alive = false;
                false
            }
            Ok(None) => true, // Process is still running
            Err(e) => {
                eprintln!("Error checking process status: {}", e);
                self.process_alive = false;
                false
            }
        }
    }
}
