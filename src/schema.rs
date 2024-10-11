// @generated automatically by Diesel CLI.

diesel::table! {
    dumps (receipt_id) {
        receipt_id -> Varchar,
        method_name -> Nullable<Varchar>,
        block_height -> Nullable<Int8>,
        block_timestamp -> Nullable<Numeric>,
        args -> Nullable<Varchar>,
        author -> Nullable<Varchar>,
        proposal_id -> Nullable<Int8>,
    }
}

diesel::table! {
    proposal_snapshots (proposal_id, ts) {
        proposal_id -> Int4,
        block_height -> Nullable<Int8>,
        ts -> Numeric,
        editor_id -> Nullable<Varchar>,
        social_db_post_block_height -> Nullable<Int8>,
        labels -> Nullable<Jsonb>,
        proposal_version -> Nullable<Varchar>,
        proposal_body_version -> Nullable<Varchar>,
        name -> Nullable<Text>,
        category -> Nullable<Varchar>,
        summary -> Nullable<Text>,
        description -> Nullable<Text>,
        linked_proposals -> Nullable<Jsonb>,
        linked_rfp -> Nullable<Int4>,
        requested_sponsorship_usd_amount -> Nullable<Numeric>,
        requested_sponsorship_paid_in_currency -> Nullable<Varchar>,
        requested_sponsor -> Nullable<Varchar>,
        receiver_account -> Nullable<Varchar>,
        supervisor -> Nullable<Varchar>,
        timeline -> Nullable<Jsonb>,
        views -> Nullable<Int4>,
    }
}

diesel::table! {
    proposals (id) {
        id -> Int4,
        author_id -> Varchar,
    }
}

diesel::table! {
    rfp_dumps (receipt_id) {
        receipt_id -> Varchar,
        method_name -> Nullable<Varchar>,
        block_height -> Nullable<Int8>,
        block_timestamp -> Nullable<Numeric>,
        args -> Nullable<Varchar>,
        author -> Nullable<Varchar>,
        rfp_id -> Nullable<Int8>,
    }
}

diesel::table! {
    rfp_snapshots (rfp_id, ts) {
        rfp_id -> Int4,
        block_height -> Nullable<Int8>,
        ts -> Numeric,
        editor_id -> Nullable<Varchar>,
        social_db_post_block_height -> Nullable<Int8>,
        labels -> Nullable<Jsonb>,
        linked_proposals -> Nullable<Jsonb>,
        rfp_version -> Nullable<Varchar>,
        rfp_body_version -> Nullable<Varchar>,
        name -> Nullable<Text>,
        category -> Nullable<Varchar>,
        summary -> Nullable<Text>,
        description -> Nullable<Text>,
        timeline -> Nullable<Jsonb>,
        submission_deadline -> Nullable<Numeric>,
        views -> Nullable<Int4>,
    }
}

diesel::table! {
    rfps (id) {
        id -> Int4,
        author_id -> Varchar,
    }
}

diesel::joinable!(rfp_snapshots -> rfps (rfp_id));

diesel::allow_tables_to_appear_in_same_query!(
    dumps,
    proposal_snapshots,
    proposals,
    rfp_dumps,
    rfp_snapshots,
    rfps,
);
