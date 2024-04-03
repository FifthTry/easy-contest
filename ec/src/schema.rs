// @generated automatically by Diesel CLI.

diesel::table! {
    ec_contest (id) {
        id -> Int8,
        title -> Text,
        description -> Text,
        start_at -> Timestamptz,
        end_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    ec_contest_submission (id) {
        id -> Int8,
        user_id -> Int8,
        title -> Text,
        deploy_url -> Text,
        source_url -> Text,
        message -> Text,
        is_winner -> Bool,
        contest_id -> Nullable<Int8>,
    }
}

diesel::joinable!(ec_contest_submission -> ec_contest (contest_id));

diesel::allow_tables_to_appear_in_same_query!(ec_contest, ec_contest_submission,);
