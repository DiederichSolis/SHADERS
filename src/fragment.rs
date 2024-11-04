// Importa Vec2 y Vec3 de la biblioteca nalgebra_glm para trabajar con vectores 2D y 3D.
use nalgebra_glm::{Vec2, Vec3};
// Importa la estructura Color de tu módulo de color.
use crate::color::Color;

/// Estructura que representa un fragmento de una imagen 3D.
/// Contiene información sobre la posición, color, profundidad, normal,
/// intensidad y posición del vértice correspondiente.
pub struct Fragment {
    /// Posición del fragmento en el espacio 2D (pantalla).
    pub position: Vec2,
    /// Color del fragmento, representado por la estructura Color.
    pub color: Color,
    /// Profundidad del fragmento en el espacio 3D.
    pub depth: f32,
    /// Normal del fragmento, que puede ser utilizada para iluminación.
    pub normal: Vec3,
    /// Intensidad de la luz en el fragmento.
    pub intensity: f32,
    /// Posición del vértice en el espacio 3D.
    pub vertex_position: Vec3,
}

impl Fragment {
    /// Crea un nuevo fragmento con la posición, color, profundidad, normal,
    /// intensidad y posición de vértice especificados.
    ///
    /// # Parámetros
    /// - `x`: Coordenada X de la posición del fragmento.
    /// - `y`: Coordenada Y de la posición del fragmento.
    /// - `color`: Color del fragmento.
    /// - `depth`: Profundidad del fragmento en el espacio 3D.
    /// - `normal`: Normal del fragmento.
    /// - `intensity`: Intensidad de la luz en el fragmento.
    /// - `vertex_position`: Posición del vértice correspondiente en el espacio 3D.
    ///
    /// # Retorna
    /// Una nueva instancia de `Fragment`.
    pub fn new(
        x: f32,
        y: f32,
        color: Color,
        depth: f32,
        normal: Vec3,
        intensity: f32,
        vertex_position: Vec3,
    ) -> Self {
        Fragment {
            position: Vec2::new(x, y), // Inicializa la posición 2D con coordenadas x e y.
            color,                      // Asigna el color del fragmento.
            depth,                      // Asigna la profundidad del fragmento.
            normal,                     // Asigna la normal del fragmento.
            intensity,                  // Asigna la intensidad de la luz.
            vertex_position,            // Asigna la posición del vértice en 3D.
        }
    }
}
