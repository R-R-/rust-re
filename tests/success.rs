extern mod re;

use re::*;
use std::bool;

fn test_success(pattern: &str, string: &str) {
    match re::compile(pattern) {
        Ok(p) => {
            let mut pm = p;
            if bool::not(pm.matches(string)) {
                printfln!("\n[FAILED] Pattern '%s' against '%s'.", pattern, string);
            } else {
                print(".");
            }
        },
        Err(e) => printfln!("\nCompiling '%s' failed: %s.", pattern, e),
    }
}

fn main() {
    println("\nVerbatim matches");
    let s = ~"chair";
    test_success(s, "chair");
    test_success(s, " chair");
    test_success(s, "my chair are red");
    println("\nQuestion mark");
    let s = ~"chairs?";
    test_success(s, "chair");
    test_success(s, " chair");
    test_success(s, "my chair");
    test_success(s, "my chairs are red");
    println("\nKleene's star");
    let s = ~"baaa*!";
    test_success(s, "baa!");
    test_success(s, "baaa!");
    test_success(s, "baaaa!");
    test_success(s, " baaaa!");
    test_success(s, "I said, \" baaaaaa!\"");
    println("\nPlus");
    let s = ~"baa+!";
    test_success(s, "baa!");
    test_success(s, "baaa!");
    test_success(s, "baaaa!");
    test_success(s, " baaaa!");
    test_success(s, "I said, \" baaaaaa!\"");
    println("\nMiscelaneous");
    let s = ~"a?b+c*|d+|e+";
    test_success(s, "b");
    let s = ~"a+b+|a+b+";
    test_success(s, "ab");
    let s = ~"c(a+(bd)+)+";
    test_success(s, "cabd");
    println("\n");
}
