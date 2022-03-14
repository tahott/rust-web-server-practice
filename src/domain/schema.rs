table! {
    users (id) {
        id -> Int4,
        login -> Varchar,
        name -> Varchar,
        avatar_url -> Varchar,
        email -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
