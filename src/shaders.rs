// Importaciones necesarias para el shader, incluyendo vectores y matrices de nalgebra.
use nalgebra_glm::{Vec3, Vec4, Mat3, mat4_to_mat3};
use crate::vertex::Vertex; // Importa la estructura Vertex del módulo correspondiente.
use crate::Uniforms; // Importa la estructura Uniforms del módulo correspondiente.
use crate::fragment::Fragment; // Importa la estructura Fragment del módulo correspondiente.
use crate::color::Color; // Importa la estructura Color del módulo correspondiente.
use rand::Rng; // Mantener solo si se usa
use rand::SeedableRng; // Mantener solo si se usa
use rand::rngs::StdRng; // Mantener solo si se usa

/// Función del shader de vértices que transforma la posición del vértice y calcula el color.
pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Crea un vector 4D a partir de la posición del vértice.
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    // Transforma la posición usando las matrices de proyección, vista y modelo.
    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    // Calcula la componente w para la perspectiva.
    let w = transformed.w;

    // Normaliza las coordenadas transformadas.
    let transformed_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    // Calcula la posición en la pantalla.
    let screen_position = uniforms.viewport_matrix * transformed_position;

    // Convierte la matriz de modelo 4x4 a 3x3 para calcular las normales.
    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

    // Transforma la normal del vértice.
    let transformed_normal = normal_matrix * vertex.normal;

    // Crea un nuevo vértice con los datos transformados y el color inicial.
    let mut new_vertex = Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color, // Color inicial
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal: transformed_normal,
        elevation: vertex.elevation, // Mantiene la elevación original
    };

    // Actualiza el color del vértice basado en la elevación.
    new_vertex.update_color_based_on_elevation();

    // Retorna el nuevo vértice.
    new_vertex
}

/// Función del shader de fragmentos que calcula el color del fragmento.
pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Devuelve el color de la luna o de la tierra, según se desee.
    //earth_color(fragment, uniforms)
    //moon_color(fragment, uniforms)
   // sun_gradient(fragment, uniforms)
   //gas_planet_color(fragment, uniforms)
   //rocky_planet_color(fragment, uniforms)
   //star_planet_color(fragment, uniforms)
   fantasy_planet_color(fragment, uniforms)
}

/// Calcula el color para un fragmento en la superficie de la Tierra.
fn earth_color(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Obtener un valor de ruido para la posición del fragmento.
    let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 10.0, fragment.vertex_position.z * 10.0);
    
    // Aplica el ruido a la elevación.
    let elevation = noise_value; // Aquí puedes escalar el ruido si es necesario.

    // Define umbrales para diferentes tipos de terreno.
    let ocean_threshold = 0.0;        // Umbral para el océano.
    let land_threshold = 0.2;         // Umbral para la tierra baja.
    let mountain_threshold = 0.5;     // Umbral para las montañas.

    // Define colores representativos.
    let ocean_color = Color::new(0, 105, 148);
    let shallow_water_color = Color::new(0, 191, 255);
    let land_color = Color::new(34, 139, 34);
    let mountain_color = Color::new(139, 69, 19);
    let snow_color = Color::new(255, 255, 255);

    // Determina el color basado en la elevación.
    let color = if elevation <= ocean_threshold {
        ocean_color
    } else if elevation > ocean_threshold && elevation <= land_threshold {
        shallow_water_color
    } else if elevation > land_threshold && elevation <= mountain_threshold {
        land_color
    } else {
        let base_color = mountain_color;

        // Agregar blanco para las cumbres nevadas.
        if elevation > mountain_threshold + 0.3 {
            base_color + snow_color * 0.5
        } else {
            base_color
        }
    };

    // Devuelve el color multiplicado por la intensidad del fragmento.
    color * fragment.intensity
}

