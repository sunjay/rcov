#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private, quote)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

mod ext;

// Register the plugin

use rustc_plugin::Registry;

use syntax::ext::base::SyntaxExtension::MultiModifier;
use syntax::parse::token::intern;

use ext::expand;

#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
    reg.register_syntax_extension(intern("coverage"), MultiModifier(Box::new(expand)));
}

