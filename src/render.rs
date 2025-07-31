use crate::cli::Charset;


/// Renders a boolean QR matrix into ASCII characters
pub fn render_ascii(matrix: &[Vec<bool>], invert: bool, charset: Charset) -> String {
    let (dark, light) = match (invert, charset) {
        (false, Charset::Block) => ("██", "  "),
        (true,  Charset::Block) => ("  ", "██"),

        (false, Charset::Hash)  => ("##", "  "),
        (true,  Charset::Hash)  => ("  ", "##"),

        (false, Charset::Dot)   => ("⣿⣿", "  "),
        (true,  Charset::Dot)   => ("  ", "⣿⣿"),
    };

    let mut out = String::new();

    for row in matrix {
        for &cell in row {
            out.push_str(if cell { dark } else { light });
        }
        out.push('\n');
    }

    out
}
