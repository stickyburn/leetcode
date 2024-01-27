pub fn is_palindrome(input: i32) -> bool {
    if input < 0 {
        return false;
    }

    let mut original = input;
    let mut reversed = 0;

    while original != 0 {
        let digit = original % 10;
        reversed = reversed * 10 + digit;
        original /= 10;
    }

    return input == reversed;
}
