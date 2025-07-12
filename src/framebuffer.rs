use raylib::prelude::*;

pub struct Framebuffer {
    width:  u32,
    height: u32,
    color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    /// Crea un nuevo framebuffer con color de fondo
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        // convertimos a i32
        let w = width  as i32;
        let h = height as i32;
        let color_buffer = Image::gen_image_color(w, h, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    /// Rellena todo el buffer con el color de fondo
    pub fn clear(&mut self) {
        // de nuevo, convertimos antes de llamar
        let w = self.width  as i32;
        let h = self.height as i32;
        self.color_buffer = Image::gen_image_color(w, h, self.background_color);
    }

    /// Pone un píxel en (x, y) con el color actual
    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

    /// Cambia el color de fondo para futuros clear()
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    /// Cambia el color con que dibujas líneas, polígonos, etc.
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    /// Guarda la imagen en disco
    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }
}
