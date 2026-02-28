// @generated automatically by Diesel CLI.

diesel::table! {
    chromosome (rowid) {
        rowid -> Integer,
        seq_name -> Nullable<Text>,
        seq_length -> Nullable<Integer>,
        is_circular -> Nullable<Integer>,
    }
}

diesel::table! {
    entrezgene (rowid) {
        rowid -> Integer,
        gene_id -> Nullable<Text>,
        entrezid -> Nullable<Integer>,
    }
}

diesel::table! {
    exon (rowid) {
        rowid -> Integer,
        exon_id -> Nullable<Text>,
        exon_seq_start -> Nullable<Integer>,
        exon_seq_end -> Nullable<Integer>,
    }
}

diesel::table! {
    gene (rowid) {
        rowid -> Integer,
        gene_id -> Nullable<Text>,
        gene_name -> Nullable<Text>,
        gene_biotype -> Nullable<Text>,
        gene_seq_start -> Nullable<Integer>,
        gene_seq_end -> Nullable<Integer>,
        seq_name -> Nullable<Text>,
        seq_strand -> Nullable<Integer>,
        seq_coord_system -> Nullable<Text>,
        description -> Nullable<Text>,
        gene_id_version -> Nullable<Text>,
        canonical_transcript -> Nullable<Text>,
    }
}

diesel::table! {
    metadata (rowid) {
        rowid -> Integer,
        name -> Nullable<Text>,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    protein (rowid) {
        rowid -> Integer,
        tx_id -> Nullable<Text>,
        protein_id -> Nullable<Text>,
        protein_sequence -> Nullable<Text>,
    }
}

diesel::table! {
    protein_domain (rowid) {
        rowid -> Integer,
        protein_id -> Nullable<Text>,
        protein_domain_id -> Nullable<Text>,
        protein_domain_source -> Nullable<Text>,
        interpro_accession -> Nullable<Text>,
        prot_dom_start -> Nullable<Integer>,
        prot_dom_end -> Nullable<Integer>,
    }
}

diesel::table! {
    tx (rowid) {
        rowid -> Integer,
        tx_id -> Nullable<Text>,
        tx_biotype -> Nullable<Text>,
        tx_seq_start -> Nullable<Integer>,
        tx_seq_end -> Nullable<Integer>,
        tx_cds_seq_start -> Nullable<Integer>,
        tx_cds_seq_end -> Nullable<Integer>,
        gene_id -> Nullable<Text>,
        tx_support_level -> Nullable<Integer>,
        tx_id_version -> Nullable<Text>,
        gc_content -> Nullable<Float>,
    }
}

diesel::table! {
    tx2exon (rowid) {
        rowid -> Integer,
        tx_id -> Nullable<Text>,
        exon_id -> Nullable<Text>,
        exon_idx -> Nullable<Integer>,
    }
}

diesel::table! {
    uniprot (rowid) {
        rowid -> Integer,
        protein_id -> Nullable<Text>,
        uniprot_id -> Nullable<Text>,
        uniprot_db -> Nullable<Text>,
        uniprot_mapping_type -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    chromosome,
    entrezgene,
    exon,
    gene,
    metadata,
    protein,
    protein_domain,
    tx,
    tx2exon,
    uniprot,
);
