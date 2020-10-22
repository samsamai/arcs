use euclid::Length;

/// A Grid
#[derive(Debug, Default, PartialEq)]
pub struct Grid<S> {
    /// The spacing between the horizontal and vertical grid lines.
    pub grid_spacing: Length<f64, S>,
    /// Snapping to grid
    pub snap: bool,
}

impl<S> Grid<S> {
    /// Create a new [`Grid`].
    pub const fn new(grid_spacing: Length<f64, S>, snap: bool) -> Self {
        Grid { grid_spacing, snap }
    }
}

impl<S> Copy for Grid<S> {}

impl<S> Clone for Grid<S> {
    fn clone(&self) -> Self {
        *self
    }
}
