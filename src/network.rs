use local_ip_address::local_ip;
use qr2term::print_qr;

pub fn get_local_ip_and_print_qr(port: u16) -> Result<String, Box<dyn std::error::Error>> {
    let ip = local_ip()?;
    let url = format!("http://{}:{}", ip, port);

    println!("\n┌───────────────────────┐");
    println!("│        WIRECLIP       │");
    println!("└───────────────────────┘");
    println!("Server listening at: {}\n", url);
    println!("Scan this QR code with your phone to connect:");

    print_qr(&url)?;
    println!("\n[Press Ctrl+C to stop sharing]\n");

    Ok(url)
}
