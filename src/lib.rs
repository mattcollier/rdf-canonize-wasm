use wasm_bindgen::prelude::*;
use js_sys::{Array, JsString, Reflect};
use rdf_canonize::nquads::{
    Dataset, Graph, Object, Predicate, Quad, Subject, Term, TermType,
};

// extern crate web_sys;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn canonize(quads: &Array, _opts: js_sys::Object) -> String {
    // iterate the parameters
    let mut dataset = Dataset::new();
    for q in quads.iter() {
        let mut subject = Subject::new();
        subject.set_term_type(&get_term_type(&q, "subject").unwrap());
        let x = get_term_value(&q, "subject").to_owned();
        subject.set_value(&x);

        let mut predicate = Predicate::new();
        predicate.set_term_type(&get_term_type(&q, "predicate").unwrap());
        let s = get_term_value(&q, "predicate");
        predicate.set_value(&s);

        let mut object;
        let value = get_term_value(&q, "object");
        let term_type = get_term_type(&q, "object").unwrap();
        match term_type {
            TermType::Literal => {
                object = Object::new();
                object.set_term_type(&term_type);
                object.set_value(&value);

                if let Some(datatype) = get_datatype(&q, "object") {
                    object.set_datatype(&datatype);
                }

                // get language which sometimes exists
                if let Some(language) = get_language(&q, "object") {
                    object.set_language(&language);
                }
            }
            // BlankNode or NamedNode
            _ => {
                object = Object::new();
                object.set_term_type(&term_type);
                object.set_value(&value);
            }
        }


        let mut graph = Graph::new();
        graph.set_term_type(&get_term_type(&q, "graph").unwrap());
        let s = get_term_value(&q, "graph");
        graph.set_value(&s);

        dataset.add(Quad {
            subject,
            predicate,
            object,
            graph
        });
    }

    rdf_canonize::canonize(&dataset, "URDNA2015").unwrap()
}

fn get_term_type(o: &JsValue, key: &str) -> Option<TermType> {
    let term_type: JsValue = "termType".to_string().into();
    let s = Reflect::get(&o, &key.to_string().into()).unwrap();
    let t = Reflect::get(&s, &term_type).unwrap();
    let q: JsString = t.into();
    let z: String = q.into();
    match z.as_str() {
        "BlankNode" => Some(TermType::BlankNode),
        "NamedNode" => Some(TermType::NamedNode),
        "Literal" => Some(TermType::Literal),
        "DefaultGraph" => Some(TermType::DefaultGraph),
        _ => None,
    }
}

fn get_term_value(o: &JsValue, key: &str) -> String {
    let term_type: JsValue = "value".to_string().into();
    let s = Reflect::get(&o, &key.to_string().into()).unwrap();
    let t = Reflect::get(&s, &term_type).unwrap();
    let q: JsString = t.into();
    q.into()
}

fn get_datatype(o: &JsValue, key: &str) -> Option<String> {
    let data_type: JsValue = "datatype".to_string().into();
    let term_type: JsValue = "value".to_string().into();
    let s = Reflect::get(&o, &key.to_string().into()).unwrap();
    let t = Reflect::get(&s, &data_type).unwrap();
    if t.is_undefined() {
        return None
    }
    let p: JsString = Reflect::get(&t, &term_type).unwrap().into();
    Some(p.into())
}

fn get_language(o: &JsValue, key: &str) -> Option<String> {
    let language: JsValue = "language".to_string().into();
    let s = Reflect::get(&o, &key.to_string().into()).unwrap();
    let t = Reflect::get(&s, &language).unwrap();
    if t.is_undefined() {
        return None
    }
    let p: JsString = t.into();
    Some(p.into())
}
