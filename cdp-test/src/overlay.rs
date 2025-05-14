pub use crate::common::*;
use crate::dom::*;
use crate::page::*;
/// Configuration data for drawing the source order of an elements children.
pub struct SourceOrderConfig {
    pub parent_outline_color: (),
    pub child_outline_color: (),
}
/// Configuration data for the highlighting of Grid elements.
pub struct GridHighlightConfig {
    pub show_grid_extension_lines: (),
    pub show_positive_line_numbers: (),
    pub show_negative_line_numbers: (),
    pub show_area_names: (),
    pub show_line_names: (),
    pub show_track_sizes: (),
    pub grid_border_color: (),
    pub cell_border_color: (),
    pub row_line_color: (),
    pub column_line_color: (),
    pub grid_border_dash: (),
    pub cell_border_dash: (),
    pub row_line_dash: (),
    pub column_line_dash: (),
    pub row_gap_color: (),
    pub row_hatch_color: (),
    pub column_gap_color: (),
    pub column_hatch_color: (),
    pub area_border_color: (),
    pub grid_background_color: (),
}
/// Configuration data for the highlighting of Flex container elements.
pub struct FlexContainerHighlightConfig {
    pub container_border: (),
    pub line_separator: (),
    pub item_separator: (),
    pub main_distributed_space: (),
    pub cross_distributed_space: (),
    pub row_gap_space: (),
    pub column_gap_space: (),
    pub cross_alignment: (),
}
/// Configuration data for the highlighting of Flex item elements.
pub struct FlexItemHighlightConfig {
    pub base_size_box: (),
    pub base_size_border: (),
    pub flexibility_arrow: (),
}
/// Style information for drawing a line.
pub struct LineStyle {
    pub color: (),
    pub pattern: String,
}
/// Style information for drawing a box.
pub struct BoxStyle {
    pub fill_color: (),
    pub hatch_color: (),
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
    pub content_color: (),
    pub padding_color: (),
    pub border_color: (),
    pub margin_color: (),
    pub event_target_color: (),
    pub shape_color: (),
    pub shape_margin_color: (),
    pub css_grid_color: (),
    pub color_format: (),
    pub grid_highlight_config: (),
    pub flex_container_highlight_config: (),
    pub flex_item_highlight_config: (),
    pub contrast_algorithm: (),
    pub container_query_container_highlight_config: (),
}
pub enum ColorFormat {
    Rgb,
    Hsl,
    Hwb,
    Hex,
}
/// Configurations for Persistent Grid Highlight
pub struct GridNodeHighlightConfig {
    pub grid_highlight_config: (),
    pub node_id: (),
}
pub struct FlexNodeHighlightConfig {
    pub flex_container_highlight_config: (),
    pub node_id: (),
}
pub struct ScrollSnapContainerHighlightConfig {
    pub snapport_border: (),
    pub snap_area_border: (),
    pub scroll_margin_color: (),
    pub scroll_padding_color: (),
}
pub struct ScrollSnapHighlightConfig {
    pub scroll_snap_container_highlight_config: (),
    pub node_id: (),
}
/// Configuration for dual screen hinge
pub struct HingeConfig {
    pub rect: (),
    pub content_color: (),
    pub outline_color: (),
}
/// Configuration for Window Controls Overlay
pub struct WindowControlsOverlayConfig {
    pub show_css: (),
    pub selected_platform: String,
    pub theme_color: String,
}
pub struct ContainerQueryHighlightConfig {
    pub container_query_container_highlight_config: (),
    pub node_id: (),
}
pub struct ContainerQueryContainerHighlightConfig {
    pub container_border: (),
    pub descendant_border: (),
}
pub struct IsolatedElementHighlightConfig {
    pub isolation_mode_highlight_config: (),
    pub node_id: (),
}
pub struct IsolationModeHighlightConfig {
    pub resizer_color: (),
    pub resizer_handle_color: (),
    pub mask_color: (),
}
pub enum InspectMode {
    SearchForNode,
    SearchForUaShadowDom,
    CaptureAreaScreenshot,
    ShowDistances,
    None,
}
