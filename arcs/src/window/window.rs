use crate::{
    algorithms::Bounded,
    components::{
        layer::LayerType, DrawingObject, Geometry, GridStyle, Layer, LineStyle,
        PointStyle, Viewport, WindowStyle,
    },
    BoundingBox, CanvasSpace, DrawingSpace, Grid, Line, Point,
};
use euclid::{Point2D, Scale, Size2D};
use kurbo::Circle;
use piet::RenderContext;
use shred_derive::SystemData;
use specs::{join::MaybeJoin, prelude::*};
use std::{cmp::Reverse, collections::BTreeMap};

/// A wrapper around the "window" object.
#[derive(Debug, Clone, PartialEq)]
pub struct Window(pub Entity);

impl Window {
    /// Creates a new [`Window`] entity populated with all default components.
    pub fn create(world: &mut World) -> Self {
        let ent = world
            .create_entity()
            .with(Viewport {
                centre: Point::zero(),
                pixels_per_drawing_unit: Scale::new(1.0),
            })
            .with(GridStyle::default())
            .with(LineStyle::default())
            .with(PointStyle::default())
            .with(WindowStyle::default())
            .build();

        Window(ent)
    }

    /// Get a [`System`] which will render using a particular [`RenderContext`].
    ///
    /// # Note
    ///
    /// This snapshots the window's state and styling (e.g. [`Viewport`] and
    /// [`WindowStyle`]) so you shouldn't keep this system around for any length
    /// of time.
    pub fn render_system<'a, R>(
        &'a self,
        backend: R,
        window_size: Size2D<f64, CanvasSpace>,
    ) -> impl System<'a> + 'a
    where
        R: RenderContext + 'a,
    {
        RenderSystem {
            backend,
            window_size,
            window: self,
        }
    }
}

macro_rules! components {
    ($( $get:ident, $get_mut:ident, $component_name:expr => $component_type:ty ),* $(,)?) => {
        $(
            #[doc = "Get a reference to the [`Window`]'s [`"]
            #[doc = $component_name]
            #[doc = "`] component."]
            pub fn $get<'a>(&self, storage: &'a ReadStorage<'a, $component_type>) -> &'a $component_type
            {
                storage
                    .get(self.0)
                    .expect(concat!("The window should always have a ", stringify!($component_type), " component"))
            }

            #[doc = "Get a mutable reference to the [`Window`]'s [`"]
            #[doc = $component_name]
            #[doc = "`] component."]
            pub fn $get_mut<'a, 'world: 'a>(&self, storage: &'a mut WriteStorage<'world, $component_type>) -> &'a mut $component_type
            {
                storage
                    .get_mut(self.0)
                    .expect(concat!("The window should always have a ", stringify!($component_type), " component"))
            }
        )*
    };
}

/// Accessors for the various components attached to this [`Window`].
impl Window {
    components! {
        viewport, viewport_mut, stringify!(Viewport) => Viewport,
        default_point_style, default_point_style_mut, stringify!(PointStyle) => PointStyle,
        default_line_style, default_line_style_mut, stringify!(LineStyle) => LineStyle,
        default_grid_style, default_grid_style_mut, stringify!(GridStyle) => GridStyle,
        style, style_mut, stringify!(WindowStyle) => WindowStyle,
    }
}

/// The [`System`] which actually renders things.
///
/// This is a temporary object "closing over" the [`Window`] and some
/// [`RenderContext`].
#[derive(Debug)]
struct RenderSystem<'window, B> {
    backend: B,
    window_size: Size2D<f64, CanvasSpace>,
    window: &'window Window,
}

impl<'window, B> RenderSystem<'window, B> {
    /// Calculate the area of the drawing displayed by the viewport.
    fn viewport_dimensions(
        &self,
        viewport: &Viewport,
    ) -> BoundingBox<DrawingSpace> {
        let window_size = viewport
            .pixels_per_drawing_unit
            .inv()
            .transform_size(self.window_size);

        BoundingBox::from_centre_and_size(viewport.centre, window_size)
    }
}

impl<'window, B: RenderContext> RenderSystem<'window, B> {
    fn render(
        &mut self,
        ent: Entity,
        drawing_object: &DrawingObject,
        layer: &Layer,
        styles: &Styling,
        viewport: &Viewport,
    ) {
        match layer.layer_type {
            LayerType::System => {
                self.render_system_layer(ent, drawing_object, styles, viewport)
            }
            LayerType::Geometry => self.render_geometry_layers(
                ent,
                drawing_object,
                styles,
                viewport,
            ),
        }
    }

