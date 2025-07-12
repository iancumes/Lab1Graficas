// src/main.rs
mod framebuffer;
use framebuffer::Framebuffer;
use raylib::prelude::*;

// Estructura para el algoritmo scanline
struct Edge {
    y_max: i32,
    current_x: f32,
    inv_slope: f32,
}

/// Dibuja una línea con Bresenham usando el framebuffer
fn plot_line_bresenham(
    fb: &mut Framebuffer,
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    color: Color,
) {
    fb.set_current_color(color);
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 {  1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 {  1 } else { -1 };
    let mut err = dx + dy;

    loop {
        fb.set_pixel(x0 as u32, y0 as u32);
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0  += sx;
        }
        if e2 <= dx {
            err += dx;
            y0  += sy;
        }
    }
}

/// Rellena un polígono (y sus agujeros) con scanline usando el framebuffer
fn fill_polygon_scanline(
    fb: &mut Framebuffer,
    outer: &[(i32, i32)],
    holes: Option<&[&[(i32, i32)]]>,
    color: Color,
) {
    fb.set_current_color(color);

    let y_min = outer.iter().map(|p| p.1).min().unwrap();
    let y_max = outer.iter().map(|p| p.1).max().unwrap();
    let height = (y_max - y_min + 1) as usize;

    let mut table: Vec<Vec<Edge>> = (0..height).map(|_| Vec::new()).collect();
    let mut add_edges = |pts: &[(i32, i32)]| {
        let n = pts.len();
        for i in 0..n {
            let (x0, y0) = pts[i];
            let (x1, y1) = pts[(i + 1) % n];
            if y0 == y1 { continue; }
            let (y_start, x_start, y_end) = if y0 < y1 {
                (y0, x0, y1)
            } else {
                (y1, x1, y0)
            };
            let slope = (x1 - x0) as f32 / (y1 - y0) as f32;
            let edge = Edge {
                y_max: y_end,
                current_x: x_start as f32,
                inv_slope: slope,
            };
            let idx = (y_start - y_min) as usize;
            table[idx].push(edge);
        }
    };

    add_edges(outer);
    if let Some(holes_list) = holes {
        for &h in holes_list {
            add_edges(h);
        }
    }

    let mut active: Vec<Edge> = Vec::new();
    for y in y_min..y_max {
        let row = (y - y_min) as usize;
        active.extend(table[row].drain(..));
        active.retain(|e| e.y_max > y);
        active.sort_by(|a, b| a.current_x.partial_cmp(&b.current_x).unwrap());

        let mut i = 0;
        while i + 1 < active.len() {
            let x_start = active[i].current_x.ceil() as i32;
            let x_end   = active[i + 1].current_x.ceil() as i32;
            for x in x_start..x_end {
                fb.set_pixel(x as u32, y as u32);
            }
            i += 2;
        }

        for edge in active.iter_mut() {
            edge.current_x += edge.inv_slope;
        }
    }
}

/// Dibuja únicamente el contorno de un polígono
fn draw_polygon(fb: &mut Framebuffer, points: &[(i32, i32)], color: Color) {
    if points.len() < 2 { return; }
    for i in 0..points.len() {
        let (x0, y0) = points[i];
        let (x1, y1) = if i + 1 < points.len() { points[i + 1] } else { points[0] };
        plot_line_bresenham(fb, x0, y0, x1, y1, color);
    }
}

fn main() {
    // Creamos nuestro framebuffer de 800×800 con fondo negro
    let mut fb = Framebuffer::new(800, 800, Color::BLACK);

    // Definimos los polígonos
    let polygon_points  = [(165,380),(185,360),(180,330),(207,345),(233,330),
                           (230,360),(250,380),(220,385),(205,410),(193,383)];
    let polygon_points2 = [(321,335),(288,286),(339,251),(374,302)];
    let polygon_points3 = [(377,249),(411,197),(436,249)];


    // Rellenamos cada polígono (polygon_points5 lo pasamos como agujero de polygon_points4)
    fill_polygon_scanline(&mut fb, &polygon_points,  None,                          Color::WHITE);
    fill_polygon_scanline(&mut fb, &polygon_points2, None,                          Color::RED);
    fill_polygon_scanline(&mut fb, &polygon_points3, None,                          Color::BLUE);


    // Dibujamos luego sus contornos
    draw_polygon(&mut fb, &polygon_points,  Color::WHITE);
    draw_polygon(&mut fb, &polygon_points2, Color::RED);
    draw_polygon(&mut fb, &polygon_points3, Color::BLUE);


    // Guardamos la imagen final
    fb.render_to_file("out.png");
    println!("Imagen guardada como 'out.png'");
}
