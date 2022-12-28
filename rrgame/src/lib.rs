/*
Client server code for a multiplayer roulette game.
First prototype will just echo messages back and forth.
*/
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

const SERVER: &str = "127.0.0.1:8080";

/*server function accepts a string message and sends it to client
logs the message to the console as well
*/
pub async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(SERVER).await.unwrap();
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                let n = socket.read(&mut buf).await.unwrap();
                if n == 0 {
                    return;
                }
                let message = String::from_utf8_lossy(&buf[..n]);
                println!("Server received: {}", message);
                //send the message back to the client with hostname/port
                let message = format!("{}:{}", message, SERVER);
                socket.write_all(message.as_bytes()).await.unwrap();
            }
        });
    }
}

//create a unique id for each client
fn get_id() -> String {
    let uuid = uuid::Uuid::new_v4();
    uuid.to_string()
}

//client function accepts string message and sends it to server
#[allow(clippy::unused_io_amount)]
pub async fn client(message: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(SERVER).await?;
    //send the message to the server with unique id
    let message = format!("{}:{}", get_id(), message);
    stream.write_all(message.as_bytes()).await?;
    //read the response
    let mut buf = vec![0; 1024];
    stream.read(&mut buf).await.unwrap();
    let response = String::from_utf8_lossy(&buf);
    println!("Client received: {}", response);
    Ok(())
}
