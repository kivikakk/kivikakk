use std::error::Error;

use comrak::{
    format_commonmark,
    nodes::{NodeLink, NodeList, NodeValue},
    parse_document, Arena, Options,
};
use yaml_rust2::YamlLoader;

fn main() -> Result<(), Box<dyn Error>> {
    let arena = Arena::new();

    let doc = std::fs::read_to_string("README.base.md")?;
    let root = parse_document(&arena, &doc, &Options::default());

    for node in root.children() {
        let ast = node.data.borrow_mut();

        if let NodeValue::CodeBlock(ref ncb) = ast.value {
            if ncb.info == "yaml" {
                let yaml = YamlLoader::load_from_str(&ncb.literal)?;

                let list = arena.alloc(NodeValue::List(NodeList::default()).into());
                for (title, detail) in yaml[0].as_hash().unwrap() {
                    let item = arena.alloc(NodeValue::Item(NodeList::default()).into());

                    let link = arena.alloc(
                        NodeValue::Link(NodeLink {
                            url: detail["url"].as_str().unwrap().to_string(),
                            title: "".to_string(),
                        })
                        .into(),
                    );
                    item.append(link);
                    link.append(
                        arena.alloc(NodeValue::Text(title.as_str().unwrap().to_string()).into()),
                    );

                    for tag in detail["tags"].as_vec().unwrap() {
                        item.append(arena.alloc(NodeValue::Text(" ".to_string()).into()));

                        let inline = arena.alloc(
                            NodeValue::HtmlInline(format!("<kbd>{}</kbd>", tag.as_str().unwrap()))
                                .into(),
                        );
                        item.append(inline);
                    }
                    item.append(arena.alloc(NodeValue::Text(" -- ".to_string()).into()));

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
                let blank = arena.alloc(NodeValue::LineBreak.into());
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
