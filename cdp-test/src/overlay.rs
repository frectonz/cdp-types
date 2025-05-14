use crate::common::*;
use crate::dom::*;
use crate::page::*;
/// Configuration data for drawing the source order of an elements children.
pub struct SourceOrderConfig {
    pub parent_outline_color: Box<DomRgba>,
    pub child_outline_color: Box<DomRgba>,
}
/// Configuration data for the highlighting of Grid elements.
pub struct GridHighlightConfig {
    pub show_grid_extension_lines: (),
    pub show_positive_line_numbers: (),
    pub show_negative_line_numbers: (),
    pub show_area_names: (),
    pub show_line_names: (),
    pub show_track_sizes: (),
    pub grid_border_color: Box<DomRgba>,
    pub cell_border_color: Box<DomRgba>,
    pub row_line_color: Box<DomRgba>,
    pub column_line_color: Box<DomRgba>,
    pub grid_border_dash: (),
    pub cell_border_dash: (),
    pub row_line_dash: (),
    pub column_line_dash: (),
    pub row_gap_color: Box<DomRgba>,
    pub row_hatch_color: Box<DomRgba>,
    pub column_gap_color: Box<DomRgba>,
    pub column_hatch_color: Box<DomRgba>,
    pub area_border_color: Box<DomRgba>,
    pub grid_background_color: Box<DomRgba>,
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
    pub color: Box<DomRgba>,
    pub pattern: Box<String>,
}
/// Style information for drawing a box.
pub struct BoxStyle {
    pub fill_color: Box<DomRgba>,
    pub hatch_color: Box<DomRgba>,
}
pub enum ContrastAlgorithm {
    Aa,
    Aaa,
    Apca,
}
/// Configuration data for the highlighting of page elements.
pub struct HighlightConfig {
    pub show_info: (),
    pub show_styles: (),
    pub show_rulers: (),
    pub show_accessibility_info: (),
    pub show_extension_lines: (),
    pub content_color: Box<DomRgba>,
    pub padding_color: Box<DomRgba>,
    pub border_color: Box<DomRgba>,
    pub margin_color: Box<DomRgba>,
    pub event_target_color: Box<DomRgba>,
    pub shape_color: Box<DomRgba>,
    pub shape_margin_color: Box<DomRgba>,
    pub css_grid_color: Box<DomRgba>,
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
    pub node_id: Box<DomNodeId>,
}
pub struct FlexNodeHighlightConfig {
    pub flex_container_highlight_config: Box<FlexContainerHighlightConfig>,
    pub node_id: Box<DomNodeId>,
}
pub struct ScrollSnapContainerHighlightConfig {
    pub snapport_border: Box<LineStyle>,
    pub snap_area_border: Box<LineStyle>,
    pub scroll_margin_color: Box<DomRgba>,
    pub scroll_padding_color: Box<DomRgba>,
}
pub struct ScrollSnapHighlightConfig {
    pub scroll_snap_container_highlight_config: Box<ScrollSnapContainerHighlightConfig>,
    pub node_id: Box<DomNodeId>,
}
/// Configuration for dual screen hinge
pub struct HingeConfig {
    pub rect: Box<DomRect>,
    pub content_color: Box<DomRgba>,
    pub outline_color: Box<DomRgba>,
}
/// Configuration for Window Controls Overlay
pub struct WindowControlsOverlayConfig {
    pub show_css: (),
    pub selected_platform: Box<String>,
    pub theme_color: Box<String>,
}
pub struct ContainerQueryHighlightConfig {
    pub container_query_container_highlight_config: Box<
        ContainerQueryContainerHighlightConfig,
    >,
    pub node_id: Box<DomNodeId>,
}
pub struct ContainerQueryContainerHighlightConfig {
    pub container_border: Box<LineStyle>,
    pub descendant_border: Box<LineStyle>,
}
pub struct IsolatedElementHighlightConfig {
    pub isolation_mode_highlight_config: Box<IsolationModeHighlightConfig>,
    pub node_id: Box<DomNodeId>,
}
pub struct IsolationModeHighlightConfig {
    pub resizer_color: Box<DomRgba>,
    pub resizer_handle_color: Box<DomRgba>,
    pub mask_color: Box<DomRgba>,
}
pub enum InspectMode {
    SearchForNode,
    SearchForUaShadowDom,
    CaptureAreaScreenshot,
    ShowDistances,
    None,
}
