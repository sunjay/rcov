use syntax::ext::base::ExtCtxt;
use syntax::codemap::Span;
use syntax::ast::MetaItem;
use syntax::ext::base::Annotatable;

pub fn expand(ecx: &mut ExtCtxt, span: Span, meta_item: &MetaItem, item: Annotatable) -> Annotatable {
    println!("{:#?}", item);
    return item;
}
