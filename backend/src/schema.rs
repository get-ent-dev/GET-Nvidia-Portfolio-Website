// @generated automatically by Diesel CLI.

diesel::table! {
    about_pages (id) {
        #[max_length = 36]
        id -> Varchar,
        content -> Text,
        #[max_length = 255]
        image_url -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    contact_messages (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        subject -> Nullable<Varchar>,
        message -> Text,
        sent_at -> Nullable<Timestamp>,
        is_read -> Nullable<Bool>,
    }
}

diesel::table! {
    projects (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        #[max_length = 255]
        image_url -> Nullable<Varchar>,
        #[max_length = 255]
        project_url -> Nullable<Varchar>,
        #[max_length = 255]
        tags -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    about_pages,
    contact_messages,
    projects,
    users,
);
