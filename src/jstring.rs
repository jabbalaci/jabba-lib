//! string

/// Returns the reversed version of the input string.
///
/// # Examples
///
/// ```
/// let text = "abcd";
/// let answer = jabba_lib::jstring::str_rev(text);
///
/// assert_eq!(answer, "dcba");
/// ```
pub fn str_rev(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

/// Returns `true` if the given string is palindrome.
///
/// # Examples
///
/// ```
/// let yes = "abcba";
/// let no = "ABC";
///
/// assert_eq!(jabba_lib::jstring::is_palindrome(yes), true);
/// assert_eq!(jabba_lib::jstring::is_palindrome(no), false);
/// ```
pub fn is_palindrome(s: &str) -> bool {
    s == str_rev(s)
}

/// Removes the trailing newline of the given string.
///
/// It modifies the string in place.
///
/// It was inspired by Perl's `chomp()`.
///
/// # Examples
///
/// ```
/// let mut hello = String::from("hello\n");
/// let mut world = String::from("world\r\n");
///
/// jabba_lib::jstring::chomp(&mut hello);
/// jabba_lib::jstring::chomp(&mut world);
///
/// assert_eq!(hello, "hello");
/// assert_eq!(world, "world");
/// ```
pub fn chomp(text: &mut String) {
    if text.ends_with('\n') {
        text.pop();
        if text.ends_with('\r') {
            text.pop();
        }
    }
}

/// Returns a centered string of length `width`.
///
/// Padding is done with spaces.
///
/// It's similar to Python's `str.center()`.
///
/// # Examples
///
/// ```
/// let text = "*";
/// let result = jabba_lib::jstring::center(text, 3);
///
/// assert_eq!(result, " * ");
/// ```
pub fn center(s: &str, width: usize) -> String {
    format!("{s:^w$}", w = width)
}

/// Returns a capitalized version of the string.
///
/// More specifically, it makes the first character upper case and
/// the rest lower case.
///
/// It's like Python's `str.capitalize()`.
///
/// # Examples
///
/// ```
/// let name = "kAtE";
/// let result = jabba_lib::jstring::capitalize(name);
///
/// assert_eq!(result, "Kate");
/// ```
pub fn capitalize(s: &str) -> String {
    if s.is_empty() {
        String::new()
    } else {
        let chars: Vec<char> = s.chars().collect();
        let first = chars[0].to_uppercase();
        let rest = chars.into_iter().skip(1).collect::<String>().to_lowercase();
        format!("{}{}", first, rest)
    }
}

// ==========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_rev_test1() {
        assert_eq!(str_rev(""), "");
        assert_eq!(str_rev("a"), "a");
        assert_eq!(str_rev("ab"), "ba");
        assert_eq!(str_rev("anna"), "anna");
        assert_eq!(str_rev("abc"), "cba");
        assert_eq!(str_rev("AbCdE"), "EdCbA");
    }

    #[test]
    fn is_palindrome_test1() {
        assert!(is_palindrome(""));
        assert!(is_palindrome("a"));
        assert!(is_palindrome("aa"));
        assert!(is_palindrome("anna"));
        assert!(is_palindrome("görög"));
    }

    #[test]
    fn is_palindrome_test2() {
        assert_eq!(is_palindrome("ab"), false);
        assert_eq!(is_palindrome("Anna"), false);
    }

    #[test]
    fn chomp_test1() {
        let mut text = "".to_string();
        chomp(&mut text);
        assert_eq!(text, "");
        //
        let mut text = "abc".to_string();
        chomp(&mut text);
        assert_eq!(text, "abc");
        //
        let mut text = "\n".to_string();
        chomp(&mut text);
        assert_eq!(text, "");
        //
        let mut text = "\r\n".to_string();
        chomp(&mut text);
        assert_eq!(text, "");
        //
        let mut text = "\nend".to_string();
        chomp(&mut text);
        assert_eq!(text, "\nend");
        //
        let mut text = "\r\nend".to_string();
        chomp(&mut text);
        assert_eq!(text, "\r\nend");
        //
        let mut text = "longer\nstring\n".to_string();
        chomp(&mut text);
        assert_eq!(text, "longer\nstring");
    }

    #[test]
    fn center_test1() {
        assert_eq!(center("-", 0), "-");
        assert_eq!(center("-", 1), "-");
        assert_eq!(center("-", 2), "- ");
        assert_eq!(center("-", 3), " - ");
    }

    #[test]
    fn capitalize_test1() {
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("a"), "A");
        assert_eq!(capitalize("aa"), "Aa");
        assert_eq!(capitalize("aNnA"), "Anna");
    }
}
