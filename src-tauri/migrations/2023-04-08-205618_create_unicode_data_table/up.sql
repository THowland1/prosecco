CREATE TABLE unicode_data (
    code_value CHAR(4) NOT NULL PRIMARY KEY,
    character_name VARCHAR NOT NULL,
    general_category VARCHAR NULL,
    canonical_combining_classes VARCHAR NULL,
    bidirectional_category VARCHAR NULL,
    character_decomposition_mapping VARCHAR NULL,
    decimal_digit_value VARCHAR NULL,
    digit_value VARCHAR NULL,
    numeric_value VARCHAR NULL,
    mirrored VARCHAR NULL,
    unicode_1_0_name VARCHAR NULL,
    iso_10646_comment_field VARCHAR NULL,
    uppercase_mapping VARCHAR NULL,
    lowercase_mapping VARCHAR NULL,
    titlecase_mapping VARCHAR NULL
);