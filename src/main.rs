use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;
use std::str;
use std::thread;

fn main () 
{
  let mut handles = Vec::new ();
  for _ in 0..50 {
    handles.push (thread::spawn (|| {
      let time = 3;

      let mut stream = TcpStream::connect ("127.0.0.1:7878").unwrap ();

      let string = format!("test");
      stream.set_write_timeout(Some (Duration::from_secs (time))).unwrap();
      stream.write (string.as_bytes ()).unwrap ();
      stream.flush ().unwrap ();

      let mut buffer = [0; 256];
      stream.set_read_timeout (Some (Duration::from_secs(time))).unwrap();
      let len = stream.read (&mut buffer).unwrap ();

      println! ("Got {} with {} bytes", str::from_utf8 (&buffer).unwrap (), len);
    }));
  }

  for handle in handles.into_iter () {
    handle.join ().unwrap();
  }
}
