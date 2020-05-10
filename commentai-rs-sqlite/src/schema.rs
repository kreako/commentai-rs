table! {
    comments (id) {
        id -> Nullable<Integer>,
        title -> Nullable<Text>,
        content -> Text,
        author_name -> Nullable<Text>,
        author_email -> Nullable<Text>,
        author_ip -> Text,
        dt -> Text,
        url -> Text,
    }
}
