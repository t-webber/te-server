// @generated automatically by Diesel CLI.

#![allow(clippy::restriction)]

diesel::table! {
    users (id) {
        id -> Integer,
        firstname -> Nullable<Text>,
        lastname -> Nullable<Text>,
        email -> Text,
    }
}
