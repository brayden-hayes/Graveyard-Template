use crate::geometry::pose2d::Pose2D;
use std::fs::File;
use std::io::Write;

/// Field size in inches 12 ft = 144 in.
const FIELD_SIZE: f64 = 144.0;

/// Pixels per inch in the output image
const SCALE: f64 = 5.0;

/// Draw a list of poses as a trajectory on the field and save it as an SVG.
pub fn write_trajectory_svg(poses: &[Pose2D], path: &str) -> std::io::Result<()> {
    let width = (FIELD_SIZE * SCALE) as u32;
    let height = (FIELD_SIZE * SCALE) as u32;

    let mut file = File::create(path)?;

    // SVG header
    writeln!(file, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
    writeln!(
        file,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
        width, height, width, height
    )?;

    // Dark background
    writeln!(file, r##"<rect width="100%" height="100%" fill="#1a1a2e"/>"##)?;

    // Field border
    writeln!(
        file,
        r##"<rect x="0" y="0" width="{}" height="{}" fill="none" stroke="#555" stroke-width="2"/>"##,
        width, height
    )?;

    // Light grid every 24 inches
    for i in 1..6 {
        let pos = (i as f64 * 24.0 * SCALE) as u32;
        writeln!(
            file,
            r##"<line x1="{}" y1="0" x2="{}" y2="{}" stroke="#333" stroke-width="1"/>"##,
            pos, pos, height
        )?;
        writeln!(
            file,
            r##"<line x1="0" y1="{}" x2="{}" y2="{}" stroke="#333" stroke-width="1"/>"##,
            pos, width, pos
        )?;
    }

    // Trajectory line
    if poses.len() >= 2 {
        write!(file, r##"<polyline fill="none" stroke="#00d4ff" stroke-width="2" points=""##)?;
        for pose in poses {
            let x = pose.x() * SCALE;
            let y = (FIELD_SIZE - pose.y()) * SCALE; // flip Y (SVG y goes down)
            write!(file, "{:.1},{:.1} ", x, y)?;
        }
        writeln!(file, r#""/>"#)?;
    }

    // Start marker (green)
    if let Some(start) = poses.first() {
        let x = start.x() * SCALE;
        let y = (FIELD_SIZE - start.y()) * SCALE;
        writeln!(
            file,
            r##"<circle cx="{:.1}" cy="{:.1}" r="6" fill="#00ff88"/>"##,
            x, y
        )?;
    }

    // End marker (red) + heading tick
    if let Some(end) = poses.last() {
        let x = end.x() * SCALE;
        let y = (FIELD_SIZE - end.y()) * SCALE;
        writeln!(
            file,
            r##"<circle cx="{:.1}" cy="{:.1}" r="6" fill="#ff4466"/>"##,
            x, y
        )?;

        // Small line showing final heading
        let angle = end.rotation().radians();
        let len = 12.0;
        let x2 = x + len * angle.cos();
        let y2 = y - len * angle.sin(); // minus because Y is flipped
        writeln!(
            file,
            r##"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="#ff4466" stroke-width="2"/>"##,
            x, y, x2, y2
        )?;
    }

    // Legend
    writeln!(
        file,
        r##"<text x="10" y="20" fill="#ccc" font-family="monospace" font-size="14">Cyan = path | Green = start | Red = end</text>"##
    )?;

    writeln!(file, "</svg>")?;
    Ok(())
}