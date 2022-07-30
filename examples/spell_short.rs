use jabba_lib::jspell;

fn main() {
    assert_eq!(jspell::spell_number(0), "zero");
    assert_eq!(jspell::spell_number(5), "five");
    assert_eq!(jspell::spell_number(11), "eleven");
    assert_eq!(jspell::spell_number(21), "twenty-one");
    assert_eq!(jspell::spell_number(101), "one hundred and one");
    assert_eq!(jspell::spell_number(999), "nine hundred and ninety-nine");
}
