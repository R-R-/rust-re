extern mod re;

use re::*;

fn test_success(pattern: &str, string: &str) {
    match re::compile(pattern) {
        Ok(p) => {
            let mut pm = p;
            if !pm.matches(string) {
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
    println("\nDot");
    let s = ~"ca.";
    test_success(s, "cat");
    test_success(s, "car");
    test_success(s, "cap");
    test_success(s, "cam");
    test_success(s, "can");
    test_success(s, "cab");
    println("\nEscaped characters");
    let s = ~"\\?\\*\\+\\.\\|\\(\\)";
    test_success(s, "?*+.|()");
    println("\nMiscelaneous");
    let s = ~"a?b+c*|d+|e+";
    test_success(s, "b");
    test_success(s, "bb");
    test_success(s, "ab");
    test_success(s, "bc");
    test_success(s, "abc");
    test_success(s, "d");
    test_success(s, "e");
    let s = ~"a+b+|c+";
    test_success(s, "ab");
    test_success(s, "aabb");
    test_success(s, "c");
    test_success(s, "cc");
    let s = ~"c(a+(bd)+)+";
    test_success(s, "cabd");
    println("\n");
}
