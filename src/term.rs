//! Terminal emulation types using `alacritty_terminal`

use vte::ansi::Rgb;

/// A single cell in the terminal grid
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    /// Character displayed in this cell
    pub c: char,
    /// Foreground color
    pub fg: Color,
    /// Background color
    pub bg: Color,
    /// Text style
    pub style: Style,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            c: ' ',
            fg: Color::default_fg(),
            bg: Color::default_bg(),
            style: Style::default(),
        }
    }
}

impl Cell {
    /// Create a new cell with the given character
    #[must_use]
    pub fn new(c: char) -> Self {
        Self {
            c,
            ..Default::default()
        }
    }

    /// Create a cell with character and colors
    #[must_use]
    pub fn with_colors(c: char, fg: Color, bg: Color) -> Self {
        Self {
            c,
            fg,
            bg,
            style: Style::default(),
        }
    }

    /// Check if this cell is empty (whitespace with default colors)
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.c.is_whitespace() && self.fg == Color::default_fg() && self.bg == Color::default_bg()
    }
}

/// RGB color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new color from RGB values
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Default foreground color (light gray)
    #[must_use]
    pub const fn default_fg() -> Self {
        Self::new(204, 204, 204)
    }

    /// Default background color (black)
    #[must_use]
    pub const fn default_bg() -> Self {
        Self::new(0, 0, 0)
    }

    /// Convert to CSS `rgb()` string
    #[must_use]
    pub fn to_css(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    /// Convert to hex color string
    #[must_use]
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        Self::new(rgb.r, rgb.g, rgb.b)
    }
}

/// Text style flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct Style {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub dim: bool,
    pub inverse: bool,
}

impl Style {
    /// Create a new style with all flags disabled
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            dim: false,
            inverse: false,
        }
    }

    /// Check if any style is applied
    #[must_use]
    pub const fn has_any(&self) -> bool {
        self.bold || self.italic || self.underline || self.strikethrough || self.dim || self.inverse
    }

    /// Generate CSS classes for this style
    #[must_use]
    pub fn to_css_classes(&self) -> String {
        let mut classes = Vec::new();
        if self.bold {
            classes.push("font-bold");
        }
        if self.italic {
            classes.push("italic");
        }
        if self.underline {
            classes.push("underline");
        }
        if self.strikethrough {
            classes.push("line-through");
        }
        if self.dim {
            classes.push("opacity-50");
        }
        classes.join(" ")
    }
}

/// Terminal grid containing all cells
#[derive(Debug, Clone)]
pub struct Grid {
    cells: Vec<Cell>,
    rows: usize,
    cols: usize,
}

impl Grid {
    /// Create a new grid with the given dimensions
    ///
    /// # Panics
    ///
    /// Panics if rows or cols is 0.
    #[must_use]
    pub fn new(rows: usize, cols: usize) -> Self {
        assert!(rows > 0, "rows must be positive");
        assert!(cols > 0, "cols must be positive");

        Self {
            cells: vec![Cell::default(); rows * cols],
            rows,
            cols,
        }
    }

    /// Get the number of rows
    #[must_use]
    pub const fn rows(&self) -> usize {
        self.rows
    }

    /// Get the number of columns
    #[must_use]
    pub const fn cols(&self) -> usize {
        self.cols
    }