    fn render_geometry_layers(
        &mut self,
        ent: Entity,
        drawing_object: &DrawingObject,
        styles: &Styling,
        viewport: &Viewport,
    ) {
        match drawing_object.geometry {
            Geometry::Point(point) => {
                self.render_point(
                    ent,
                    point,
                    drawing_object.layer,
                    styles,
                    viewport,
                );
            }
            Geometry::Line(ref line) => {
                self.render_line(
                    ent,
                    line,
                    drawing_object.layer,
                    styles,
                    viewport,
                );
            }
            _ => unimplemented!(),
        }
    }

    fn render_system_layer(
        &mut self,
        ent: Entity,
        drawing_object: &DrawingObject,
        styles: &Styling,
        viewport: &Viewport,
    ) {
        match drawing_object.geometry {
            Geometry::Grid(ref grid) => {
                self.render_grid(
                    ent,
                    grid,
                    drawing_object.layer,
                    styles,
                    viewport,
                );
            }
            _ => unimplemented!(),
        }
    }

    /// Draw a [`Point`] as a circle on the canvas.
    fn render_point(
        &mut self,
        entity: Entity,
        point: Point,
        layer: Entity,
        styles: &Styling,
        viewport: &Viewport,
    ) {
        let style = resolve_point_style(styles, self.window, entity, layer);

        let centre = self.to_canvas_coordinates(point, viewport);
        let shape = Circle {
            center: kurbo::Point::new(centre.x, centre.y),
            radius: style.radius.in_pixels(viewport.pixels_per_drawing_unit),
        };
        log::trace!("Drawing {:?} as {:?} using {:?}", point, shape, style);

        self.backend.fill(shape, &style.colour);
    }

    fn render_line(
        &mut self,
        entity: Entity,
        line: &Line,
        layer: Entity,
        styles: &Styling,
        viewport: &Viewport,
    ) {
        let style = resolve_line_style(styles, self.window, entity, layer);

        let start = self.to_canvas_coordinates(line.start, viewport);
        let end = self.to_canvas_coordinates(line.end, viewport);
        let shape = kurbo::Line::new(start.to_tuple(), end.to_tuple());
        let stroke_width =
            style.width.in_pixels(viewport.pixels_per_drawing_unit);
        log::trace!("Drawing {:?} as {:?} using {:?}", line, shape, style);

        self.backend.stroke(shape, &style.stroke, stroke_width);
    }

    fn render_grid(
        &mut self,
        entity: Entity,
        grid: &Grid,
        layer: Entity,
        styles: &Styling,
        viewport: &Viewport,
    ) {
        let style = resolve_grid_style(styles, self.window, entity, layer);
        let stroke_width =
            style.width.in_pixels(viewport.pixels_per_drawing_unit);

        let viewport_dimensions = self.viewport_dimensions(&viewport);

        let min_x = viewport_dimensions.bottom_left().x;
        let start_x = min_x - min_x % grid.grid_spacing.0;
        let max_x = viewport_dimensions.top_right().x;
        let end_x = max_x - max_x % grid.grid_spacing.0;
        let min_y = viewport_dimensions.bottom_left().y;
        let start_y = min_y - min_y % grid.grid_spacing.0;
        let max_y = viewport_dimensions.top_right().y;
        let end_y = max_y - max_y % grid.grid_spacing.0;

        let mut x = start_x;
        while x <= end_x {
            let start = Point2D::new(x, min_y);
            let start = self.to_canvas_coordinates(start, viewport);
            let end = Point2D::new(x, max_y);
            let end = self.to_canvas_coordinates(end, viewport);

            let shape = kurbo::Line::new(start.to_tuple(), end.to_tuple());
            if x == 0. {
                self.backend.stroke(shape, &style.stroke, 1.);
            } else {
                self.backend.stroke(shape, &style.stroke, stroke_width);
            }

            x += grid.grid_spacing.0;
        }

        let mut y = start_y;
        while y <= end_y {
            let start = Point2D::new(min_x, y);
            let start = self.to_canvas_coordinates(start, viewport);
            let end = Point2D::new(max_x, y);
            let end = self.to_canvas_coordinates(end, viewport);

            let shape = kurbo::Line::new(start.to_tuple(), end.to_tuple());
            if y == 0. {
                self.backend.stroke(shape, &style.stroke, 1.);
            } else {
                self.backend.stroke(shape, &style.stroke, stroke_width);
            }

            y += grid.grid_spacing.0;
        }
    }

