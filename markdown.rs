// This code is heavily inspired by hoedown.

//
// Performance observations:
//
// Note that buf.head() == Some(&ch) is much faster than buf[0] == ch.
//

static SP: u8 = b' ';
static NL: u8 = b'\n';
static CR: u8 = b'\r';
static TAB: u8 = b'\t';
static STAR: u8 = b'*';
static DASH: u8 = b'-';
static UNDERSCORE: u8 = b'_';
static TILDE: u8 = b'~';
static BACKTICK: u8 = b'`';

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
    if buf.head() == Some(&SP) { buf = buf.tail();
    if buf.head() == Some(&SP) { buf = buf.tail();
    if buf.head() == Some(&SP) { buf = buf.tail(); } } }
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
    assert!(is_hrule(b"* * *\n").is_some());
    assert!(is_hrule(b"***\n").is_some());
    assert!(is_hrule(b"*****\n").is_some());
    assert!(is_hrule(b"- - -\n").is_some());
    assert!(is_hrule(b"---------------------------------------\n").is_some());

    // up to three spaces ignored
    assert!(is_hrule(b" ***\n").is_some());
    assert!(is_hrule(b"  ***\n").is_some());
    assert!(is_hrule(b"   ***\n").is_some());

    // but not four, or a tab which is equivalent to four spaces
    assert!(is_hrule(b"    ***\n").is_none());
    assert!(is_hrule(b"\t***\n").is_none());
 
    // need at least three
    assert!(is_hrule(b"*\n").is_none());
    assert!(is_hrule(b"**\n").is_none());
    assert!(is_hrule(b"* *\n").is_none());
    assert!(is_hrule(b"   * *\n").is_none());

    // Also works without newline at the end
    assert!(is_hrule(b"* * *").is_some());

    // And underscores also supported
    assert!(is_hrule(b"___").is_some());
    assert!(is_hrule(b"______________").is_some());
    assert!(is_hrule(b" ______________").is_some());

    // Test if the remaining buf actually works.
    let s = b"   * * *\nremaining";
    let res = is_hrule(s);
    assert!(res.is_some());
    assert_eq!(res.unwrap(), b"remaining");
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
    assert!(is_empty(b"\n").is_some());
    assert!(is_empty(b"    \n").is_some());
    assert!(is_empty(b"  \t  \n").is_some());
    assert!(is_empty(b"  \t  \r\n").is_some());
    assert!(is_empty(b"  \t  \nabc").is_some());
    assert!(is_empty(b"  \t  ").is_some());

    assert!(is_empty(b"a").is_none());
    assert!(is_empty(b" a").is_none());
    assert!(is_empty(b" a\n").is_none());
    assert!(is_empty(b" \ta\n").is_none());

    // Test if the remaining buf actually works.
    let s = b"   \t\r\nremaining";
    let res = is_empty(s);
    assert!(res.is_some());
    assert_eq!(res.unwrap(), b"remaining");
}

fn is_codefence<'a>(buf: &'a[u8]) -> Option<(&'a[u8], uint, u8)> {
    let buf = skip_initial_three_spaces(buf);

    let item = match buf.head() {
        Some(&c) if c == TILDE ||
                    c == BACKTICK => c,
        _                         => return None
    };

    // The count of '~' or '`' characters
    let mut cnt: uint = 0;

    for &ch in buf.iter() {
        if ch == item { cnt += 1; }
        else          { break; }
    }

    if cnt >= 3 {
        Some((buf.slice_from(cnt), cnt, item))
    } else {
        None
    }
}

#[test]
fn test_is_codefence() {
    assert!(is_codefence(b"```").is_some());
    assert!(is_codefence(b"~~~").is_some());
    assert!(is_codefence(b"`````````").is_some());
    assert!(is_codefence(b"~~~~").is_some());
    assert!(is_codefence(b"   ```").is_some());
    assert!(is_codefence(b"  ~~~").is_some());

    assert!(is_codefence(b"  ~~").is_none());
    assert!(is_codefence(b" ``").is_none());
    assert!(is_codefence(b"    ```").is_none());
    assert!(is_codefence(b"\t```").is_none());

    // Test if the remaining buf actually works.
    let s = b"   ```remaining\n";
    let res = is_codefence(s);
    assert!(res.is_some());
    assert_eq!(res.unwrap(), (b"remaining\n", 3, BACKTICK));

    let s = b"   ~~~~~~~~remaining\n";
    let res = is_codefence(s);
    assert!(res.is_some());
    assert_eq!(res.unwrap(), (b"remaining\n", 8, TILDE));
}


