// This code is heavily inspired by hoedown.

//
// Performance observations:
//
// Note that buf.head() == Some(&ch) is much faster than buf[0] == ch.
//

#[cfg(test)] extern crate extra;
#[cfg(test)] use extra::test::BenchHarness;

static SP: u8 = ' ' as u8;
static NL: u8 = '\n' as u8;
static CR: u8 = '\r' as u8;
static TAB: u8 = '\t' as u8;
static STAR: u8 = '*' as u8;
static DASH: u8 = '-' as u8;
static UNDERSCORE: u8 = '_' as u8;

//
// Skip up to three leading spaces.
//
// Up to three leading spaces are allowed for many elements.
//
// We don't need to care about a TAB here as in this position it is equivalent
// to 4 spaces, which means that when we find a TAB here we would not parse the
// corresponding element.
//
fn skip_initial_three_spaces<'a>(buf: &'a[u8])-> &'a[u8] {
    let mut buf = buf;
    if buf.head() == Some(&SP) { buf = buf.tail(); }
    if buf.head() == Some(&SP) { buf = buf.tail(); }
    if buf.head() == Some(&SP) { buf = buf.tail(); }
    return buf;
}

//
// Return Some(`rem`) if the line is a horizontal rule, with `rem` being the
// buf after the hrule. Otherwise return None.
//
fn is_hrule<'a>(buf: &'a[u8]) -> Option<&'a[u8]> {
    let buf = skip_initial_three_spaces(buf);

    let item = match buf.head() {
        Some(&c) if c == STAR ||
                    c == DASH ||
                    c == UNDERSCORE => c,
        _                           => return None
    };

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

    // Test if the remaining buf actually works.
    let s = bytes!("   * * *\nremaining");
    let res = is_hrule(s);
    assert!(res.is_some());
    assert_eq!(res.unwrap(), bytes!("remaining"));
}

#[bench]
fn bench_is_hrule(b: &mut BenchHarness) {
    let s = bytes!("   * * * * * * * * * * * * * * * *\n");
    b.iter(|| is_hrule(s));
}

//
// Return Some(`rem`) if the line is an empty line, with `rem` being the buf
// after the empty line. Otherwise return None.
//
fn is_empty<'a>(buf: &'a[u8]) -> Option<&'a[u8]> {
    let mut cnt: uint = 0;

    for &ch in buf.iter() {
        if      ch == NL  { cnt += 1; break; }
        else if ch == CR  { cnt += 1; }
        else if ch == SP  { cnt += 1; }
        else if ch == TAB { cnt += 1; }
        else              { return None; }
    }

    if cnt > 0 {
        Some(buf.slice_from(cnt))
    } else {
        None
    }
}

#[test]
fn test_is_empty() {
    assert!(is_empty(bytes!("\n")).is_some());
    assert!(is_empty(bytes!("    \n")).is_some());
    assert!(is_empty(bytes!("  \t  \n")).is_some());
    assert!(is_empty(bytes!("  \t  \r\n")).is_some());
    assert!(is_empty(bytes!("  \t  \nabc")).is_some());
    assert!(is_empty(bytes!("  \t  ")).is_some());

    assert!(is_empty(bytes!("a")).is_none());
    assert!(is_empty(bytes!(" a")).is_none());
    assert!(is_empty(bytes!(" a\n")).is_none());
    assert!(is_empty(bytes!(" \ta\n")).is_none());

    // Test if the remaining buf actually works.
    let s = bytes!("   \t\r\nremaining");
    let res = is_empty(s);
    assert!(res.is_some());
    assert_eq!(res.unwrap(), bytes!("remaining"));
}
