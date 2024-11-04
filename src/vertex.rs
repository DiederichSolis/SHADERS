use nalgebra_glm::{Vec2, Vec3};
use crate::color::Color;

/// Estructura que representa un vértice en un espacio 3D.
#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vec3,            // Posición del vértice en coordenadas 3D
    pub normal: Vec3,              // Normal del vértice para cálculos de iluminación
    pub tex_coords: Vec2,          // Coordenadas de textura para mapeo de texturas
    pub color: Color,               // Color del vértice
    pub transformed_position: Vec3, // Posición transformada del vértice
    pub transformed_normal: Vec3,   // Normal transformada del vértice
    pub elevation: f32,             // Elevación del vértice (nueva propiedad)
}

impl Vertex {
    /// Crea un nuevo vértice con la posición, normal y coordenadas de textura especificadas.
    /// Inicializa el color a negro y las posiciones transformadas a las originales.
    pub fn new(position: Vec3, normal: Vec3, tex_coords: Vec2, elevation: f32) -> Self {
        Vertex {
            position,
            normal,
            tex_coords,
            color: Color::black(), // Color predeterminado
            transformed_position: position, // Posición transformada inicializada a la original
            transformed_normal: normal,      // Normal transformada inicializada a la original
            elevation,                       // Inicializa la elevación
        }
    }

    /// Crea un nuevo vértice con la posición y color especificados.
    /// Inicializa la normal y las coordenadas de textura a cero, y las posiciones transformadas a cero.
    pub fn new_with_color(position: Vec3, color: Color) -> Self {
        Vertex {
            position,
            normal: Vec3::new(0.0, 0.0, 0.0), // Normal inicializada a cero
            tex_coords: Vec2::new(0.0, 0.0),  // Coordenadas de textura inicializadas a cero
            color,
            transformed_position: Vec3::new(0.0, 0.0, 0.0), // Posición transformada inicializada a cero
            transformed_normal: Vec3::new(0.0, 0.0, 0.0),   // Normal transformada inicializada a cero
            elevation: 0.0, // Inicializa la elevación a cero
        }
    }

    /// Establece la posición y normal transformadas del vértice.
    pub fn set_transformed(&mut self, position: Vec3, normal: Vec3) {
        self.transformed_position = position; // Actualiza la posición transformada
        self.transformed_normal = normal;     // Actualiza la normal transformada
    }

    /// Actualiza el color del vértice en función de su elevación.
    /// - Color de océano (azul) si la elevación es menor que 0.
    /// - Color de tierra (verde) si la elevación es entre 0 y 0.5.
    /// - Color de montaña (marrón) si la elevación es mayor o igual a 0.5.
    pub fn update_color_based_on_elevation(&mut self) {
        if self.elevation < 0.0 {
            self.color = Color::new(0, 105, 148); // Color de océano
        } else if self.elevation < 0.5 {
            self.color = Color::new(34, 139, 34); // Color de tierra
        } else {
            self.color = Color::new(139, 69, 19); // Color de montaña
        }
    }
}

/// Implementación de Default para la estructura Vertex.
impl Default for Vertex {
    /// Crea un vértice predeterminado con valores iniciales.
    fn default() -> Self {
        Vertex {
            position: Vec3::new(0.0, 0.0, 0.0), // Posición inicializada a (0, 0, 0)
            normal: Vec3::new(0.0, 1.0, 0.0),   // Normal inicializada hacia arriba
            tex_coords: Vec2::new(0.0, 0.0),    // Coordenadas de textura inicializadas a (0, 0)
            color: Color::black(),               // Color predeterminado a negro
            transformed_position: Vec3::new(0.0, 0.0, 0.0), // Posición transformada inicializada a cero
            transformed_normal: Vec3::new(0.0, 1.0, 0.0),   // Normal transformada inicializada hacia arriba
            elevation: 0.0,                      // Inicializa la elevación a cero
        }
    }
}
