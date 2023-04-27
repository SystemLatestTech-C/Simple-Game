use std::net::TcpListener;
use std::io;
use std::thread;
use std::io::{Read, Write};

use crate::constants::*; // constants.rs 파일을 가져옵니다.

/**
 *   서버부분입니다.
 *   1. TcpListener를 통해서 플레이어1,2의 접속을 받습니다.
 *   2. 플레이어1,2의 소켓을 통해서 데이터를 주고받습니다. 이때 각 소켓은 별도의 스레드에서 동작합니다.
 *
 *   각 소켓의 버퍼에 담기는 공통 정보 : 각 플레이어의 라켓의 y좌표 , 크기는 4바이트입니다.
 *   플레이어1 : 4바이트의 라켓1의 y좌표에 추가로 4바이트의 공의 x좌표, 4바이트의 공의 y좌표를 담아서 플레이어2 소켓에 전송
 *   플레이어2 : 4바이트의 라켓2의 y좌표만을 플레이어1 소켓에 전송
 *
 *   각 플레이어는 전달받은 정보를 통해 라켓1, 라켓2, 공의 좌표를 업데이트해서 동일한 화면을 그리는 것이 가능합니다.
 */

pub fn listen_for_clients() -> io::Result<()> {

    let listener = TcpListener::bind(SERVER_ADDR)?;
    println!("Server listening on {}", SERVER_ADDR);

    let (mut client_1_socket, client_1_addr) = listener.accept()?;
    println!("Client 1 connected from {}", client_1_addr);

    let (mut client_2_socket, client_2_addr) = listener.accept()?;
    println!("Client 2 connected from {}", client_2_addr);

    let mut client_1_socket_clone = client_1_socket.try_clone()?;
    let mut client_2_socket_clone = client_2_socket.try_clone()?;

    let client_1_to_client_2 = thread::spawn(move || {
        let mut buffer = [0; 12];
        loop {
            match client_1_socket.read_exact(&mut buffer) {
                Ok(_) => {
                    if let Err(e) = client_2_socket.write_all(&buffer) {
                        println!("Error sending data to client 2: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    println!("Error reading data from client 1: {}", e);
                    break;
                }
            }
        }
    });

    let client_2_to_client_1 = thread::spawn(move || {
        let mut buffer = [0; 4];
        loop {
            match client_2_socket_clone.read_exact(&mut buffer) {
                Ok(_) => {
                    if let Err(e) = client_1_socket_clone.write_all(&buffer) {
                        println!("Error sending data to client 1: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    println!("Error reading data from client 2: {}", e);
                    break;
                }
            }
        }
    });

    client_1_to_client_2.join().unwrap();
    client_2_to_client_1.join().unwrap();

    Ok(())
}