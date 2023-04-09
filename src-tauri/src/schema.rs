// @generated automatically by Diesel CLI.

diesel::table! {
    unicode_data (code_value) {
        code_value -> Text,
        character_name -> Text,
        general_category -> Nullable<Text>,
        canonical_combining_classes -> Nullable<Text>,
        bidirectional_category -> Nullable<Text>,
        character_decomposition_mapping -> Nullable<Text>,
        decimal_digit_value -> Nullable<Text>,
        digit_value -> Nullable<Text>,
        numeric_value -> Nullable<Text>,
        mirrored -> Nullable<Text>,
        unicode_1_0_name -> Nullable<Text>,
        iso_10646_comment_field -> Nullable<Text>,
        uppercase_mapping -> Nullable<Text>,
        lowercase_mapping -> Nullable<Text>,
        titlecase_mapping -> Nullable<Text>,
    }
}
