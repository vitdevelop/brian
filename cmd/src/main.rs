use color_eyre::Result;
use tokio::io::{stdout, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080".to_string();

    echo(&addr);

    // TODO: fix handling in docker container when need execute command after
    match signal(SignalKind::terminate())?.recv().await {
        _ => Ok(()),
    }
}

#[allow(unused)]
fn echo(addr: &String) {
    let addr = addr.clone();
    tokio::spawn(async move {
        let listener = TcpListener::bind(&addr).await.unwrap();
        println!("Listening on: {}", addr);

        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                let mut buf = vec![0; 1024];

                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                stdout()
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to stdout");

                stdout().flush().await.expect("failed to flush to stdout");

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");

                socket.flush().await.expect("failed to flush socket");
            });
        }
    });
}

#[allow(unused)]
fn stdout_print(addr: &String) {
    let addr = addr.clone();
    tokio::spawn(async move {
        let listener = TcpListener::bind(&addr).await.unwrap();
        println!("Listening on: {}", addr);

        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                let mut buf = vec![0; 1024];

                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                stdout()
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to stdout");

                stdout().flush().await.expect("failed to flush to stdout");
            });
        }
    });
}
