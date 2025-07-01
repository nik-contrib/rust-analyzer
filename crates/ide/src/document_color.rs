#![allow(warnings)]

use hir::{EditionedFileId, Semantics, db::HirDatabase};
use ide_db::RootDatabase;
use span::{FileId, TextRange};
use syntax::AstNode as _;
use tracing::error;

pub struct DocumentColor {
    pub range: TextRange,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

/// Document colors let IDEs annotate regions of source code as having a particular color.
///
/// The advantage over `textDocument/documentHighlight` is that IDEs can create virtual text
/// to show swatches, representing the color.
///
/// # Basic Plan
///
/// 1. Visit every expression that is known at compile time (is `const`)
/// 1. If the type `struct` of the expression contains
///    an attribute `#[rust_analyzer::color::rgb]`, proceed
/// 1. If there are 3 `f32` fields with attribute `#[rust_analyzer::color::rgb::{r,g,b}]`,
///    then we have all the necessary information to construct the type itself
pub(crate) fn document_color(db: &RootDatabase, file_id: FileId) -> Vec<DocumentColor> {
    error!("hello world");
    let sema = Semantics::new(db);

    let file = sema.parse_guess_edition(file_id);

    // let file_id = sema.attach_first_edition(file_id);

    let file = file.syntax();

    let scope = sema.scope(file);

    if std::fs::read_to_string("ENABLE_DOCUMENT_COLOR").is_err() {
        return Vec::new();
    }
    let interner = hir::next_solver::DbInterner::new_no_crate(db);

    let mut colors = Vec::new();

    // let def: DefWithBody = match item {
    //     ast::Item::Fn(it) => sema.to_def(&it)?.into(),
    //     ast::Item::Const(it) => sema.to_def(&it)?.into(),
    //     ast::Item::Static(it) => sema.to_def(&it)?.into(),
    //     _ => return None,
    // };
    let mut id = None;

    eprintln!("iterating thourh descendants");

    'outer: for event in file.descendants() {
        let range = event.text_range();

        if let Some(expr) = syntax::ast::Expr::cast(event.clone()) {
            let descended = sema.descend_node_into_attributes(expr.clone()).pop();
            let desc_expr = descended.as_ref().unwrap_or(&expr);

            // let id = sema.expr_id

            // InferenceContext::new();

            eprintln!("it is an EXPR");

            let Some(expr_id) = sema.expr_id(&expr) else { continue };

            eprintln!("got expr id");

            let Some(ty) = sema.type_of_expr(&expr) else { continue };
            let ty = ty.original;

            eprintln!("got ty of expr");

            let Some(adt) = ty.as_adt() else { continue };

            eprintln!("got ty");

            let Some(struc) = adt.as_struct() else { continue };

            eprintln!("got struct");

            eprintln!("color: detecting");

            if !struc.is_color(db) {
                continue;
            }

            eprintln!("color: detected");

            // this reads the fields
            if let Some(id) = id {
                eprintln!("has id");
                hir::InferenceResult::eval_expr_id(db, id, expr_id);
            } else {
                eprintln!("no id");
            }
        } else if let Some(f) = syntax::ast::Fn::cast(event) {
            if let Some(f) = sema.to_def(&f) {
                let a: hir::DefWithBody = f.into();
                let def: hir::DefWithBodyId = a.try_into().unwrap();
                id = Some(def);
            } else {
                continue;
            }
        } else {
            continue;
        };

        // let mut r = None;
        // let mut g = None;
        // let mut b = None;

        // db.const_eval(def, hir::next_solver::GenericArgs::empty(interner), None);

        // for field in struc.fields(db) {
        //     match field.name(db).as_str() {
        //         "red" => r = Some(255.0),
        //         "green" => g = Some(0.0),
        //         "blue" => b = Some(0.0),
        //         _ => continue 'outer,
        //     }
        // }

        // let (Some(r), Some(g), Some(b)) = (r, g, b) else {
        //     continue;
        // };

        // colors.push(DocumentColor { range, r, g, b, a: 1.0 });

        // for field in struc.fields(db) {
        //     // field.
        // }
    }

    colors
}

// ^^^^^ INFO: `sema.scope(file)` is NONE
// let Some(scope) = sema.scope(file) else {
//     return None;
// };
