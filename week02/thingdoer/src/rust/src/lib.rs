use crate::schema::{exon, gene, tx, tx2exon};
use diesel::prelude::*;
use extendr_api::prelude::*;

mod schema;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

/// solves the exercise given the path to a valid sql database containing the annotated genomic data.
/// @export
#[extendr]
fn do_the_thing(path: &str) -> Result<List> {
    let mut conn = SqliteConnection::establish(path).unwrap();
    let gene_id_count: i64 = gene::table
        .filter(gene::gene_biotype.eq("protein_coding"))
        .select(gene::gene_id)
        .distinct()
        .count()
        .get_result(&mut conn)
        .map_err(|_| extendr_api::Error::EvalError("could not read db".into()))?;

    let gene_symbol_count: i64 = gene::table
        .filter(gene::gene_biotype.eq("protein_coding"))
        .select(gene::gene_name)
        .distinct()
        .count()
        .get_result(&mut conn)
        .map_err(|_| extendr_api::Error::EvalError("could not read db".into()))?;

    rprintln!("counted {gene_id_count} gene ids and {gene_symbol_count} gene_symbols");

    let lengths: Vec<i64> = tx::table
        .inner_join(tx2exon::table.on(tx2exon::tx_id.eq(tx::tx_id)))
        .inner_join(exon::table.on(tx2exon::exon_id.eq(exon::exon_id)))
        .filter(tx::tx_biotype.eq("protein_coding"))
        .group_by(tx::tx_id)
        .select(diesel::dsl::sum(
            exon::exon_seq_end - exon::exon_seq_start + 1, // include end
        ))
        .load::<Option<i64>>(&mut conn)
        .map_err(|_| extendr_api::Error::EvalError("could not read db".into()))?
        .into_iter()
        .flatten()
        .collect();

    Ok(list!(
        ensembl_id_count = gene_id_count,
        gene_symbol_count = gene_symbol_count,
        spliced_lengths = lengths
    ))
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod thingdoer;
    fn hello_world;
    fn do_the_thing;
}
