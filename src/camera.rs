use nalgebra_glm::{Vec3, rotate_vec3};
use std::f32::consts::PI;

/// Representa una cámara en un espacio 3D.
pub struct Camera {
    /// Posición de la cámara (punto de vista).
    pub eye: Vec3,
    /// Punto al que la cámara está mirando.
    pub center: Vec3,
    /// Vector que representa la dirección "arriba" para la cámara.
    pub up: Vec3,
    /// Indica si la cámara ha cambiado de posición o dirección.
    pub has_changed: bool,
}

impl Camera {
    /// Crea una nueva instancia de `Camera`.
    ///
    /// # Parámetros
    /// - `eye`: La posición inicial de la cámara.
    /// - `center`: El punto que la cámara está mirando.
    /// - `up`: La dirección "arriba" para la cámara.
    ///
    /// # Retorna
    /// Una nueva instancia de `Camera`.
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera {
            eye,
            center,
            up,
            has_changed: true,
        }
    }

    /// Cambia la base del vector proporcionado según la orientación de la cámara.
    ///
    /// # Parámetros
    /// - `vector`: Un vector que se transformará según la orientación de la cámara.
    ///
    /// # Retorna
    /// El vector transformado.
    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();

        let rotated = 
            vector.x * right +
            vector.y * up +
            -vector.z * forward;

        rotated.normalize()
    }

    /// Realiza una órbita alrededor del punto central de la cámara.
    ///
    /// # Parámetros
    /// - `delta_yaw`: Cambio en el ángulo de giro alrededor del eje Y.
    /// - `delta_pitch`: Cambio en el ángulo de elevación alrededor del eje X.
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        let current_yaw = radius_vector.z.atan2(radius_vector.x);

        let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        let new_eye = self.center + Vec3::new(
            radius * new_yaw.cos() * new_pitch.cos(),
            -radius * new_pitch.sin(),
            radius * new_yaw.sin() * new_pitch.cos()
        );

        self.eye = new_eye;
        self.has_changed = true;
    }

    /// Acerca o aleja la cámara a lo largo de su dirección de visión.
    ///
    /// # Parámetros
    /// - `delta`: Cantidad para mover la cámara hacia adelante (positivo) o hacia atrás (negativo).
    pub fn zoom(&mut self, delta: f32) {
        let direction = (self.center - self.eye).normalize();
        self.eye += direction * delta;
        self.has_changed = true;
    }

    /// Mueve el punto central de la cámara en función de un vector de dirección.
    ///
    /// # Parámetros
    /// - `direction`: Un vector que determina la dirección y la magnitud del movimiento.
    pub fn move_center(&mut self, direction: Vec3) {
        let radius_vector = self.center - self.eye;
        let radius = radius_vector.magnitude();

        let angle_x = direction.x * 0.05; // Ajusta este factor para controlar la velocidad de rotación
        let angle_y = direction.y * 0.05;

        let rotated = rotate_vec3(&radius_vector, angle_x, &Vec3::new(0.0, 1.0, 0.0));

        let right = rotated.cross(&self.up).normalize();
        let final_rotated = rotate_vec3(&rotated, angle_y, &right);

        self.center = self.eye + final_rotated.normalize() * radius;
        self.has_changed = true;
    }

    /// Verifica si la cámara ha cambiado y actualiza su estado.
    ///
    /// # Retorna
    /// `true` si la cámara ha cambiado desde la última verificación; de lo contrario, `false`.
    pub fn check_if_changed(&mut self) -> bool {
        if self.has_changed {
            self.has_changed = false;
            true
        } else {
            false
        }
    }
}
