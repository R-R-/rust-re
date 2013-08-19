extern mod re;

use re::*;
use std::bool;

fn test_success(pattern: &str, string: &str) {
    match re::compile(pattern) {
        Ok(p) => {
            let mut pm = p;
            if bool::not(pm.matches(string)) {
                printfln!("\nMatch failed: pattern '%s' against '%s'.", pattern, string);
            } else {
                print(".");
            }
        },
        Err(e) => printfln!("\nCompiling '%s' failed: %s.", pattern, e),
    }
}

fn main() {
    let s = ~"baa*!";
    test_success(s, "ba!");
    let s = ~"a?b+c*|d+|e+";
    test_success(s, "b");
    let s = ~"a+b+|a+b+";
    test_success(s, "ab");
    let s = ~"c(a+(bd)+)+";
    test_success(s, "cabd");
    println(".");
}
