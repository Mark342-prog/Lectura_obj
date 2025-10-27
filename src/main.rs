use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
struct Normal {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
struct TextureCoord {
    u: f32,
    v: f32,
}

#[derive(Debug)]
struct Face {
    vertex_indices: Vec<usize>,
    normal_indices: Vec<usize>,
    texture_indices: Vec<usize>,
}

fn parse_vertex(parts: &[&str]) -> Option<Vertex> {
    if parts.len() >= 4 {
        Some(Vertex {
            x: parts[1].parse().ok()?,
            y: parts[2].parse().ok()?,
            z: parts[3].parse().ok()?,
        })
    } else {
        None
    }
}

fn parse_normal(parts: &[&str]) -> Option<Normal> {
    if parts.len() >= 4 {
        Some(Normal {
            x: parts[1].parse().ok()?,
            y: parts[2].parse().ok()?,
            z: parts[3].parse().ok()?,
        })
    } else {
        None
    }
}

fn parse_texture_coord(parts: &[&str]) -> Option<TextureCoord> {
    if parts.len() >= 3 {
        Some(TextureCoord {
            u: parts[1].parse().ok()?,
            v: parts[2].parse().ok()?,
        })
    } else {
        None
    }
}

fn parse_face(parts: &[&str]) -> Option<Face> {
    if parts.len() < 4 {
        return None;
    }
    
    let mut vertex_indices = Vec::new();
    let mut normal_indices = Vec::new();
    let mut texture_indices = Vec::new();
    
    for i in 1..parts.len() {
        let face_part = parts[i];
        let indices: Vec<&str> = face_part.split('/').collect();
        
        if let Ok(v_idx) = indices[0].parse::<usize>() {
            vertex_indices.push(v_idx - 1); // OBJ files are 1-indexed, convert to 0-indexed
        }
        
        if indices.len() > 1 && !indices[1].is_empty() {
            if let Ok(t_idx) = indices[1].parse::<usize>() {
                texture_indices.push(t_idx - 1);
            }
        }
        
        if indices.len() > 2 && !indices[2].is_empty() {
            if let Ok(n_idx) = indices[2].parse::<usize>() {
                normal_indices.push(n_idx - 1);
            }
        }
    }
    
    Some(Face {
        vertex_indices,
        normal_indices,
        texture_indices,
    })
}

fn main() {
    // Use relative path to the OBJ file
    let path = "src/obj/Character_female_1.obj";
    
    if !Path::new(path).exists() {
        eprintln!("Error: El archivo {} no existe.", path);
        eprintln!("Asegúrate de que el archivo Character_female_1.obj esté en la carpeta src/obj/");
        return;
    }
    
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error abriendo el archivo {}: {}", path, e);
            return;
        }
    };
    
    let reader = BufReader::new(file);

    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut texcoords = Vec::new();
    let mut faces = Vec::new();
    let mut materials = Vec::new();
    let mut mtllibs = Vec::new();
    let mut objects = Vec::new();
    let mut groups = Vec::new();

    println!("Cargando modelo 3D: Character_female_1.obj");
    println!("{}", "=".repeat(50));

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = match line_result {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error leyendo línea {}: {}", line_num + 1, e);
                continue;
            }
        };
        
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        
        match parts[0] {
            "v" => {
                if let Some(vertex) = parse_vertex(&parts) {
                    vertices.push(vertex);
                }
            },
            "vn" => {
                if let Some(normal) = parse_normal(&parts) {
                    normals.push(normal);
                }
            },
            "vt" => {
                if let Some(texcoord) = parse_texture_coord(&parts) {
                    texcoords.push(texcoord);
                }
            },
            "f" => {
                if let Some(face) = parse_face(&parts) {
                    faces.push(face);
                }
            },
            "usemtl" => {
                if parts.len() > 1 {
                    materials.push(parts[1].to_string());
                }
            },
            "mtllib" => {
                if parts.len() > 1 {
                    mtllibs.push(parts[1].to_string());
                }
            },
            "o" => {
                if parts.len() > 1 {
                    objects.push(parts[1].to_string());
                }
            },
            "g" => {
                if parts.len() > 1 {
                    groups.push(parts[1].to_string());
                }
            },
            _ => {}
        }
    }

    println!("Modelo cargado exitosamente!");
    println!();
    
    // Summary statistics
    println!(" ESTADÍSTICAS DEL MODELO:");
    println!("  🔹 Vértices: {}", vertices.len());
    println!("  🔹 Normales: {}", normals.len());
    println!("  🔹 Coordenadas de textura: {}", texcoords.len());
    println!("  🔹 Caras: {}", faces.len());
    println!("  🔹 Materiales: {}", materials.len());
    println!("  🔹 Bibliotecas de materiales: {}", mtllibs.len());
    println!("  🔹 Objetos: {}", objects.len());
    println!("  🔹 Grupos: {}", groups.len());
    println!();
    
    // Show bounding box
    if !vertices.is_empty() {
        let min_x = vertices.iter().map(|v| v.x).fold(f32::INFINITY, f32::min);
        let max_x = vertices.iter().map(|v| v.x).fold(f32::NEG_INFINITY, f32::max);
        let min_y = vertices.iter().map(|v| v.y).fold(f32::INFINITY, f32::min);
        let max_y = vertices.iter().map(|v| v.y).fold(f32::NEG_INFINITY, f32::max);
        let min_z = vertices.iter().map(|v| v.z).fold(f32::INFINITY, f32::min);
        let max_z = vertices.iter().map(|v| v.z).fold(f32::NEG_INFINITY, f32::max);
        
        println!("📐 DIMENSIONES (Bounding Box):");
        println!(" X: {:.3} a {:.3} (ancho: {:.3})", min_x, max_x, max_x - min_x);
        println!(" Y: {:.3} a {:.3} (alto: {:.3})", min_y, max_y, max_y - min_y);
        println!(" Z: {:.3} a {:.3} (profundidad: {:.3})", min_z, max_z, max_z - min_z);
        println!();
    }
    
    // Show material libraries
    if !mtllibs.is_empty() {
        println!(" BIBLIOTECAS DE MATERIALES:");
        for lib in &mtllibs {
            println!("  📖 {}", lib);
        }
        println!();
    }
    
    // Show materials
    if !materials.is_empty() {
        println!(" MATERIALES UTILIZADOS:");
        let unique_materials: std::collections::HashSet<_> = materials.iter().collect();
        for material in unique_materials {
            println!("  🎭 {}", material);
        }
        println!();
    }
    
    // Show objects and groups
    if !objects.is_empty() {
        println!(" OBJETOS:");
        for obj in &objects {
            println!("  🔧 {}", obj);
        }
        println!();
    }
    
    if !groups.is_empty() {
        println!(" GRUPOS:");
        for group in &groups {
            println!("  🏷️ {}", group);
        }
        println!();
    }
    
    // Show first few vertices as example
    if !vertices.is_empty() {
        println!(" PRIMEROS VÉRTICES (máximo 5):");
        for (i, vertex) in vertices.iter().take(5).enumerate() {
            println!("  {}: ({:.3}, {:.3}, {:.3})", i + 1, vertex.x, vertex.y, vertex.z);
        }
        if vertices.len() > 5 {
            println!("  ... y {} vértices más", vertices.len() - 5);
        }
        println!();
    }
    
    // Show first few faces as example
    if !faces.is_empty() {
        println!(" PRIMERAS CARAS (máximo 3):");
        for (i, face) in faces.iter().take(3).enumerate() {
            println!("  Cara {}: {} vértices", i + 1, face.vertex_indices.len());
            print!("    Vértices: [");
            for (j, &v_idx) in face.vertex_indices.iter().enumerate() {
                if j > 0 { print!(", "); }
                print!("{}", v_idx + 1); // Convert back to 1-indexed for display
            }
            println!("]");
            
            if !face.texture_indices.is_empty() {
                print!("    Texturas: [");
                for (j, &t_idx) in face.texture_indices.iter().enumerate() {
                    if j > 0 { print!(", "); }
                    print!("{}", t_idx + 1);
                }
                println!("]");
            }
            
            if !face.normal_indices.is_empty() {
                print!("    Normales: [");
                for (j, &n_idx) in face.normal_indices.iter().enumerate() {
                    if j > 0 { print!(", "); }
                    print!("{}", n_idx + 1);
                }
                println!("]");
            }
        }
        if faces.len() > 3 {
            println!("  ... y {} caras más", faces.len() - 3);
        }
    }
    
    println!();
    println!(" Análisis del modelo completado!");
}
