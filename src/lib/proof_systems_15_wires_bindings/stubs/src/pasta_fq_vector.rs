use crate::{arkworks::CamlFq, caml_pointer::CamlPointer};
use mina_curves::pasta::fq::Fq;
use ocaml_gen::ocaml_gen;

pub type CamlPastaFqVector = CamlPointer<Vec<Fq>>;

#[ocaml_gen]
#[ocaml::func]
pub fn caml_pasta_fq_vector_create() -> CamlPastaFqVector {
    CamlPointer::new(Vec::new())
}

#[ocaml_gen]
#[ocaml::func]
pub fn caml_pasta_fq_vector_length(v: CamlPastaFqVector) -> ocaml::Int {
    v.len() as isize
}

#[ocaml_gen]
#[ocaml::func]
pub fn caml_pasta_fq_vector_emplace_back(mut v: CamlPastaFqVector, x: CamlFq) {
    v.push(x.into());
}

#[ocaml_gen]
#[ocaml::func]
pub fn caml_pasta_fq_vector_get(
    v: CamlPastaFqVector,
    i: ocaml::Int,
) -> Result<CamlFq, ocaml::Error> {
    match v.get(i as usize) {
        Some(x) => Ok(x.into()),
        None => Err(ocaml::Error::invalid_argument("caml_pasta_fq_vector_get")
            .err()
            .unwrap()),
    }
}
