use qrcode::render::unicode;
use qrcode::{EcLevel, QrCode};

pub fn encode_qr(data: &str) {
    match QrCode::with_error_correction_level(data, EcLevel::H) {
        Ok(code) => {
            let image = code.render::<unicode::Dense1x2>().quiet_zone(true).build();
            println!("{}", image);
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
