#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]

extern crate rustc_hir;
extern crate rustc_span;

use rustc_lint::{LateContext, LateLintPass};
use clippy_utils::diagnostics::{self, span_lint};
//use dylint_linting::declare_late_lint;
use if_chain::if_chain;
use rustc_hir::{
    intravisit::{walk_expr,  Visitor},
    Expr, ExprKind, TyKind,QPath,
    
};
//use rustc_hir::{LateContext, LateLintPass};
use rustc_span::Span;


dylint_linting::declare_late_lint!{
    pub AVOID_AUTOKEY_UPGRADABLE,
    Warn,
    "UN MENSAJEEEEEEEEEEE"
}

// espub struct AvoidAutokeyUpgradable {
//     is_lazy: Vec<String>,
// }

impl<'tcx> LateLintPass<'tcx> for AvoidAutokeyUpgradable {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>, //una sola expresion, block de expression
        sp: rustc_span::Span,
        _: rustc_span::def_id::LocalDefId,
    ) {
        let mut au_storage = AvoidAutokeyUpgradableVisitor {
            is_lazy: Vec::new(),
            has_sets: std::collections::HashMap::new(),
            has_gets: std::collections::HashMap::new(),
            span: std::collections::HashMap::new(),
        };

        walk_expr(&mut au_storage, body.value);

        for (key, value) in &au_storage.has_gets {
            if !au_storage.has_sets.contains_key(key) {
                span_lint(
                    cx,
                    AVOID_AUTOKEY_UPGRADABLE,
                    *au_storage.span.get(key).unwrap(),
                    "NO TENES SET PARA ESTE GET",
                );
            }
        }
    }
    
    fn check_field_def(&mut self, cx: &LateContext<'tcx>, field: &'tcx rustc_hir::FieldDef<'tcx>) {
        
        if let TyKind::Path(qpath) = field.ty.kind {
            if let QPath::Resolved(Some(ty), _) = qpath {
                if let TyKind::Path(qpath) = ty.kind {
                    if let QPath::Resolved(None, p) = qpath {
                        let did = p.res.def_id();
                        let v = cx.get_def_path(did);
                        
                        //AvoidAutokeyUpgradable.insert(field.ident.name.to_string(), field.span);
                        //ident es el nombre de la variable
                        //Me interesa
                        //[ink_storage, lazy, Lazy]
                        //if v[1].to_string() == "lazy" && v[2].to_string() == "Lazy"{
                        //    self.is_lazy.push(field.ident.name.to_string());}
                        if v[1].to_string() == "lazy" && v[2].to_string() == "Lazy"{
                            

                            self.is_lazy.push(field.ident.name.to_string());
                            
                        
                        }
                    }
                   
                }
            }
        } 
    }
   /*
    fn check_struct_def(&mut self, _: &LateContext<'tcx>, vd: &'tcx rustc_hir::VariantData<'tcx>) {
    
        if let VariantData::Struct {fields,..} = vd {
            for field in *fields {
                //println!("{}", field.ident);
                //println!("KIND DEL FIELD-------{:?}", field.ty.kind);
                if_chain!{
                    if let TyKind::Path(qpath) = field.ty.kind;
                    if let QPath::Resolved(option, _) = qpath;
                    if let Some(Lo_de_adentro) = option;
                    if let TyKind::Path(qpath2) = Lo_de_adentro.kind;
                    if let QPath::Resolved(_, hir_path2) = qpath2;
                    then {
                        //println!("LO DE ADENTRO DEFid {:?}", hir_path2.segments[0].res);
                        //println!("--------------");
                    }
                }
            }
        } */
        //println!("struct / {}", self.gets.len());  
        /* 
        if_chain!{
            if let VariantData::Struct {fields,..} = vd;
            if let TyKind::Path(qpath) = fields[0].ty.kind;
            if let QPath::Resolved(option, hir_path) = qpath;
            then {
                println!("OPTION{:?}", option);
                println!("--------------");
            }}
        }
        */

        
}


pub struct AvoidAutokeyUpgradableVisitor {
    is_lazy: Vec<String>,
    has_sets: std::collections::HashMap<String, bool>,
    has_gets: std::collections::HashMap<String, bool>,
    span: std::collections::HashMap<String, Span>

}
        
impl<'tcx> Visitor<'tcx> for AvoidAutokeyUpgradableVisitor {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        if_chain! {
            if let ExprKind::MethodCall(path, self_arg,_, _) = &expr.kind;
            if path.ident.name.as_str() == "get";
            if let ExprKind::Field(_, ident) = self_arg.kind;
            then {
                self.has_gets.insert(ident.name.to_string(), true); //aca guardo (large_vec, true)
                self.span.insert(ident.name.to_string(), expr.span);
                
                }
        }

        if_chain! {
            
            if let ExprKind::MethodCall(path, self_arg,_, _) = &expr.kind;
            if path.ident.name.as_str() == "set";
            if let ExprKind::Field(_, ident) = self_arg.kind;
            if self.has_gets.contains_key(&ident.name.to_string()); //aca se fija si esta large_vec
            then {
                
                self.has_sets.insert(ident.name.to_string(), true);  
                
                }
        }
        
        walk_expr(self, expr)
        
    }
}
