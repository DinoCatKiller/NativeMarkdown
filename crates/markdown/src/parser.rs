use crate::BlockNode;
use crate::Document;
use crate::InlineNode;

pub fn parse_markdown(markdown: &str) -> Document {
    if markdown.trim().is_empty() {
        return Document { children: vec![] };
    }
    Document {
        children: vec![BlockNode::Paragraph {
            children: markdown
                .trim()
                .split("\n\n")
                .map(|x| InlineNode::Text {
                    value: x.to_string(),
                })
                .collect(),
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*; // 把外面 parse_markdown、Document 等引入进来

    #[test]
    fn test_empty() {
        let doc = parse_markdown("");
        assert_eq!(doc.children, vec![]);
    }

    #[test]
    fn test_whitespace_only() {
        let doc = parse_markdown("   \n\n  \t ");
        assert_eq!(doc.children, vec![]);
    }

    #[test]
    fn test_single_paragraph() {
        let doc = parse_markdown("hello world");
        assert_eq!(
            doc,
            Document {
                children: vec![BlockNode::Paragraph {
                    children: vec![InlineNode::Text {
                        value: "hello world".to_string()
                    }],
                }],
            }
        );
    }

    #[test]
    fn test_two_paragraphs() {
        let doc = parse_markdown("first\n\nsecond");
        assert_eq!(
            doc,
            Document {
                children: vec![BlockNode::Paragraph {
                    children: vec![
                        InlineNode::Text {
                            value: "first".to_string()
                        },
                        InlineNode::Text {
                            value: "second".to_string()
                        },
                    ],
                }],
            }
        );
    }
}

// - 输入空字符串时，返回一个有效的空文档。
// - 输入普通文本时，返回一个包含段落和文本节点的文档。
// - 两个由空行分隔的文本块，应解析为两个段落。
// - 文本内容应保留，不应被静默修改。
// - 当前阶段不需要返回 `Result`，因为还没有定义解析失败的情况。

// .iter().map(|x| InlineNode::Text { value:x.to_string() }).collect()
// let paragraphs: Vec<&str> = markdown.trim().split("\n\n").collect();
// return Document{children:vec![BlockNode::Paragraph { children: vec![markdown.trim().split("\n\n").collect() ].iter().map(|x| InlineNode::Text { value:x.to_string() }).collect()}]};
