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

/// Returns Some(`rem`) if the line is a horizontal rule, with `rem` being the
/// buf after the hrule. Otherwise returns None.
fn is_hrule<'a>(buf: &'a[u8]) -> Option<&'a[u8]> {
    let mut buf = buf;

    if buf.len() < 3 { return None }

    // Skip up to three leading spaces.
    //
    // We don't need to care about tabs here as in this position they are
    // equivalent to 4 spaces, which means we would not parse this as a
    // hrule.
    if buf.head() == Some(&SP) { buf = buf.tail(); }
    if buf.head() == Some(&SP) { buf = buf.tail(); }
    if buf.head() == Some(&SP) { buf = buf.tail(); }

    // We need at least 3 items
    if buf.len() < 3 { return None } 

    let item = buf[0];

    if !(item == ('*' as u8) || item == ('-' as u8) || item == ('_' as u8)) {
        return None;
    }

    // The count of '*', '-' or '_'
    let mut cnt: uint = 0;

    // Counts the consumed spaces (and the final NL)
    let mut spc: uint = 0;

    for &ch in buf.iter() {
        if      ch == item { cnt += 1; }
        else if ch == NL   { spc += 1; break; }
        else if ch == SP   { spc += 1; }
        else               { return None; }
    }

    if cnt >= 3 {
        Some(buf.slice_from(cnt + spc))
    } else {
        None
    }
}

#[test]
fn test_is_hrule() {
    // examples as given on the markdown homepage
    assert!(is_hrule(bytes!("* * *\n")).is_some());
    assert!(is_hrule(bytes!("***\n")).is_some());
    assert!(is_hrule(bytes!("*****\n")).is_some());
    assert!(is_hrule(bytes!("- - -\n")).is_some());
    assert!(is_hrule(bytes!("---------------------------------------\n")).is_some());

    // up to three spaces ignored
    assert!(is_hrule(bytes!(" ***\n")).is_some());
    assert!(is_hrule(bytes!("  ***\n")).is_some());
    assert!(is_hrule(bytes!("   ***\n")).is_some());

    // but not four, or a tab which is equivalent to four spaces

    assert!(is_hrule(bytes!("    ***\n")).is_none());
    assert!(is_hrule(bytes!("\t***\n")).is_none());
 
    // need at least three
    assert!(is_hrule(bytes!("*\n")).is_none());
    assert!(is_hrule(bytes!("**\n")).is_none());
    assert!(is_hrule(bytes!("* *\n")).is_none());
    assert!(is_hrule(bytes!("   * *\n")).is_none());

    // Also works without newline at the end
    assert!(is_hrule(bytes!("* * *")).is_some());

    // And underscores also supported
    assert!(is_hrule(bytes!("___")).is_some());
    assert!(is_hrule(bytes!("______________")).is_some());
    assert!(is_hrule(bytes!(" ______________")).is_some());
}

#[bench]
fn bench_is_hrule(b: &mut BenchHarness) {
    let s = bytes!("   * * * * * * * * * * * * * * * *\n");
    b.iter(|| is_hrule(s));
}
