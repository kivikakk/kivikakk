use std::{cell::RefCell, error::Error};

use comrak::{
    format_commonmark,
    nodes::{Ast, AstNode, LineColumn, NodeLink, NodeList, NodeValue},
    parse_document, Arena, Options,
};
use yaml_rust2::YamlLoader;

fn mknode<'a>(arena: &'a Arena<AstNode<'a>>, sp: LineColumn, value: NodeValue) -> &'a AstNode<'a> {
    arena.alloc(AstNode::new(RefCell::new(Ast::new(value, sp))))
}

fn main() -> Result<(), Box<dyn Error>> {
    let arena = Arena::new();

    let doc = std::fs::read_to_string("README.base.md")?;
    let root = parse_document(&arena, &doc, &Options::default());

    for node in root.children() {
        let ast = node.data.borrow_mut();
        let sp = ast.sourcepos.start;

        if let NodeValue::CodeBlock(ref ncb) = ast.value {
            if ncb.info == "yaml" {
                let yaml = YamlLoader::load_from_str(&ncb.literal)?;

                let list = mknode(&arena, sp, NodeValue::List(NodeList::default()));
                for (title, detail) in yaml[0].as_hash().unwrap() {
                    let item = mknode(&arena, sp, NodeValue::Item(NodeList::default()));

                    let link = mknode(
                        &arena,
                        sp,
                        NodeValue::Link(NodeLink {
                            url: detail["url"].as_str().unwrap().to_string(),
                            title: "".to_string(),
                        }),
                    );
                    item.append(link);
                    link.append(mknode(
                        &arena,
                        sp,
                        NodeValue::Text(title.as_str().unwrap().to_string()),
                    ));

                    for tag in detail["tags"].as_vec().unwrap() {
                        item.append(mknode(&arena, sp, NodeValue::Text(" ".to_string())));

                        let inline = mknode(
                            &arena,
                            sp,
                            NodeValue::HtmlInline(format!("<kbd>{}</kbd>", tag.as_str().unwrap())),
                        );
                        item.append(inline);
                    }
                    item.append(mknode(&arena, sp, NodeValue::Text(" -- ".to_string())));

                    let description_doc = parse_document(
                        &arena,
                        detail["description"].as_str().unwrap(),
                        &Options::default(),
                    );
                    for description_node in description_doc.first_child().unwrap().children() {
                        item.append(description_node);
                    }

                    list.append(item);
                }

                node.insert_before(list);
                let blank = mknode(&arena, sp, NodeValue::LineBreak);
                node.insert_after(blank);

                node.detach();
            }
        }
    }

    let mut options = Options::default();
    options.render.list_style = comrak::ListStyleType::Star;

    format_commonmark(root, &options, &mut std::io::stdout().lock())?;

    Ok(())
}
