//! In-memory Buffer
//! 컴퓨터 메모리에 저장된 데이터를 임시로 보관하는 구조
//! 하드 드라이브나 네트워크를 통한 입출력보다 훨씬 빠르게 데이터에 접근할 수 있게 해줌
//! 주로 I/O 작업에서 사용되며, 한 번에 많은 양의 데이터를 읽고 쓸 수 함.
//! 
//! Cursor Struct in Rust
//! std::io::Cursor 구조체는 이러한 in-memory 버퍼에 Seek와 Read 및 Write 트레잇(trait)을 제공 
//! 마치 파일이나 네트워크 스트림과 같은 다른 데이터 소스를 다루는 것처럼 
//! 이 in-memory 버퍼를 읽고 쓸 수 있게 만들어 줌.
//! 
//! Cursor Sturct 는 언제 사용하는가?
//! 
//! 1. 직렬화 및 역직렬화
//!    데이터를 바이트 스트림으로 변환하거나 (직렬화), 바이트 스트림을 데이터로 복원할 때 (역직렬화)
//!    Cursor를 사용하면 편리합니다. 이렇게 하면 파일 시스템이나 네트워크 소켓 대신 메모리 버퍼로 
//!    직렬화 및 역직렬화를 수행할 수 있습니다. 예를 들어, 바이너리 데이터를 효율적으로 처리하거나 
//!    프로토콜을 구현할 때 유용합니다.
//! 
//! 2. 테스트 환경 구축
//!    Cursor를 사용하면 파일이나 네트워크 소켓과 같이 외부의 실제 자원을 사용하는 코드를 테스트할 때,
//!    실제 자원 대신 메모리 버퍼를 사용하여 테스트 환경을 구성할 수 있습니다. 이를 통해 테스트의 
//!    실행 속도를 높일 수 있고, 테스트 환경을 좀 더 통제할 수 있습니다.
//! 
//! 3. 입출력 스트림 처리 
//!    어떤 경우에는 파일이나 네트워크 소켓 등의 입출력 스트림을 직접 다루는 대신, 
//!    메모리 버퍼를 사용하여 스트림의 일부분을 미리 읽거나 쓸 수 있게 하여, 
//!    입출력 작업의 성능을 개선하거나 복잡성을 줄일 수 있습니다. 이 때, Cursor를 사용하면 
//!    입출력 스트림을 쉽게 다룰 수 있습니다.

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestStruct {
    a: u32,
    b: u64,
}


fn main() {

}

#[test]
fn test_cursor() {
    use std::io::Cursor;
    use bincode::{serialize_into, deserialize_from};
    
    let test_struct = TestStruct { a: 5, b: 10 };
    let mut buffer = Cursor::new(Vec::new());
    serialize_into(&mut buffer, &test_struct).unwrap();

    buffer.set_position(0); // Cursor의 위치를 처음으로 되돌립니다.

    let decoded: TestStruct = deserialize_from(&mut buffer).unwrap();
    assert_eq!(test_struct, decoded);
}




