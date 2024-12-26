use crate::message::{client_message, server_message, ClientMessage, ServerMessage};
use log::{error, info, warn};
use prost::Message;
use std::{
    io::{self, ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Client { stream }
    }

    pub fn handle(&mut self) -> io::Result<()> {
        loop {
            let mut buffer = vec![0; 1024];
            match self.stream.read(&mut buffer) {
                Ok(0) => {
                    info!("Client disconnected.");
                    return Ok(());
                }
                Ok(bytes_read) => {
                    match ClientMessage::decode(&buffer[..bytes_read]) {
                        Ok(client_message) => match client_message.message {
                            Some(client_message::Message::EchoMessage(echo)) => {
                                info!("Received EchoMessage: {}", echo.content);
                                let response = ServerMessage {
                                    message: Some(server_message::Message::EchoMessage(echo)),
                                };
                                self.send_response(response)?;
                            }
                            Some(client_message::Message::AddRequest(add_request)) => {
                                info!(
                                    "Received AddRequest: a = {}, b = {}",
                                    add_request.a, add_request.b
                                );
                                // Calculate the sum and prepare the response
                                let add_response = crate::message::AddResponse {
                                    result: add_request.a + add_request.b,
                                };
                                let server_message = crate::message::ServerMessage {
                                    message: Some(server_message::Message::AddResponse(
                                        add_response,
                                    )),
                                };
                                self.send_response(server_message)?;
                            }
                            _ => {
                                warn!("Unsupported message type received");
                            }
                        },
                        Err(err) => {
                            error!("Failed to decode ClientMessage: {}", err);
                        }
                    }
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    // No data available, but connection is still open
                    thread::sleep(Duration::from_millis(10));
                }
                Err(e) => {
                    error!("Error reading from stream: {}", e);
                    return Err(e);
                }
            }
        }
    }

    fn send_response(&mut self, response: ServerMessage) -> io::Result<()> {
        let response_payload = response.encode_to_vec();
        self.stream.write_all(&response_payload)?;
        self.stream.flush()?;
        Ok(())
    }
}

pub struct Server {
    listener: TcpListener,
    is_running: Arc<AtomicBool>,
    clients: Arc<Mutex<Vec<TcpStream>>>,
}

impl Server {
    pub fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        let is_running = Arc::new(AtomicBool::new(false));
        let clients = Arc::new(Mutex::new(Vec::new()));
        Ok(Server {
            listener,
            is_running,
            clients,
        })
    }

    pub fn run(&self) -> io::Result<()> {
        self.is_running.store(true, Ordering::SeqCst);
        info!("Server is running on {}", self.listener.local_addr()?);

        while self.is_running.load(Ordering::SeqCst) {
            match self.listener.accept() {
                Ok((stream, addr)) => {
                    info!("New client connected: {}", addr);
                    let is_running = Arc::clone(&self.is_running);
                    let clients = Arc::clone(&self.clients);

                    {
                        let mut clients = clients.lock().unwrap();
                        clients.push(stream.try_clone()?);
                    }

                    thread::spawn(move || {
                        let mut client = Client::new(stream);
                        while is_running.load(Ordering::SeqCst) {
                            if let Err(e) = client.handle() {
                                error!("Error handling client: {}", e);
                                break;
                            }
                        }
                        // Remove the client from the list when disconnected
                        let mut clients = clients.lock().unwrap();
                        clients.retain(|c| {
                            c.peer_addr().unwrap() != client.stream.peer_addr().unwrap()
                        });
                    });
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    // No new connections, sleep for a short time
                    thread::sleep(Duration::from_millis(10));
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }
        }

        info!("Server stopped.");
        Ok(())
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }
}
