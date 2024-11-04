// Importa las bibliotecas necesarias.
use tobj; // Para cargar archivos OBJ.
use nalgebra_glm::{Vec2, Vec3}; // Para trabajar con vectores 2D y 3D.
use crate::vertex::Vertex; // Importa la estructura Vertex desde el módulo vertex.

// Define la estructura principal que representa un objeto 3D.
pub struct Obj {
    // Contiene un vector de mallas (meshes) del objeto.
    meshes: Vec<Mesh>,
}

// Define la estructura que representa una malla, que contiene vértices, normales, coordenadas de textura e índices.
struct Mesh {
    vertices: Vec<Vec3>, // Vértices de la malla en 3D.
    normals: Vec<Vec3>,  // Normales para cada vértice.
    texcoords: Vec<Vec2>, // Coordenadas de textura para cada vértice.
    indices: Vec<u32>,    // Índices que definen el orden de los vértices.
}

// Implementación de métodos para la estructura Obj.
impl Obj {
    // Carga un objeto desde un archivo .obj.
    pub fn load(filename: &str) -> Result<Self, tobj::LoadError> {
        // Carga el archivo OBJ utilizando tobj.
        let (models, _) = tobj::load_obj(filename, &tobj::LoadOptions {
            single_index: true, // Usa un solo índice para los vértices.
            triangulate: true,  // Triangula la malla.
            ..Default::default() // Usa las opciones predeterminadas.
        })?;

        // Convierte cada modelo en una malla.
        let meshes = models.into_iter().map(|model| {
            let mesh = model.mesh; // Obtiene la malla del modelo.
            Mesh {
                // Convierte las posiciones de los vértices en Vec3.
                vertices: mesh.positions.chunks(3)
                    .map(|v| Vec3::new(v[0], v[1], v[2]))
                    .collect(),
                // Convierte las normales en Vec3.
                normals: mesh.normals.chunks(3)
                    .map(|n| Vec3::new(n[0], n[1], n[2]))
                    .collect(),
                // Convierte las coordenadas de textura en Vec2, invirtiendo el eje Y.
                texcoords: mesh.texcoords.chunks(2)
                    .map(|t| Vec2::new(t[0], 1.0 - t[1]))
                    .collect(),
                // Usa los índices directamente de la malla.
                indices: mesh.indices,
            }
        }).collect();

        // Devuelve el objeto cargado.
        Ok(Obj { meshes })
    }

    // Devuelve un vector de vértices (Vertex) a partir de la malla.
    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new(); // Vector para almacenar los vértices.

        // Itera sobre cada malla.
        for mesh in &self.meshes {
            // Itera sobre cada índice en la malla.
            for &index in &mesh.indices {
                // Obtiene la posición del vértice usando el índice.
                let position = mesh.vertices[index as usize];
                // Obtiene la normal, o usa una normal predeterminada si no está disponible.
                let normal = mesh.normals.get(index as usize)
                    .cloned()
                    .unwrap_or(Vec3::new(0.0, 1.0, 0.0));
                // Obtiene las coordenadas de textura, o usa (0, 0) si no están disponibles.
                let tex_coords = mesh.texcoords.get(index as usize)
                    .cloned()
                    .unwrap_or(Vec2::new(0.0, 0.0));

                // Calcula la elevación (puedes personalizar este cálculo).
                let elevation = position.y; // Usa la componente Y como elevación.

                // Crea un nuevo vértice y lo agrega al vector.
                vertices.push(Vertex::new(position, normal, tex_coords, elevation));
            }
        }

        // Devuelve el vector de vértices.
        vertices
    }
}