    /// Get a cell at the given position
    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        if row < self.rows && col < self.cols {
            Some(&self.cells[row * self.cols + col])
        } else {
            None
        }
    }

    /// Get a mutable reference to a cell
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        if row < self.rows && col < self.cols {
            Some(&mut self.cells[row * self.cols + col])
        } else {
            None
        }
    }

    /// Set a cell at the given position
    pub fn set(&mut self, row: usize, col: usize, cell: Cell) {
        if row < self.rows && col < self.cols {
            self.cells[row * self.cols + col] = cell;
        }
    }

    /// Clear the entire grid
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::default();
        }
    }

    /// Iterate over rows
    pub fn iter_rows(&self) -> impl Iterator<Item = &[Cell]> {
        self.cells.chunks(self.cols)
    }

    /// Resize the grid, preserving content where possible
    ///
    /// # Panics
    ///
    /// Panics if `new_rows` or `new_cols` is 0.
    pub fn resize(&mut self, new_rows: usize, new_cols: usize) {
        assert!(new_rows > 0, "rows must be positive");
        assert!(new_cols > 0, "cols must be positive");

        let mut new_cells = vec![Cell::default(); new_rows * new_cols];

        for row in 0..new_rows.min(self.rows) {
            for col in 0..new_cols.min(self.cols) {
                new_cells[row * new_cols + col] = self.cells[row * self.cols + col].clone();
            }
        }

        self.cells = new_cells;
        self.rows = new_rows;
        self.cols = new_cols;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_default() {
        let cell = Cell::default();
        assert_eq!(cell.c, ' ');
        assert!(cell.is_empty());
    }

    #[test]
    fn test_cell_new() {
        let cell = Cell::new('A');
        assert_eq!(cell.c, 'A');
        assert!(!cell.is_empty());
    }

    #[test]
    fn test_cell_with_colors() {
        let fg = Color::new(255, 0, 0);
        let bg = Color::new(0, 0, 255);
        let cell = Cell::with_colors('X', fg, bg);
        assert_eq!(cell.c, 'X');
        assert_eq!(cell.fg, fg);
        assert_eq!(cell.bg, bg);
    }

    #[test]
    fn test_color_new() {
        let c = Color::new(128, 64, 32);
        assert_eq!(c.r, 128);
        assert_eq!(c.g, 64);
        assert_eq!(c.b, 32);
    }

    #[test]
    fn test_color_to_css() {
        let c = Color::new(255, 128, 0);
        assert_eq!(c.to_css(), "rgb(255, 128, 0)");
    }

    #[test]
    fn test_color_to_hex() {
        let c = Color::new(255, 128, 0);
        assert_eq!(c.to_hex(), "#ff8000");
    }

    #[test]
    fn test_style_default() {
        let s = Style::default();
        assert!(!s.bold);
        assert!(!s.italic);
        assert!(!s.has_any());
    }

    #[test]
    fn test_style_has_any() {
        let mut s = Style::new();
        assert!(!s.has_any());
        s.bold = true;
        assert!(s.has_any());
    }

    #[test]
    fn test_style_to_css_classes() {
        let s = Style {
            bold: true,
            italic: true,
            underline: false,
            strikethrough: false,
            dim: false,
            inverse: false,
        };
        let classes = s.to_css_classes();
        assert!(classes.contains("font-bold"));
        assert!(classes.contains("italic"));
    }

    #[test]
    fn test_grid_new() {
        let grid = Grid::new(24, 80);
        assert_eq!(grid.rows(), 24);
        assert_eq!(grid.cols(), 80);
    }

    #[test]
    #[should_panic(expected = "rows must be positive")]
    fn test_grid_zero_rows_panics() {
        let _ = Grid::new(0, 80);
    }

    #[test]
    #[should_panic(expected = "cols must be positive")]
    fn test_grid_zero_cols_panics() {
        let _ = Grid::new(24, 0);
    }

    #[test]
    fn test_grid_get_set() {
        let mut grid = Grid::new(10, 10);
        grid.set(5, 5, Cell::new('X'));
        let cell = grid.get(5, 5).unwrap();
        assert_eq!(cell.c, 'X');
    }

    #[test]
    fn test_grid_get_out_of_bounds() {
        let grid = Grid::new(10, 10);
        assert!(grid.get(10, 5).is_none());
        assert!(grid.get(5, 10).is_none());
    }

    #[test]
    fn test_grid_clear() {
        let mut grid = Grid::new(10, 10);
        grid.set(5, 5, Cell::new('X'));
        grid.clear();
        assert!(grid.get(5, 5).unwrap().is_empty());
    }

    #[test]
    fn test_grid_resize_larger() {
        let mut grid = Grid::new(10, 10);
        grid.set(5, 5, Cell::new('X'));
        grid.resize(20, 20);
        assert_eq!(grid.rows(), 20);
        assert_eq!(grid.cols(), 20);
        assert_eq!(grid.get(5, 5).unwrap().c, 'X');
    }

    #[test]
    fn test_grid_resize_smaller() {
        let mut grid = Grid::new(20, 20);
        grid.set(5, 5, Cell::new('X'));
        grid.set(15, 15, Cell::new('Y'));
        grid.resize(10, 10);
        assert_eq!(grid.rows(), 10);
        assert_eq!(grid.cols(), 10);
        assert_eq!(grid.get(5, 5).unwrap().c, 'X');
        assert!(grid.get(15, 15).is_none());
    }

    #[test]
    fn test_grid_iter_rows() {
        let grid = Grid::new(3, 4);
        let rows: Vec<_> = grid.iter_rows().collect();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].len(), 4);
    }
}
