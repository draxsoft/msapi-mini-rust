use std::io::Write;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let script = "loadstring(game:HttpGet(\"https://raw.githubusercontent.com/EdgeIY/infiniteyield/master/source\"))()";
    let len = (script.len() + 1) as u32; // +1 for the null terminator
    let mut header = [0u8; 16];
    header[8..12].copy_from_slice(&len.to_le_bytes());
    let mut stream = TcpStream::connect("127.0.0.1:5553")?;
    stream.write_all(&header)?;
    stream.write_all(script.as_bytes())?;
    stream.write_all(&[0])?;
    println!("F12 in Roblox to see script activity.");
    Ok(())
}