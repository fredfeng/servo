use crate::context::LayoutContext;
use crate::flow::{Flow, GetBaseFlow};
use crate::incremental::RelayoutMode;
use crate::traversal::{preorder, postorder};

pub fn reflow(root: &mut dyn Flow, layout_context: &LayoutContext, relayout_mode: RelayoutMode) {
    postorder(root, &|flow| flow.bubble_inline_sizes());
    preorder(root, &|flow| flow.assign_inline_sizes(layout_context));
    postorder(root, &|flow| flow.assign_block_size(layout_context));
    preorder(root, &|flow| flow.compute_stacking_relative_position(layout_context));
}
