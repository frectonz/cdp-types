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
/// Disables domain notifications.
pub type OverlayDisable = ();
/// Enables domain notifications.
pub type OverlayEnable = ();
/// For testing.
pub type OverlayGetHighlightObjectForTest = ();
/// For Persistent Grid testing.
pub type OverlayGetGridHighlightObjectsForTest = ();
/// For Source Order Viewer testing.
pub type OverlayGetSourceOrderHighlightObjectForTest = ();
/// Hides any highlight.
pub type OverlayHideHighlight = ();
#[deprecated]
/** Highlights owner element of the frame with given id.
Deprecated: Doesn't work reliably and cannot be fixed due to process
separation (the owner node might be in a different process). Determine
the owner node in the client and use highlightNode.*/
pub type OverlayHighlightFrame = ();
/** Highlights DOM node with given id or with the given JavaScript object wrapper. Either nodeId or
objectId must be specified.*/
pub type OverlayHighlightNode = ();
/// Highlights given quad. Coordinates are absolute with respect to the main frame viewport.
pub type OverlayHighlightQuad = ();
/// Highlights given rectangle. Coordinates are absolute with respect to the main frame viewport.
pub type OverlayHighlightRect = ();
/** Highlights the source order of the children of the DOM node with given id or with the given
JavaScript object wrapper. Either nodeId or objectId must be specified.*/
pub type OverlayHighlightSourceOrder = ();
/** Enters the 'inspect' mode. In this mode, elements that user is hovering over are highlighted.
Backend then generates 'inspectNodeRequested' event upon element selection.*/
pub type OverlaySetInspectMode = ();
/// Highlights owner element of all frames detected to be ads.
pub type OverlaySetShowAdHighlights = ();
pub type OverlaySetPausedInDebuggerMessage = ();
/// Requests that backend shows debug borders on layers
pub type OverlaySetShowDebugBorders = ();
/// Requests that backend shows the FPS counter
pub type OverlaySetShowFpsCounter = ();
/// Highlight multiple elements with the CSS Grid overlay.
pub type OverlaySetShowGridOverlays = ();
pub type OverlaySetShowFlexOverlays = ();
pub type OverlaySetShowScrollSnapOverlays = ();
pub type OverlaySetShowContainerQueryOverlays = ();
/// Requests that backend shows paint rectangles
pub type OverlaySetShowPaintRects = ();
/// Requests that backend shows layout shift regions
pub type OverlaySetShowLayoutShiftRegions = ();
/// Requests that backend shows scroll bottleneck rects
pub type OverlaySetShowScrollBottleneckRects = ();
#[deprecated]
/// Deprecated, no longer has any effect.
pub type OverlaySetShowHitTestBorders = ();
#[deprecated]
/// Deprecated, no longer has any effect.
pub type OverlaySetShowWebVitals = ();
/// Paints viewport size upon main frame resize.
pub type OverlaySetShowViewportSizeOnResize = ();
/// Add a dual screen device hinge
pub type OverlaySetShowHinge = ();
/// Show elements in isolation mode with overlays.
pub type OverlaySetShowIsolatedElements = ();
/// Show Window Controls Overlay for PWA
pub type OverlaySetShowWindowControlsOverlay = ();
