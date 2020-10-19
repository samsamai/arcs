use euclid::{Length, Point2D};

/// A Grid
#[derive(Debug, Default, PartialEq)]
pub struct Grid<S> {
    /// The spacing between the horizontal and vertical grid lines.
    pub grid_spacing: Length<f64, S>,
}

impl<S> Grid<S> {
    /// Create a new [`Grid`].
    pub const fn new(grid_spacing: Length<f64, S>) -> Self {
        Grid { grid_spacing }
    }

    /// return the effective location of the cursor according
    /// to the grid and snapping options
    pub fn effective_location(
        &self,
        drawing_location: Point2D<f64, S>,
    ) -> Point2D<f64, S> {
        Point2D::new(
            drawing_location.x - drawing_location.x % self.grid_spacing.0,
            drawing_location.y - drawing_location.y % self.grid_spacing.0,
        )
    }
}

impl<S> Copy for Grid<S> {}

impl<S> Clone for Grid<S> {
    fn clone(&self) -> Self {
        *self
    }
}
