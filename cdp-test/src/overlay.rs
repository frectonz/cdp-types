use crate::common::*;
use crate::dom::*;
use crate::page::*;
/// Configuration data for drawing the source order of an elements children.
pub struct SourceOrderConfig {
    pub parent_outline_color: Box<Rgba>,
    pub child_outline_color: Box<Rgba>,
}
/// Configuration data for the highlighting of Grid elements.
pub struct GridHighlightConfig {
    pub show_grid_extension_lines: bool,
    pub show_positive_line_numbers: bool,
    pub show_negative_line_numbers: bool,
    pub show_area_names: bool,
    pub show_line_names: bool,
    pub show_track_sizes: bool,
    pub grid_border_color: Box<Rgba>,
    pub cell_border_color: Box<Rgba>,
    pub row_line_color: Box<Rgba>,
    pub column_line_color: Box<Rgba>,
    pub grid_border_dash: bool,
    pub cell_border_dash: bool,
    pub row_line_dash: bool,
    pub column_line_dash: bool,
    pub row_gap_color: Box<Rgba>,
    pub row_hatch_color: Box<Rgba>,
    pub column_gap_color: Box<Rgba>,
    pub column_hatch_color: Box<Rgba>,
    pub area_border_color: Box<Rgba>,
    pub grid_background_color: Box<Rgba>,
}
/// Configuration data for the highlighting of Flex container elements.
pub struct FlexContainerHighlightConfig {
    pub container_border: Box<LineStyle>,
    pub line_separator: Box<LineStyle>,
    pub item_separator: Box<LineStyle>,
    pub main_distributed_space: Box<BoxStyle>,
    pub cross_distributed_space: Box<BoxStyle>,
    pub row_gap_space: Box<BoxStyle>,
    pub column_gap_space: Box<BoxStyle>,
    pub cross_alignment: Box<LineStyle>,
}
/// Configuration data for the highlighting of Flex item elements.
pub struct FlexItemHighlightConfig {
    pub base_size_box: Box<BoxStyle>,
    pub base_size_border: Box<LineStyle>,
    pub flexibility_arrow: Box<LineStyle>,
}
/// Style information for drawing a line.
pub struct LineStyle {
    pub color: Box<Rgba>,
    pub pattern: String,
}
/// Style information for drawing a box.
pub struct BoxStyle {
    pub fill_color: Box<Rgba>,
    pub hatch_color: Box<Rgba>,
}
pub enum ContrastAlgorithm {
    Aa,
    Aaa,
    Apca,
}
/// Configuration data for the highlighting of page elements.
pub struct HighlightConfig {
    pub show_info: bool,
    pub show_styles: bool,
    pub show_rulers: bool,
    pub show_accessibility_info: bool,
    pub show_extension_lines: bool,
    pub content_color: Box<Rgba>,
    pub padding_color: Box<Rgba>,
    pub border_color: Box<Rgba>,
    pub margin_color: Box<Rgba>,
    pub event_target_color: Box<Rgba>,
    pub shape_color: Box<Rgba>,
    pub shape_margin_color: Box<Rgba>,
    pub css_grid_color: Box<Rgba>,
    pub color_format: Box<ColorFormat>,
    pub grid_highlight_config: Box<GridHighlightConfig>,
    pub flex_container_highlight_config: Box<FlexContainerHighlightConfig>,
    pub flex_item_highlight_config: Box<FlexItemHighlightConfig>,
    pub contrast_algorithm: Box<ContrastAlgorithm>,
    pub container_query_container_highlight_config: Box<
        ContainerQueryContainerHighlightConfig,
    >,
}
pub enum ColorFormat {
    Rgb,
    Hsl,
    Hwb,
    Hex,
}
/// Configurations for Persistent Grid Highlight
pub struct GridNodeHighlightConfig {
    pub grid_highlight_config: Box<GridHighlightConfig>,
    pub node_id: Box<NodeId>,
}
pub struct FlexNodeHighlightConfig {
    pub flex_container_highlight_config: Box<FlexContainerHighlightConfig>,
    pub node_id: Box<NodeId>,
}
pub struct ScrollSnapContainerHighlightConfig {
    pub snapport_border: Box<LineStyle>,
    pub snap_area_border: Box<LineStyle>,
    pub scroll_margin_color: Box<Rgba>,
    pub scroll_padding_color: Box<Rgba>,
}
pub struct ScrollSnapHighlightConfig {
    pub scroll_snap_container_highlight_config: Box<ScrollSnapContainerHighlightConfig>,
    pub node_id: Box<NodeId>,
}
/// Configuration for dual screen hinge
pub struct HingeConfig {
    pub rect: Box<Rect>,
    pub content_color: Box<Rgba>,
    pub outline_color: Box<Rgba>,
}
/// Configuration for Window Controls Overlay
pub struct WindowControlsOverlayConfig {
    pub show_css: bool,
    pub selected_platform: String,
    pub theme_color: String,
}
pub struct ContainerQueryHighlightConfig {
    pub container_query_container_highlight_config: Box<
        ContainerQueryContainerHighlightConfig,
    >,
    pub node_id: Box<NodeId>,
}
pub struct ContainerQueryContainerHighlightConfig {
    pub container_border: Box<LineStyle>,
    pub descendant_border: Box<LineStyle>,
}
pub struct IsolatedElementHighlightConfig {
    pub isolation_mode_highlight_config: Box<IsolationModeHighlightConfig>,
    pub node_id: Box<NodeId>,
}
pub struct IsolationModeHighlightConfig {
    pub resizer_color: Box<Rgba>,
    pub resizer_handle_color: Box<Rgba>,
    pub mask_color: Box<Rgba>,
}
pub enum InspectMode {
    SearchForNode,
    SearchForUaShadowDom,
    CaptureAreaScreenshot,
    ShowDistances,
    None,
}
pub type OverlayDisable = ();
pub type OverlayEnable = ();
pub type OverlayGetHighlightObjectForTest = ();
pub type OverlayGetGridHighlightObjectsForTest = ();
pub type OverlayGetSourceOrderHighlightObjectForTest = ();
pub type OverlayHideHighlight = ();
pub type OverlayHighlightFrame = ();
pub type OverlayHighlightNode = ();
pub type OverlayHighlightQuad = ();
pub type OverlayHighlightRect = ();
pub type OverlayHighlightSourceOrder = ();
pub type OverlaySetInspectMode = ();
pub type OverlaySetShowAdHighlights = ();
pub type OverlaySetPausedInDebuggerMessage = ();
pub type OverlaySetShowDebugBorders = ();
pub type OverlaySetShowFpsCounter = ();
pub type OverlaySetShowGridOverlays = ();
pub type OverlaySetShowFlexOverlays = ();
pub type OverlaySetShowScrollSnapOverlays = ();
pub type OverlaySetShowContainerQueryOverlays = ();
pub type OverlaySetShowPaintRects = ();
pub type OverlaySetShowLayoutShiftRegions = ();
pub type OverlaySetShowScrollBottleneckRects = ();
pub type OverlaySetShowHitTestBorders = ();
pub type OverlaySetShowWebVitals = ();
pub type OverlaySetShowViewportSizeOnResize = ();
pub type OverlaySetShowHinge = ();
pub type OverlaySetShowIsolatedElements = ();
pub type OverlaySetShowWindowControlsOverlay = ();
