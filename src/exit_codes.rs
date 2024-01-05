// renamed exit to quit so we may name our own exit function as exit
use std::process::exit as quit;

pub(crate) fn exit(code: i32) -> ! {
    // This is where we will internally document exit codes
    // 0 is of course a normal A-OK status
    // 1 is reserved for general purpose, which should only be thrown by libraries
    // 2 is a failure to parse a provided KDL document and we cannot proceed
    quit(code); // quit is std::process::exit, just renamed so we may call our own function exit
}