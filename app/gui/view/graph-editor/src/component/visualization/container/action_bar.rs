//! Definition of the `ActionBar` component for the `visualization::Container`.

use crate::prelude::*;
use ensogl::display::shape::*;
use ensogl::display::traits::*;

use crate::component::node;
use crate::component::visualization;
use crate::component::visualization::container::visualization_chooser::VisualizationChooser;
use crate::data::enso;

use enso_frp as frp;
use enso_frp;
use ensogl::application::Application;
use ensogl::data::color;
use ensogl::display;
use ensogl::display::shape::system::Shape;
use ensogl::gui::component::ShapeView;
use ensogl_component::drop_down_menu;
use ensogl_hardcoded_theme as theme;



// =================
// === Constants ===
// =================

const HOVER_COLOR: color::Rgba = color::Rgba::new(1.0, 0.0, 0.0, 0.000_001);
/// Gap between action bar and selection menu
const MENU_GAP: f32 = 5.0;
const ACTION_ICON_SIZE: f32 = 20.0;



// ===============
// === Shapes  ===
// ===============

/// Invisible rectangular area that can be hovered.
mod hover_area {
    use super::*;

    ensogl::shape! {
        below = [drop_down_menu::arrow];
        alignment = center;
        (style: Style) {
            let width  : Var<Pixels> = "input_size.x".into();
            let height : Var<Pixels> = "input_size.y".into();
            let background           = Rect((&width,&height));
            let background           = background.fill(HOVER_COLOR);
            background.into()
        }
    }
}

/// Background of the action bar.
/// Note: needs to be an extra shape for sorting purposes.
mod background {
    use super::*;

    ensogl::shape! {
        below = [hover_area];
        alignment = center;
        (style:Style) {
            let width              = Var::<Pixels>::from("input_size.x");
            let height             = Var::<Pixels>::from("input_size.y");
            let radius             = node::RADIUS.px() ;
            let background_rounded = Rect((&width,&height)).corners_radius(radius);
            let background_sharp   = Rect((&width,&height/2.0)).translate_y(-&height/4.0);
            let background         = background_rounded + background_sharp;
            let color_path         = theme::graph_editor::visualization::action_bar::background;
            let fill_color         = style.get_color(color_path);
            let background         = background.fill(fill_color);
            background.into()
        }
    }
}



// ========================
// === Action Bar Icons ===
// ========================

/// Icon that appears as four arrows pointing in each direction: up, down, left and right.
mod four_arrow_icon {
    use super::*;

    use std::f32::consts::PI;
    const ARROW_LINE_WIDTH: f32 = 1.0;

    ensogl::shape! {
        alignment = center;
        (style:Style) {
            let width      = Var::<Pixels>::from("input_size.x");
            let height     = Var::<Pixels>::from("input_size.y");
            let background = Rect((&width,&height)).fill(HOVER_COLOR);

            let horizontal_bar_height = &height / 2.0;
            let horizontal_bar        = Rect((&horizontal_bar_height,ARROW_LINE_WIDTH.px()));
            let vertical_bar          = horizontal_bar.rotate((PI/2.0).radians());
            let cross                 = horizontal_bar + vertical_bar;

            let arrow_head_size   = (ARROW_LINE_WIDTH * 3.0).floor();
            let arrow_head_offset = &horizontal_bar_height / 2.0;
            let arrow_head        = Rect((arrow_head_size.px(),&arrow_head_size.px())).rotate((PI/4.0).radians());
            let split_plane       = HalfPlane();
            let arrow_head        = arrow_head.difference(split_plane);
            let arrow_head        = arrow_head.translate_y(arrow_head_offset);

            let arrow_heads = &arrow_head;
            let arrow_heads = arrow_heads + arrow_head.rotate(2.0 * (PI/4.0).radians());
            let arrow_heads = arrow_heads + arrow_head.rotate(4.0 * (PI/4.0).radians());
            let arrow_heads = arrow_heads + arrow_head.rotate(6.0 * (PI/4.0).radians());

            let color_path = theme::graph_editor::visualization::action_bar::icon;
            let fill_color = style.get_color(color_path);
            let icon       = (arrow_heads + cross).fill(fill_color);

            (background + icon).into()
        }
    }
}

/// Icon that appears as a pin with a head and a point, slanted so the pin points towards
/// the bottom left.
mod pin_icon {
    use super::*;

    use std::f32::consts::PI;
    const PIN_THORN_WIDTH: f32 = 1.0;

