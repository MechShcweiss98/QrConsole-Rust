use qrcode::QrCode;

pub fn generate_qr_code(data: &str) -> Result<QrCode, String> {
    QrCode::new(data).map_err(|e| format!("Error to generate QR code:{}", e))
}
