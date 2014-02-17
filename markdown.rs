// This code is heavily inspired by hoedown.

//
// Performance observations:
//
// Note that buf.head() == Some(&ch) is much faster than buf[0] == ch.
//

extern crate extra;
use extra::test::BenchHarness;

static SP: u8 = ' ' as u8;
static NL: u8 = '\n' as u8;

/// Returns true if the line is a horizontal rule.
fn is_hrule(buf: &[u8]) -> bool {
    let mut buf = buf;

    if buf.len() < 3 { return false }

    // Skip up to three leading spaces.
    //
    // We don't need to care about tabs here as in this position they are
    // equivalent to 4 spaces, which means we would not parse this as a
    // hrule.
    if buf.head() == Some(&SP) { buf = buf.tail(); }
    if buf.head() == Some(&SP) { buf = buf.tail(); }
    if buf.head() == Some(&SP) { buf = buf.tail(); }

    // We need at least 3 items
    if buf.len() < 3 { return false } 

    let item = buf[0];

    if !(item == ('*' as u8) || item == ('-' as u8) || item == ('_' as u8)) {
        return false;
    }

    // The count of '*', '-' or '_'
    let mut cnt: uint = 0;

    for &ch in buf.iter() {
        if ch == item {
            cnt += 1;
        } else if ch == NL {
            break;
        } else if ch != SP {
            return false;
        }
    }

    return cnt >= 3;
}

#[test]
fn test_is_hrule() {
    // examples as given on the markdown homepage
    assert_eq!(is_hrule(bytes!("* * *\n")), true);
    assert_eq!(is_hrule(bytes!("***\n")), true);
    assert_eq!(is_hrule(bytes!("*****\n")), true);
    assert_eq!(is_hrule(bytes!("- - -\n")), true);
    assert_eq!(is_hrule(bytes!("---------------------------------------\n")), true);

    // up to three spaces ignored
    assert_eq!(is_hrule(bytes!(" ***\n")), true);
    assert_eq!(is_hrule(bytes!("  ***\n")), true);
    assert_eq!(is_hrule(bytes!("   ***\n")), true);

    // but not four, or a tab which is equivalent to four spaces
    assert_eq!(is_hrule(bytes!("    ***\n")), false);
    assert_eq!(is_hrule(bytes!("\t***\n")), false);
 
    // need at least three
    assert_eq!(is_hrule(bytes!("*\n")), false);
    assert_eq!(is_hrule(bytes!("**\n")), false);
    assert_eq!(is_hrule(bytes!("* *\n")), false);
    assert_eq!(is_hrule(bytes!("   * *\n")), false);

    // Also works without newline at the end
    assert_eq!(is_hrule(bytes!("* * *")), true);

    // And underscores also supported
    assert_eq!(is_hrule(bytes!("___")), true);
    assert_eq!(is_hrule(bytes!("______________")), true);
    assert_eq!(is_hrule(bytes!(" ______________")), true);
}

#[bench]
fn bench_is_hrule(b: &mut BenchHarness) {
    let s = bytes!("   * * * * * * * * * * * * * * * *\n");
    b.iter(|| is_hrule(s));
}
