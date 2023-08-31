// @generated automatically by Diesel CLI.

diesel::table! {
    players (id) {
        id -> Nullable<Integer>,
        username -> Text,
        hashed_password -> Text,
    }
}
