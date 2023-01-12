use bitbuffer::{BitReadBuffer, BitReadStream, BitWriteStream, LittleEndian};
use std::net::UdpSocket;

enum TCPState {
    Close,
    Listen,
    SynSent,
    SynRecv,
}

struct TCPFlags {
    urg: bool,
    ack: bool,
    psh: bool,
    rst: bool,
    syn: bool,
    fin: bool,
}

struct TCPHeader {
    src_port_addr: u16,
    dst_port_addr: u16,
    seq_num: u32,
    ack_num: u32,
    offset: u8,
    flags: TCPFlags,
    window_size: u16,
    checksum: u16,
    urgent_point: u16,
}

impl TCPHeader {
    pub fn new() -> TCPHeader {
        let flags = TCPFlags {
            urg: false,
            ack: false,
            psh: false,
            rst: false,
            syn: false,
            fin: false,
        };
        TCPHeader {
            src_port_addr: 0,
            dst_port_addr: 0,
            seq_num: 0,
            ack_num: 0,
            offset: 0,
            flags,
            window_size: 0,
            checksum: 0,
            urgent_point: 0,
        }
    }
}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");

    const DEFAULT_MSS: usize = 536;
    let mut buf = [0; DEFAULT_MSS];

    loop {
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");

        let request_buf = &mut buf[..number_of_bytes];
        let mut response_buf = [0; DEFAULT_MSS];

        handle_request(request_buf);

        socket
            .send_to(request_buf, &src_addr)
            .expect("Error on Send");
    }
}

fn handle_request(buffer: &mut [u8]) {
    let header = &mut buffer[..20];
    let request_header = parse_header(header);

    let mut response_header = TCPHeader::new();
    if request_header.flags.syn {
        response_header.flags.syn = true;
        response_header.flags.ack = true;
        response_header.seq_num = request_header.seq_num;
        response_header.ack_num = request_header.seq_num + 1;
    } else if request_header.flags.ack {
    }
}

fn parse_header(header: &mut [u8]) -> TCPHeader {
    let buff = BitReadBuffer::new(&header, LittleEndian);
    let mut stream = BitReadStream::new(buff);

    let src_port_addr: u16 = stream.read().unwrap();
    let dst_port_addr: u16 = stream.read().unwrap();
    let seq_num: u32 = stream.read().unwrap();
    let ack_num: u32 = stream.read().unwrap();
    let offset: u8 = stream.read_int(4).unwrap();
    let _resv = stream.read_bits(6).unwrap();
    let mut flags = stream.read_bits(6).unwrap();
    let window_size: u16 = stream.read().unwrap();
    let checksum: u16 = stream.read().unwrap();
    let urgent_point: u16 = stream.read().unwrap();

    let urg: bool = flags.read_bool().unwrap();
    let ack: bool = flags.read_bool().unwrap();
    let psh: bool = flags.read_bool().unwrap();
    let rst: bool = flags.read_bool().unwrap();
    let syn: bool = flags.read_bool().unwrap();
    let fin: bool = flags.read_bool().unwrap();

    let tcp_flags = TCPFlags {
        urg,
        ack,
        psh,
        rst,
        syn,
        fin,
    };
    TCPHeader {
        src_port_addr,
        dst_port_addr,
        seq_num,
        ack_num,
        offset,
        flags: tcp_flags,
        window_size,
        checksum,
        urgent_point,
    }
}

fn serialize_header(header: &mut TCPHeader) -> Vec<u8> {
    let mut write_bytes = vec![];
    let mut write_stream = BitWriteStream::new(&mut write_bytes, LittleEndian);

    write_stream.write(&header.src_port_addr).unwrap();
    write_stream.write(&header.dst_port_addr).unwrap();
    write_stream.write(&header.seq_num).unwrap();
    write_stream.write(&header.ack_num).unwrap();
    write_stream.write_int(header.offset, 4).unwrap();
    write_stream.write_int(0, 6).unwrap();
    write_stream.write_bool(header.flags.urg).unwrap();
    write_stream.write_bool(header.flags.ack).unwrap();
    write_stream.write_bool(header.flags.psh).unwrap();
    write_stream.write_bool(header.flags.rst).unwrap();
    write_stream.write_bool(header.flags.syn).unwrap();
    write_stream.write_bool(header.flags.fin).unwrap();
    write_stream.write(&header.window_size).unwrap();
    write_stream.write(&header.checksum).unwrap();
    write_stream.write(&header.urgent_point).unwrap();

    write_bytes
}
