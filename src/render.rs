use qrcode::QrCode;

pub trait QrRenderer {
    fn render(&self, code: &QrCode) -> String;
}

pub struct TextQrRenderer;

impl QrRenderer for TextQrRenderer {
    fn render(&self, code: &QrCode) -> String {
        code.render::<char>()
            .quiet_zone(false)
            .module_dimensions(2, 1)
            .build()
    }
}
