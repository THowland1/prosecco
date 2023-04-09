export type UnicodeData = {
	code_value: string;
	character_name: string;
	general_category: string | null;
	canonical_combining_classes: string | null;
	bidirectional_category: string | null;
	character_decomposition_mapping: string | null;
	decimal_digit_value: string | null;
	digit_value: string | null;
	numeric_value: string | null;
	mirrored: string | null;
	unicode_1_0_name: string | null;
	iso_10646_comment_field: string | null;
	uppercase_mapping: string | null;
	lowercase_mapping: string | null;
	titlecase_mapping: string | null;
};
