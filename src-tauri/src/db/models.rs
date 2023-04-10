use serde::Serialize; // makes object of this class json serializable, we will convert these objects to json and send to user.

#[derive(Queryable, Serialize, Debug)] // these annotation adds extra functionality to objects of this struct, Debug is for printing in console `dbg!(todo)`
pub struct UnicodeData {
    pub code_value: String,
    pub character_name: String,
    pub general_category: Option<String>,
    pub canonical_combining_classes: Option<String>,
    pub bidirectional_category: Option<String>,
    pub character_decomposition_mapping: Option<String>,
    pub decimal_digit_value: Option<String>,
    pub digit_value: Option<String>,
    pub numeric_value: Option<String>,
    pub mirrored: Option<String>,
    pub unicode_1_0_name: Option<String>,
    pub iso_10646_comment_field: Option<String>,
    pub uppercase_mapping: Option<String>,
    pub lowercase_mapping: Option<String>,
    pub titlecase_mapping: Option<String>,
}
