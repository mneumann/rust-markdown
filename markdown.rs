// This code is heavily inspired by hoedown.


extern crate extra;
use extra::test::BenchHarness;


static SP: u8 = ' ' as u8;
static NL: u8 = '\n' as u8;

/// Returns true if the line is a horizontal rule.
fn is_hrule(buf: &[u8]) -> bool {
    let len = buf.len();

    if len < 3 { return false }

    let mut pos: uint = 0;

    // Skip up to three leading spaces.
    //
    // We don't need to care about tabs here as in this position they are
    // equivalent to 4 spaces.
    if buf[0] == SP {
        pos += 1;
        if buf[1] == SP {
            pos += 1;
            if buf[2] == SP {
                pos += 1;
            }
        }
    }

    // We need at least 3 items
    if pos + 2 >= len { return false } 

    let item = buf[pos];

    if !(item == ('*' as u8) || item == ('-' as u8) || item == ('_' as u8)) {
        return false;
    }

    let mut cnt: uint = 0; // The count of '*', '-' or '_'
    for &ch in buf.slice_from(pos).iter() {
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
