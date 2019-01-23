#![feature(test)]

extern crate test;

use strings_exercise::{to_snakecase, to_snakecase_ascii};

macro_rules! snakecase_bench {
    ($name:ident, $s:expr) => {
        #[bench]
        fn $name(b: &mut ::test::Bencher) {
            let s = $s;
            b.bytes = (s.bytes().count()) as u64;
            b.iter(|| to_snakecase(s));
        }
    };
}

snakecase_bench!(snakecase_simple1, "sample text");
snakecase_bench!(snakecase_simple1, "sample_text");
snakecase_bench!(snakecase_long1, "inviteYourCustomersAddInvites");
snakecase_bench!(snakecase_long2, "invite_your_customers_add_invites");
snakecase_bench!(
    snakecase_long_special_chars,
    "FOO:BAR$BAZ__Sample    Text___"
);
snakecase_bench!(snakecase_unicode1, "ẞ•¶§ƒ˚foo˙∆˚¬");
snakecase_bench!(snakecase_unicode2, "ß_ƒ_foo");

macro_rules! snakecase_ascii_bench {
    ($name:ident, $s:expr) => {
        #[bench]
        fn $name(b: &mut ::test::Bencher) {
            let s = $s;
            b.bytes = (s.bytes().count()) as u64;
            b.iter(|| to_snakecase_ascii(s));
        }
    };
}

snakecase_ascii_bench!(ascii_snakecase_simple1, "sample text");
snakecase_ascii_bench!(ascii_snakecase_simple2, "sample_text");
snakecase_ascii_bench!(ascii_snakecase_long1, "inviteYourCustomersAddInvites");
snakecase_ascii_bench!(ascii_snakecase_long2, "invite_your_customers_add_invites");
snakecase_ascii_bench!(
    ascii_snakecase_long_special_chars,
    "FOO:BAR$BAZ__Sample    Text___"
);
snakecase_ascii_bench!(ascii_snakecase_unicode1, "ẞ•¶§ƒ˚foo˙∆˚¬");
snakecase_ascii_bench!(ascii_snakecase_unicode2, "ß_ƒ_foo");
