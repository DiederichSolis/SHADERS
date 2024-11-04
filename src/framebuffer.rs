/// Estructura que representa un framebuffer para renderizar gráficos.
pub struct Framebuffer {
    /// Ancho del framebuffer.
    pub width: usize,
    /// Alto del framebuffer.
    pub height: usize,
    /// Buffer que almacena el color de cada píxel.
    pub buffer: Vec<u32>,
    /// Z-buffer que almacena la profundidad de cada píxel para el manejo de la superposición.
    pub zbuffer: Vec<f32>,
    /// Color de fondo del framebuffer.
    background_color: u32,
    /// Color actual que se usará para dibujar.
    current_color: u32,
}

impl Framebuffer {
    /// Crea un nuevo framebuffer con el ancho y alto especificados.
    ///
    /// # Parámetros
    /// - `width`: El ancho del framebuffer.
    /// - `height`: El alto del framebuffer.
    ///
    /// # Retorna
    /// Una instancia de `Framebuffer`.
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            // Inicializa el buffer de píxeles con color negro (0x000000).
            buffer: vec![0; width * height],
            // Inicializa el z-buffer con infinito, indicando que no hay píxeles dibujados.
            zbuffer: vec![f32::INFINITY; width * height],
            background_color: 0x000000, // Color de fondo inicial.
            current_color: 0xFFFFFF      // Color actual inicial (blanco).
        }
    }

    /// Limpia el framebuffer y el z-buffer, restableciendo los colores de fondo.
    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color; // Establece cada píxel al color de fondo.
        }
        for depth in self.zbuffer.iter_mut() {
            *depth = f32::INFINITY; // Restablece la profundidad a infinito.
        }
    }

    /// Dibuja un punto en el framebuffer en la posición (x, y) con la profundidad especificada.
    ///
    /// # Parámetros
    /// - `x`: Coordenada x del punto a dibujar.
    /// - `y`: Coordenada y del punto a dibujar.
    /// - `depth`: La profundidad del punto; solo se dibuja si es menor que la profundidad actual en el z-buffer.
    pub fn point(&mut self, x: usize, y: usize, depth: f32) {
        if x < self.width && y < self.height { // Verifica que las coordenadas estén dentro del framebuffer.
            let index = y * self.width + x; // Calcula el índice del píxel en el buffer.

            // Si la nueva profundidad es menor que la profundidad almacenada, se actualiza el píxel.
            if self.zbuffer[index] > depth {
                self.buffer[index] = self.current_color; // Establece el color actual en el buffer.
                self.zbuffer[index] = depth; // Actualiza el z-buffer con la nueva profundidad.
            }
        }
    }

    /// Establece el color de fondo del framebuffer.
    ///
    /// # Parámetros
    /// - `color`: El color que se usará como fondo, representado como un valor u32.
    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color; // Asigna el nuevo color de fondo.
    }

    /// Establece el color actual que se usará para dibujar.
    ///
    /// # Parámetros
    /// - `color`: El color que se usará para dibujar, representado como un valor u32.
    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color; // Asigna el nuevo color actual.
    }
}
