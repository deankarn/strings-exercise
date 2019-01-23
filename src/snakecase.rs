use std::borrow::Cow;

pub fn to_snakecase() {}
pub fn to_snakecase_ascii() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    macro_rules! snakecase_test {
        ($name:ident, $input:expr, $output:expr) => {
            #[test]
            fn $name() {
                let results = to_snakecase($input);
                assert_eq!(results, $output);
            }
        };
    }

    snakecase_test!(empty, "", "");
    snakecase_test!(equal, "sample_text", "sample_text");
    snakecase_test!(space, "sample text", "sample_text");
    snakecase_test!(dash, "sample-text", "sample_text");
    snakecase_test!(multi_underscore, "sample___text", "sample_text");
    snakecase_test!(ending_underscore, "sample_text_", "sample_text");
    snakecase_test!(ending_multi_underscore, "sample_text__", "sample_text");
    snakecase_test!(uppercase_sep, "sampleText", "sample_text");
    snakecase_test!(
        multi_uppercase,
        "inviteYourCustomersAddInvites",
        "invite_your_customers_add_invites"
    );
    snakecase_test!(space_with_uppercase, "sample 2 Text", "sample_2_text");
    snakecase_test!(special_chars, "FOO:BAR$BAZ", "foo_bar_baz");
    snakecase_test!(caps, "samPLE text", "sam_ple_text");
    snakecase_test!(multi_spaces, "   sample   2    Text   ", "sample_2_text");
    snakecase_test!(
        special_with_spaces,
        "   $#$sample   2    Text   ",
        "sample_2_text"
    );
    snakecase_test!(caps_with_space_sep, "SAMPLE 2 TEXT", "sample_2_text");
    snakecase_test!(
        leading_underscore_special,
        "___$$Base64Encode",
        "base64_encode"
    );
    snakecase_test!(caps_hash_sep, "FOO#BAR#BAZ", "foo_bar_baz");
    snakecase_test!(domain, "something.com", "something_com");
    snakecase_test!(special_leading_and_trailing, "$something%", "something");
    snakecase_test!(camel_case, "CStringRef", "cstring_ref");
    snakecase_test!(unicode_mixed, "ẞ•¶§ƒ˚foo˙∆˚¬", "ß_ƒ_foo");
    snakecase_test!(unicode_uppercase, "ẞ", "ß"); // capitol unicode german to lowercase
    snakecase_test!(
        special_chars_long,
        "FOO:BAR$BAZ__Sample    Text___",
        "foo_bar_baz_sample_text"
    );

    // ascii
    macro_rules! snakecase_ascii_test {
        ($name:ident, $input:expr, $output:expr) => {
            #[test]
            fn $name() {
                let results = to_snakecase_ascii($input);
                assert_eq!(results, $output);
            }
        };
    }

    snakecase_ascii_test!(ascii_empty, "", "");
    snakecase_ascii_test!(ascii_equal, "sample_text", "sample_text");
    snakecase_ascii_test!(ascii_space, "sample text", "sample_text");
    snakecase_ascii_test!(ascii_dash, "sample-text", "sample_text");
    snakecase_ascii_test!(ascii_multi_underscore, "sample___text", "sample_text");
    snakecase_ascii_test!(ascii_ending_underscore, "sample_text_", "sample_text");
    snakecase_ascii_test!(
        ascii_ending_multi_underscore,
        "sample_text__",
        "sample_text"
    );
    snakecase_ascii_test!(ascii_uppercase_sep, "sampleText", "sample_text");
    snakecase_ascii_test!(
        ascii_multi_uppercase,
        "inviteYourCustomersAddInvites",
        "invite_your_customers_add_invites"
    );
    snakecase_ascii_test!(ascii_space_with_uppercase, "sample 2 Text", "sample_2_text");
    snakecase_ascii_test!(ascii_special_chars, "FOO:BAR$BAZ", "foo_bar_baz");
    snakecase_ascii_test!(ascii_caps, "samPLE text", "sam_ple_text");
    snakecase_ascii_test!(
        ascii_multi_spaces,
        "   sample   2    Text   ",
        "sample_2_text"
    );
    snakecase_ascii_test!(
        ascii_special_with_spaces,
        "   $#$sample   2    Text   ",
        "sample_2_text"
    );
    snakecase_ascii_test!(ascii_caps_with_space_sep, "SAMPLE 2 TEXT", "sample_2_text");
    snakecase_ascii_test!(
        ascii_leading_underscore_special,
        "___$$Base64Encode",
        "base64_encode"
    );
    snakecase_ascii_test!(ascii_caps_hash_sep, "FOO#BAR#BAZ", "foo_bar_baz");
    snakecase_ascii_test!(ascii_domain, "something.com", "something_com");
    snakecase_ascii_test!(
        ascii_special_leading_and_trailing,
        "$something%",
        "something"
    );
    snakecase_ascii_test!(ascii_camel_case, "CStringRef", "cstring_ref");
    snakecase_ascii_test!(ascii_unicode_mixed, "ẞ•¶§ƒ˚foo˙∆˚¬", "foo");
    snakecase_ascii_test!(ascii_unicode_uppercase, "ẞ", ""); // capitol unicode german to lowercase
    snakecase_ascii_test!(
        ascii_special_chars_long,
        "FOO:BAR$BAZ__Sample    Text___",
        "foo_bar_baz_sample_text"
    );
    snakecase_ascii_test!(ascii_digit_underscore, "5test", "5test");
    snakecase_ascii_test!(ascii_character_digit, "test5", "test5");
    snakecase_ascii_test!(ascii_uppercase_digit, "THE5r", "the5r");
    snakecase_ascii_test!(ascii_digit_uppercase, "5TEst", "5test");
}