    ensogl::shape! {
        alignment = center;
        (style:Style) {
            let width      = Var::<Pixels>::from("input_size.x");
            let height     = Var::<Pixels>::from("input_size.y");
            let background = Rect((&width,&height)).fill(HOVER_COLOR);

            let pin_head_size = &height / 3.0;
            let pin_head_base = Rect((&pin_head_size,&pin_head_size));
            let pin_head_top  = Triangle(&pin_head_size * 1.5,&pin_head_size / 2.0) ;
            let pin_head_top  = pin_head_top.translate_y(-&pin_head_size/2.0);
            let pin_head      = (pin_head_base + pin_head_top).translate_y(&pin_head_size/2.0);

            let pin_thorn_size  = &height / 3.0;
            let pin_thorn_width = PIN_THORN_WIDTH.px();
            let pin_thorn       = Triangle(pin_thorn_width,pin_thorn_size);
            let pin_thorn       = pin_thorn.rotate((PI).radians());
            let pin_thorn       = pin_thorn.translate_y(-&pin_head_size/2.0);

            let color_path = theme::graph_editor::visualization::action_bar::icon;
            let fill_color = style.get_color(color_path);
            let icon       = (pin_thorn + pin_head).fill(fill_color);
            let icon       = icon.rotate((PI/4.0).radians());
            let icon       = icon.fill(fill_color);

            (background + icon).into()
        }
    }
}

#[derive(Clone, CloneRef, Debug)]
struct Icons {
    display_object:      display::object::Instance,
    icon_root:           display::object::Instance,
    reset_position_icon: pin_icon::View,
    drag_icon:           four_arrow_icon::View,
    size:                Rc<Cell<Vector2>>,
}

impl Icons {
    fn new() -> Self {
        let display_object = display::object::Instance::new();
        let icon_root = display::object::Instance::new();
        let reset_position_icon = pin_icon::View::new();
        let drag_icon = four_arrow_icon::View::new();
        let size = default();

        display_object.add_child(&icon_root);
        icon_root.add_child(&reset_position_icon);
        icon_root.add_child(&drag_icon);
        Self { display_object, icon_root, reset_position_icon, drag_icon, size }.init_layout()
    }

    fn place_shape_in_slot<S: Shape>(&self, view: &ShapeView<S>, index: usize) {
        let icon_size = self.icon_size();
        let index = index as f32;
        view.set_x(index * icon_size.x + node::CORNER_RADIUS);
        view.set_size(icon_size);
    }

    fn icon_size(&self) -> Vector2 {
        Vector2::new(ACTION_ICON_SIZE, ACTION_ICON_SIZE)
    }

    fn init_layout(self) -> Self {
        self.place_shape_in_slot(&self.drag_icon, 0);
        self.place_shape_in_slot(&self.reset_position_icon, 1);
        self.set_reset_icon_visibility(false);
        self
    }

    fn update_size(&self, size: Vector2) {
        self.display_object().set_size(size);
        self.icon_root.set_x(-size.x / 2.0);
        self.place_shape_in_slot(&self.drag_icon, 0);
        self.place_shape_in_slot(&self.reset_position_icon, 1);
    }

    fn set_reset_icon_visibility(&self, visibility: bool) {
        if visibility {
            self.icon_root.add_child(&self.reset_position_icon)
        } else {
            self.reset_position_icon.unset_parent()
        }
    }
}

impl display::Object for Icons {
    fn display_object(&self) -> &display::object::Instance {
        &self.display_object
    }
}



// ===========
// === Frp ===
// ===========

ensogl::define_endpoints! {
    Input {
        set_size                   (Vector2),
        show_icons                 (),
        hide_icons                 (),
        set_selected_visualization (Option<visualization::Path>),
        set_vis_input_type         (Option<enso::Type>),
    }

    Output {
        visualisation_selection     (Option<visualization::Path>),
        mouse_over                  (),
        mouse_out                   (),
        on_container_reset_position (),
        /// Indicates whether the container should follow the mouse cursor.
        container_drag_state        (bool),
    }
}



// ========================
// === Action Bar Model ===
// ========================

#[derive(Clone, CloneRef, Debug)]
struct Model {
    hover_area:            hover_area::View,
    visualization_chooser: VisualizationChooser,
    background:            background::View,
    display_object:        display::object::Instance,
    size:                  Rc<Cell<Vector2>>,
    icons:                 Icons,
    shapes:                compound::events::MouseEvents,
}

impl Model {
    fn new(app: &Application, vis_registry: visualization::Registry) -> Self {
        let background = background::View::new();
        let hover_area = hover_area::View::new();
        let visualization_chooser = VisualizationChooser::new(app, vis_registry);
        let display_object = display::object::Instance::new();
        let size = default();
        let icons = Icons::new();
        let shapes = compound::events::MouseEvents::default();

        app.display.default_scene.layers.above_nodes.add(&icons);

        shapes.add_sub_shape(&hover_area);
        shapes.add_sub_shape(&background);
        shapes.add_sub_shape(&icons.reset_position_icon);
        shapes.add_sub_shape(&icons.drag_icon);

        Model { hover_area, visualization_chooser, background, display_object, size, icons, shapes }
            .init()
    }

    fn init(self) -> Self {
        self.add_child(&self.hover_area);
        self
    }

    fn set_size(&self, size: Vector2) {
        self.display_object().set_size(size);
        self.hover_area.set_size(size);
        self.background.set_size(size);
        self.icons.update_size(size);

        let height = size.y;
        let width = size.x;
        let right_padding = height / 2.0;
        self.visualization_chooser.frp.set_icon_size(Vector2::new(height, height));
        self.visualization_chooser.frp.set_icon_padding(Vector2::new(height / 3.0, height / 3.0));
        self.visualization_chooser.set_x((width / 2.0) - right_padding);
        self.visualization_chooser.frp.set_menu_offset_y(MENU_GAP);
    }

