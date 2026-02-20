use std::collections::HashMap;

use typst_syntax::{LinkedNode, SyntaxNode};

pub type OffsetMap = HashMap<*const SyntaxNode, usize>;
pub type Range = Option<[usize; 2]>;

pub fn build_offset_map(node: &LinkedNode, map: &mut OffsetMap) {
    map.insert(node.get() as *const SyntaxNode, node.offset());
    for child in node.children() {
        build_offset_map(&child, map);
    }
}

pub fn range_of(node: &SyntaxNode, offsets: &OffsetMap) -> Range {
    offsets
        .get(&(node as *const SyntaxNode))
        .copied()
        .map(|offset| [offset, offset + node.len()])
}
