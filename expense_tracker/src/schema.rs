// @generated automatically by Diesel CLI.

diesel::table! {
    expenses (id) {
        id -> Text,
        expense_type -> Text,
        amount -> Float,
        date -> Date,
        description -> Nullable<Text>,
    }
}