    fn show(&self) {
        self.add_child(&self.background);
        self.add_child(&self.visualization_chooser);
        self.add_child(&self.icons);
    }

    fn hide(&self) {
        self.visualization_chooser.unset_parent();
        self.background.unset_parent();
        self.icons.unset_parent();
        self.visualization_chooser.frp.hide_selection_menu.emit(());
    }
}

impl display::Object for Model {
    fn display_object(&self) -> &display::object::Instance {
        &self.display_object
    }
}



// ==================
// === Action Bar ===
// ==================

/// UI for executing actions on a node. Consists of label indicating the active visualization
/// and a drop-down menu for selecting a new visualisation.
///
/// Layout
/// ------
/// ```text
///     / ---------------------------- \
///    |              <vis chooser> V   |
///    |--------------------------------|
/// ```
#[allow(missing_docs)]
#[derive(Clone, CloneRef, Debug)]
pub struct ActionBar {
    pub frp: Frp,
    model:   Rc<Model>,
}

impl ActionBar {
    /// Constructor.
    pub fn new(app: &Application, vis_registry: visualization::Registry) -> Self {
        let frp = Frp::new();
        let model = Rc::new(Model::new(app, vis_registry));
        ActionBar { frp, model }.init_frp(app)
    }

    fn init_frp(self, app: &Application) -> Self {
        let network = &self.frp.network;
        let frp = &self.frp;
        let model = &self.model;
        let mouse = &app.display.default_scene.mouse.frp_deprecated;
        let visualization_chooser = &model.visualization_chooser.frp;

        frp::extend! { network

            // === Input Processing ===

            eval  frp.set_size ((size) model.set_size(*size));
            eval_ frp.hide_icons ( model.hide() );
            eval_ frp.show_icons ( model.show() );

            visualization_chooser.input.set_selected <+ frp.input.set_selected_visualization;


            // === Mouse Interactions ===

            any_component_over <- any(&model.shapes.mouse_over,&visualization_chooser.mouse_over);
            any_component_out  <- any(&model.shapes.mouse_out,&visualization_chooser.mouse_out);

            is_over_true  <- any_component_over.constant(true);
            is_over_false <- any_component_out.constant(false);
            any_hovered   <- any(is_over_true,is_over_false);

            eval_ any_component_over (model.show());

            mouse_out_no_menu <- any_component_out.gate_not(&visualization_chooser.menu_visible);
            remote_click      <- visualization_chooser.menu_closed.gate_not(&any_hovered);
            hide              <- any(mouse_out_no_menu,remote_click);
            eval_ hide (model.hide());

            // The action bar does not allow to deselect the visualisation, so we prohibit these
            // events, which can occur on re-initialization.
            has_selection <- visualization_chooser.chosen_entry.is_some();
            frp.source.visualisation_selection
                <+ visualization_chooser.chosen_entry.gate(&has_selection);

            let reset_position_icon = &model.icons.reset_position_icon.events_deprecated;
            let reset_position_icon_down = reset_position_icon.mouse_down_primary.clone_ref();
            frp.source.on_container_reset_position <+ reset_position_icon_down;

            let drag_icon      = &model.icons.drag_icon.events_deprecated;
            let start_dragging = &drag_icon.mouse_down_primary;
            end_dragging       <- mouse.up.gate(&frp.source.container_drag_state);
            should_drag        <- bool(&end_dragging,start_dragging);
            frp.source.container_drag_state <+ should_drag;

            show_reset_icon <- bool(&reset_position_icon_down,start_dragging);
            eval show_reset_icon((visibility) model.icons.set_reset_icon_visibility(*visibility));


           // === Visualisation Chooser ===

            // Note: we only want to update the chooser if it is visible, or when it becomes
            // visible. During startup we get the type information for every node, and propagate
            // this information to the chooser. By delaying the creation of UI list elements until
            // the chooser is visible, we can avoid creating a lot of UI elements that are never
            // used.

            visible <- any(&frp.show_icons, &any_component_over);
            hidden <- any(&frp.hide_icons, &hide);

            is_visible <- bool(&hidden, &visible);
            on_show <- is_visible.on_change().on_true();

            set_vis_input_while_visible <- frp.input.set_vis_input_type.gate(&is_visible);
            set_vis_input_on_show       <- frp.input.set_vis_input_type.sample(&on_show);
            set_vis_input <- any(&set_vis_input_while_visible, &set_vis_input_on_show).on_change();
            visualization_chooser.input.set_vis_input_type <+ set_vis_input;
        }
        self
    }

    /// Visualization Chooser component getter.
    pub fn visualization_chooser(&self) -> &VisualizationChooser {
        &self.model.visualization_chooser
    }
}

impl display::Object for ActionBar {
    fn display_object(&self) -> &display::object::Instance {
        self.model.display_object()
    }
}