    /// Translates a [`crate::Point`] from drawing space to a location in
    /// [`CanvasSpace`].
    fn to_canvas_coordinates(
        &self,
        point: Point2D<f64, DrawingSpace>,
        viewport: &Viewport,
    ) -> Point2D<f64, CanvasSpace> {
        super::to_canvas_coordinates(point, viewport, self.window_size)
    }
}

impl<'window, 'world, B: RenderContext> System<'world>
    for RenderSystem<'window, B>
{
    type SystemData = (
        DrawOrder<'world>,
        Styling<'world>,
        ReadStorage<'world, Viewport>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (draw_order, styling, viewports) = data;

        let window_style = self.window.style(&styling.window_styles);
        let viewport = self.window.viewport(&viewports);

        // make sure we're working with a blank screen
        self.backend.clear(window_style.background_colour.clone());

        let viewport_dimensions = self.viewport_dimensions(&viewport);

        for (ent, obj, layer) in draw_order.calculate(viewport_dimensions) {
            self.render(ent, obj, layer, &styling, viewport);
        }
    }
}

/// Styling information.
#[derive(SystemData)]
struct Styling<'world> {
    point_styles: ReadStorage<'world, PointStyle>,
    line_styles: ReadStorage<'world, LineStyle>,
    grid_styles: ReadStorage<'world, GridStyle>,
    window_styles: ReadStorage<'world, WindowStyle>,
}

fn resolve_point_style<'a>(
    styling: &'a Styling,
    window: &'a Window,
    point: Entity,
    layer: Entity,
) -> &'a PointStyle {
    styling
            .point_styles
            // the style for this point may have been overridden explicitly
            .get(point)
            // otherwise fall back to the layer's PointStyle
            .or_else(|| styling.point_styles.get(layer))
            // fall back to the window's default if the layer didn't specify one
            .unwrap_or_else(|| window.default_point_style(&styling.point_styles))
}

fn resolve_line_style<'a>(
    styling: &'a Styling,
    window: &'a Window,
    line: Entity,
    layer: Entity,
) -> &'a LineStyle {
    styling
        .line_styles
        .get(line)
        .or_else(|| styling.line_styles.get(layer))
        .unwrap_or_else(|| window.default_line_style(&styling.line_styles))
}

fn resolve_grid_style<'a>(
    styling: &'a Styling,
    window: &'a Window,
    grid: Entity,
    layer: Entity,
) -> &'a GridStyle {
    styling
        .grid_styles
        .get(grid)
        .or_else(|| styling.grid_styles.get(layer))
        .unwrap_or_else(|| window.default_grid_style(&styling.grid_styles))
}

/// The state needed when calculating which order to draw things in so z-levels
/// are implemented correctly.
#[derive(SystemData)]
struct DrawOrder<'world> {
    entities: Entities<'world>,
    drawing_objects: ReadStorage<'world, DrawingObject>,
    layers: ReadStorage<'world, Layer>,
    bounding_boxes: ReadStorage<'world, BoundingBox<DrawingSpace>>,
}

impl<'world> DrawOrder<'world> {
    fn calculate(
        &self,
        viewport_dimensions: BoundingBox<DrawingSpace>,
    ) -> impl Iterator<Item = (Entity, &'_ DrawingObject, &'_ Layer)> + '_ {
        type EntitiesByZLevel<'a> = BTreeMap<
            Reverse<usize>,
            Vec<(Entity, &'a DrawingObject, &'a Layer)>,
        >;

        // Iterate through all drawing objects, grouping them by the parent
        // layer's z-level in reverse order (we want to yield higher z-levels
        // first)
        let mut drawing_objects = EntitiesByZLevel::new();

        // PERF: This function has a massive impact on render times
        // Some ideas:
        //   - Use a pre-calculated quad-tree so we just need to check items
        //     within the viewport bounds
        //   - use a entities-to-layers cache so we can skip checking whether to
        //     draw an object on a hidden layer

        for (ent, obj, bounds) in (
            &self.entities,
            &self.drawing_objects,
            MaybeJoin(&self.bounding_boxes),
        )
            .join()
        {
            let layer = self
                .layers
                .get(obj.layer)
                .expect("The object's layer was deleted");

            // try to use the cached bounds, otherwise re-calculate them
            let bounds = bounds
                .copied()
                .unwrap_or_else(|| obj.geometry.bounding_box());

            if layer.visible && viewport_dimensions.intersects_with(bounds)
                || layer.layer_type == LayerType::System
            {
                drawing_objects
                    .entry(Reverse(layer.z_level))
                    .or_default()
                    .push((ent, obj, layer));
            }
        }

        drawing_objects.into_iter().flat_map(|(_, items)| items)
    }
}
