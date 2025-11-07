# Lab-4-graficas


Este repo contiene una mini “CPU rasterizer” escrita en Rust junto con varios shaders para renderizar planetas, lunas y anillos.
Aquí se explica qué archivos son importantes, qué espera el pipeline y qué significan los parámetros que se mandan como uniforms.

Organización del proyecto

src/shaders/ → aquí están los shaders específicos: planeta verde, rojo, gaseoso, luna, anillos, etc.

src/renderer/ → pipeline, rasterizador, buffers y el archivo de uniforms.rs.

src/shaders/common.rs → helpers y utilidades compartidas (math, FBM, color utilities, etc.)



Todos los shaders deben implementar el trait Shader definido en src/renderer/pipeline.rs.
Trait Shader
Método y	Para qué sirve
-name(&self)	nombre del shader (solo para logs / debug)
-vertex(vin, uniforms)	transforma el vértice al clip space y devuelve info en world space
-fragment(vary, uniforms)	recibe atributos ya interpolados y devuelve el color final del pixel
ertex In / Out

Entrada VertexIn { pos: Vec3, nrm: Vec3, uv: Vec2 }

El struct Uniforms es el que contiene TODA la info global que necesita el shader:

Campo	
time: tiempo global para animaciones
light_dir: dirección de la luz principal
view, proj,: model	matrices estándar del render
camera_pos: posición de cámara (world space)
planet: PlanetParams:	parámetros visuales del planeta actual

PlanetParams: Configura el look del planeta.

Parámetro	
base_color: color principal
band_freq: frecuencia de bandas
noise_scale: qué tan fuerte influye el ruido
rim_power: intensidad del rim light
rotation_speed: qué tan rápido gira el planeta
has_rings: boolean para renderizar anillos
has_moon: boolean para activar luna

Para correr el lab:
cargo run --release




Salida VertexOut { clip_pos: Vec4, pos_ws: Vec3, nrm_ws: Vec3, uv: Vec2 }