/// Calcula el color para un fragmento en la superficie de la Luna.
fn moon_color(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Aumenta la escala del ruido para más detalles.
    let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 20.0, fragment.vertex_position.z * 20.0);
    
    let elevation = noise_value; // Puedes combinar varios niveles de ruido si lo deseas.

    // Define umbrales para diferentes tipos de terreno lunar.
    let low_threshold = -0.1; 
    let medium_threshold = 0.1;
    let high_threshold = 0.3; // Agregar un nuevo umbral para cráteres.

    // Define colores representativos para la luna.
    let dark_surface_color = Color::new(169, 169, 169); // Gris oscuro.
    let light_surface_color = Color::new(211, 211, 211); // Gris claro.
    let crater_color = Color::new(255, 255, 255);       // Blanco para los cráteres.

    // Determina el color basado en la elevación lunar.
    let color = if elevation < low_threshold {
        dark_surface_color
    } else if elevation < medium_threshold {
        light_surface_color
    } else if elevation < high_threshold {
        crater_color // Área de cráteres.
    } else {
        Color::new(240, 240, 240) // Color para áreas muy altas.
    };

    // Devuelve el color multiplicado por la intensidad del fragmento.
    color * fragment.intensity
}

fn sun_gradient(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Obtiene un valor de ruido para efectos adicionales (opcional).
    let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 10.0, fragment.vertex_position.z * 10.0);
    
    // Define colores representativos para el sol en tonos naranjas.
    let deep_orange_color = Color::new(255, 140, 0); // Naranja profundo.
    let light_orange_color = Color::new(255, 165, 80); // Naranja claro.
    let white_color = Color::new(255, 255, 255); // Blanco para el brillo.
    let warm_orange_color = Color::new(255, 200, 100); // Naranja cálido para el resplandor.

    // Determina la posición relativa del fragmento para el difuminado.
    let distance_to_sun = (fragment.vertex_position.y - 5.0).abs(); // Ajusta la altura según necesites.
    
    // Calcula un factor de difuminado basado en la distancia.
    let gradient_factor = (1.0 - distance_to_sun / 10.0).max(0.0).min(1.0);
    
    // Calcula el color difuminado combinando los colores.
    let sun_color = 
        deep_orange_color * gradient_factor * 0.5 + 
        light_orange_color * (1.0 - gradient_factor) * 0.5 + 
        warm_orange_color * gradient_factor * 0.3; // Añadiendo naranja cálido para mayor luminosidad.

    // Agrega un brillo adicional alrededor del sol.
    let glow_color = white_color * 0.3 * gradient_factor; // Brillo suave alrededor del sol.
    
    // Combina el color del sol y el brillo.
    let final_color = sun_color + glow_color;

    // Crea variaciones adicionales para simular partes del sol y su halo.
    if noise_value > 0.2 {
        let halo_color = Color::new(255, 160, 50); // Color del halo en un tono naranja más suave.
        let halo_factor = (noise_value - 0.2).min(0.5); // Intensifica el halo basado en el ruido.
        return final_color + halo_color * halo_factor; // Combina el color del halo.
    }

    final_color
}


fn gas_planet_color(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Utiliza la posición del fragmento y el tiempo para generar un "seed" para el ruido.
    let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;
    
    // Crea un generador de números aleatorios basado en el seed.
    let mut rng = StdRng::seed_from_u64(seed.abs() as u64);
    
    // Genera un número aleatorio para la variación en el color.
    let random_number = rng.gen_range(0..=100);

    // Define colores base para el planeta gaseoso.
    let base_color = Color::new(70, 130, 180); // Azul
    let cloud_color = Color::new(255, 255, 255); // Blanco para nubes
    let shadow_color = Color::new(50, 50, 100); // Color oscuro para sombras

    // Calcular el factor de nubes usando el ruido
    let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 5.0, fragment.vertex_position.z * 5.0);
    let cloud_factor = (noise_value * 0.5 + 0.5).powi(2); // Escala el ruido entre 0 y 1.

    // Selección de color basado en el número aleatorio para agregar variación.
    let planet_color = if random_number < 50 {
        base_color * (1.0 - cloud_factor) + cloud_color * cloud_factor
    } else {
        cloud_color * cloud_factor // Predominan las nubes
    };

    // Añadir sombras sutiles
    let shadow_factor = (1.0 - noise_value).max(0.0);
    let shadow_effect = shadow_color * shadow_factor * 0.3; // Sombra suave

    // Combina el color del planeta y las sombras
    let final_color = planet_color + shadow_effect;

    // Brillo atmosférico (opcional)
    let glow_color = Color::new(200, 200, 255); // Brillo azul claro
    let glow_factor = (1.0 - (fragment.vertex_position.y / 10.0).max(0.0).min(1.0)).max(0.0); // Basado en altura
    let final_glow = glow_color * glow_factor * 0.1; // Brillo sutil

    // Devuelve el color final combinado
    final_color + final_glow
}

