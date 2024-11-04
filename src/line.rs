use crate::fragment::Fragment; // Importa la estructura Fragment del módulo fragment.
use crate::vertex::Vertex; // Importa la estructura Vertex del módulo vertex.
use crate::color::Color; // Importa la estructura Color del módulo color.

/// Dibuja una línea entre dos vértices `a` y `b` utilizando el algoritmo de Bresenham.
/// 
/// # Parámetros
/// 
/// - `a`: Una referencia al primer vértice (punto de inicio).
/// - `b`: Una referencia al segundo vértice (punto de fin).
/// 
/// # Retorna
/// 
/// Una vector de fragmentos que representan los píxeles de la línea dibujada.
pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new(); // Inicializa un vector para almacenar los fragmentos.

    let start = a.transformed_position; // Obtiene la posición transformada del vértice de inicio.
    let end = b.transformed_position; // Obtiene la posición transformada del vértice de fin.

    let mut x0 = start.x as i32; // Coordenada x del punto de inicio como un entero.
    let mut y0 = start.y as i32; // Coordenada y del punto de inicio como un entero.
    let x1 = end.x as i32; // Coordenada x del punto de fin como un entero.
    let y1 = end.y as i32; // Coordenada y del punto de fin como un entero.

    let dx = (x1 - x0).abs(); // Diferencia absoluta en x.
    let dy = (y1 - y0).abs(); // Diferencia absoluta en y.

    let sx = if x0 < x1 { 1 } else { -1 }; // Dirección del movimiento en x.
    let sy = if y0 < y1 { 1 } else { -1 }; // Dirección del movimiento en y.

    // Inicializa el error en función de la diferencia en x o y.
    let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

    loop {
        // Calcula la coordenada z interpolada entre los vértices.
        let z = start.z + (end.z - start.z) * (x0 - start.x as i32) as f32 / (end.x - start.x) as f32;
        
        // Crea un nuevo fragmento con la posición actual y el color blanco, y lo añade al vector.
        fragments.push(Fragment::new(x0 as f32, y0 as f32, Color::new(255, 255, 255), z));

        // Si hemos alcanzado el punto final, salimos del bucle.
        if x0 == x1 && y0 == y1 { break; }

        let e2 = err; // Almacena el error actual para usarlo en las siguientes decisiones.
        if e2 > -dx {
            err -= dy; // Ajusta el error.
            x0 += sx; // Avanza en la dirección de x.
        }
        if e2 < dy {
            err += dx; // Ajusta el error.
            y0 += sy; // Avanza en la dirección de y.
        }
    }

    fragments // Retorna el vector de fragmentos que representa la línea.
}
