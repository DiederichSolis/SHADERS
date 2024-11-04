Shader (moon):

![image](https://github.com/user-attachments/assets/7d330f08-94f1-458d-a5d1-95ad2ca681dc)

Shader (Tierra):

![image](https://github.com/user-attachments/assets/6ef4e7ee-8b59-4c96-922c-c9e41af9ceb2)

Shader (sol):

![image](https://github.com/user-attachments/assets/9adca974-287b-40b6-9375-4ffe5fcebbe0)

Shader (gaseoso)

![image](https://github.com/user-attachments/assets/3c017dd5-99e6-41d8-81d1-d559cd06b3e8)

Shader (rocoso)

![image](https://github.com/user-attachments/assets/901f52d7-79e1-4526-ae1c-1232c22b170a)

Shader (estrella)

![image](https://github.com/user-attachments/assets/0d89de70-111b-4105-9670-dd83099ae950)

Shader (fantasioso)

![image](https://github.com/user-attachments/assets/9bdd5a1c-1e85-48c8-ac9e-72a190ba830a)


para visualizar los demas plantes descomenta ls fn en el archivo shaders:

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