fn rocky_planet_color(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Utiliza la posición del fragmento y el tiempo para generar un "seed" para el ruido.
    let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;

    // Crea un generador de números aleatorios basado en el seed.
    let mut rng = StdRng::seed_from_u64(seed.abs() as u64);
    
    // Define colores base para el planeta rocoso.
    let base_color = Color::new(139, 69, 19); // Marrón (color de tierra)
    let highlight_color = Color::new(255, 255, 255); // Blanco para resaltar
    let shadow_color = Color::new(80, 50, 0); // Sombra más oscura

    // Calcular el factor de ruido para la textura del planeta
    let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 3.0, fragment.vertex_position.z * 3.0);
    let texture_factor = (noise_value * 0.5 + 0.5).powi(2); // Escala el ruido entre 0 y 1.

    // Crear líneas utilizando el ruido en la textura
    let line_factor = ((fragment.vertex_position.x + fragment.vertex_position.z) * 10.0).sin();
    let line_color = if line_factor > 0.0 {
        highlight_color
    } else {
        shadow_color
    };

    // Combina el color base con las líneas
    let planet_color = base_color * (1.0 - texture_factor) + line_color * texture_factor;

    // Añadir sombras sutiles
    let shadow_factor = (1.0 - noise_value).max(0.0);
    let shadow_effect = shadow_color * shadow_factor * 0.3; // Sombra suave

    // Devuelve el color final combinado
    planet_color + shadow_effect
}

fn star_planet_color(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Utiliza la posición del fragmento y el tiempo para generar un "seed" para el ruido.
    let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;

    // Crea un generador de números aleatorios basado en el seed.
    let mut rng = StdRng::seed_from_u64(seed.abs() as u64);
    
    // Define colores base para el planeta estrellado.
    let base_color = Color::new(30, 30, 60); // Azul oscuro para el fondo del planeta
    let star_color = Color::new(255, 255, 255); // Blanco para las estrellas

    // Calcular el factor de ruido para las estrellas
    let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 5.0, fragment.vertex_position.z * 5.0);
    let star_factor = (noise_value * 0.5 + 0.5).powi(2); // Escala el ruido entre 0 y 1.

    // Determinar si hay una estrella en este fragmento
    let random_number = rng.gen_range(0..=100);
    let is_star = random_number < (star_factor * 100.0) as u32; // Convertir el star_factor a un rango de 0 a 100.

    // Calcular el color del fragmento
    let planet_color = if is_star {
        star_color // Si es una estrella, usa el color de la estrella
    } else {
        base_color // Si no, usa el color de fondo
    };

    // Añadir un brillo para las estrellas
    let brightness = if is_star { 0.7 } else { 0.0 }; // Brillo solo si es una estrella
    let star_effect = star_color * brightness;

    // Devuelve el color final combinado
    planet_color + star_effect
}

fn fantasy_planet_color(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Define colores base para el planeta fantástico.
    let base_color = Color::new(50, 50, 200); // Azul profundo.
    let accent_color = Color::new(180, 80, 200); // Púrpura vibrante para los detalles.
    let cloud_color = Color::new(255, 255, 255); // Blanco para las nubes.

    // Cálculo del ruido para la textura del planeta
    let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 2.0, fragment.vertex_position.z * 2.0);
    let cloud_factor = (noise_value * 0.5 + 0.5).powi(3); // Escala el ruido para suavizar la textura.

    // Calcula un desplazamiento basado en la posición Y del fragmento para crear capas.
    let layer_offset = (fragment.vertex_position.y * 0.5).sin() * 0.5; // Controla la "altura" de las capas.
    
    // Determina el color del fragmento en función de su altura
    let planet_color = if fragment.vertex_position.y > 0.0 {
        // En la parte superior del planeta, añade un efecto de nubes
        base_color * (1.0 - cloud_factor) + cloud_color * cloud_factor
    } else {
        // En la parte inferior del planeta, utiliza el color de acento
        accent_color * (1.0 + layer_offset)
    };

    // Efecto de brillo para dar profundidad
    let brightness = 0.1; // Brillo constante
    let light_effect = Color::new((brightness * 255.0) as u8, (brightness * 255.0) as u8, (brightness * 255.0) as u8);

    // Devuelve el color final combinado
    planet_color + light_effect
}
