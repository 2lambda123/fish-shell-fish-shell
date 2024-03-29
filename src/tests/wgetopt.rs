use crate::wchar::prelude::*;
use crate::wcstringutil::join_strings;
use crate::wgetopt::{wgetopter_t, wopt, woption, woption_argument_t};

#[test]
fn test_wgetopt() {
    // Regression test for a crash.
    const short_options: &wstr = L!("-a");
    const long_options: &[woption] = &[wopt(L!("add"), woption_argument_t::no_argument, 'a')];
    let mut argv = [
        L!("abbr"),
        L!("--add"),
        L!("emacsnw"),
        L!("emacs"),
        L!("-nw"),
    ];
    let mut w = wgetopter_t::new(short_options, long_options, &mut argv);
    let mut a_count = 0;
    let mut arguments = vec![];
    while let Some(opt) = w.wgetopt_long() {
        match opt {
            'a' => {
                a_count += 1;
            }
            '\x01' => {
                // non-option argument
                arguments.push(w.woptarg.as_ref().unwrap().to_owned());
            }
            '?' => {
                // unrecognized option
                if let Some(arg) = w.argv.get(w.woptind - 1) {
                    arguments.push(arg.to_owned());
                }
            }
            _ => {
                panic!("unexpected option: {:?}", opt);
            }
        }
    }
    assert_eq!(a_count, 1);
    assert_eq!(arguments.len(), 3);
    assert_eq!(join_strings(&arguments, ' '), "emacsnw emacs -nw");
}
