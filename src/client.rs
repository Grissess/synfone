use std::net;

fn main(){
	let args = std::env::args();
	let mut numArg = 0;
	for i in args{
		numArg+=1;
	}
	numArg = numArg - 1;
//	let mut name = "\0";
	if numArg == 1{
//		name = args[1];
	}
    let buwah = [0;32];
	let mut socket = std::net::UdpSocket::bind("0.0.0.0:13676").unwrap();
	let mut buf = [0;36];
	loop{
		let (mut byteNum, mut src_addr) = socket.recv_from(&mut buf).unwrap();
		for i in 0..36{
			print!("{}",buf[i]);
		}
        println!(" ");
        let mut fByte = 0;
        fByte = fromPacket(0,4,&buf,10);  //fromBinary(0,4,&buf);
        println!(" first byte {}",fByte);
		if fByte == 2{break;
		} else if fByte == 3 {
			//play note here
			let mut dur = 0;
			dur += fromPacket(4,8,&buf,10) * 1000000;
			dur += fromPacket(8,12,&buf,10);
			dur *= 1000;// puts into nanp seconds
		} else if fByte == 1{
            println!("mirroring packet to {}",src_addr);
            socket.send_to(&buf,&src_addr);
        } else if fByte == 4 {
            println!("sending caps to source");
            let mut reBuf = [0;36];
            reBuf = mkCapsPacket(1,0);
            socket.send_to(&reBuf,&src_addr);
        }
		
		
		buf = [0;36];
		println!(" recieved packet from {}",src_addr);
	}

}

fn fromPacket(i: usize,j: usize,buf: &[u8],base: u32)->u32 { 
        let mut z: u32 = 1;
        let mut ret: u32 = 0;
        for q in i..j{
            z *= base;
        }
        z /= base;
        println!("Z {}", z);
        for q in i..j{
            print!("{}",buf[q]);
            ret += z * (buf[q] as u32);
            z/=base;
        }
        ret
}

fn mkCapsPacket(ports: u8,id: u8) ->[u8;36] {
    let mut ret = [0;36];
    ret[3] = 4;
    ret[7] = ports;
    ret[8] = 84;
    ret[9] = 79;
    ret[10] = 78;
    ret[11] = 69;
    ret
}

