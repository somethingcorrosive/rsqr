use qrcode::{QrCode, EcLevel};
use qrcode::types::Color;
use anyhow::{Result, anyhow};

/// Generates a QR code matrix from input text
pub fn generate_qr_matrix(data: &str, quiet_zone: bool) -> Result<Vec<Vec<bool>>> {
    let code = QrCode::with_error_correction_level(data.as_bytes(), EcLevel::M)
        .map_err(|e| anyhow!("QR generation error: {e}"))?;

    let width = code.width();
    let mut matrix = Vec::with_capacity(width);

    for y in 0..width {
        let mut row = Vec::with_capacity(width);
        for x in 0..width {
            let pixel = code[(x, y)];
            row.push(matches!(pixel, Color::Dark));
        }
        matrix.push(row);
    }

    if quiet_zone {
        Ok(pad_matrix(&matrix, 4))
    } else {
        Ok(matrix)
    }
}

/// Adds quiet zone (padding) around matrix
fn pad_matrix(matrix: &[Vec<bool>], pad: usize) -> Vec<Vec<bool>> {
    let width = matrix[0].len() + 2 * pad;
    let mut padded = Vec::with_capacity(matrix.len() + 2 * pad);

    let empty_row = vec![false; width];
    for _ in 0..pad {
        padded.push(empty_row.clone());
    }

    for row in matrix {
        let mut padded_row = Vec::with_capacity(width);
        padded_row.extend(std::iter::repeat(false).take(pad));
        padded_row.extend_from_slice(row);
        padded_row.extend(std::iter::repeat(false).take(pad));
        padded.push(padded_row);
    }

    for _ in 0..pad {
        padded.push(empty_row.clone());
    }

    padded
}

/// Export QR code as PNG to given path with scaling
pub fn export_png(data: &str, path: &str, scale: u32) -> Result<()> {
    use image::{ImageBuffer, Luma};

    let matrix = generate_qr_matrix(data, true)?; // Always add quiet zone for PNG
    let img_size = (matrix.len() as u32) * scale;

    let mut img = ImageBuffer::new(img_size, img_size);

    for (y, row) in matrix.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let pixel = if cell { Luma([0u8]) } else { Luma([255u8]) };
            for dy in 0..scale {
                for dx in 0..scale {
                    img.put_pixel(
                        (x as u32) * scale + dx,
                        (y as u32) * scale + dy,
                        pixel,
                    );
                }
            }
        }
    }

    img.save(path)?;
    Ok(())
}
