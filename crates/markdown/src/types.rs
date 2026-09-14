#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub children: Vec<BlockNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockNode {
    Paragraph { children: Vec<InlineNode> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InlineNode {
    Text { value: String },
}
