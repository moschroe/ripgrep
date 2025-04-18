use crate::hay::SHERLOCK;
use crate::util::{cmd_exists, sort_lines, Dir, TestCommand};

// This file contains "miscellaneous" tests that were either written before
// features were tracked more explicitly, or were simply written without
// linking them to a specific issue number. We should try to minimize the
// addition of more tests in this file and instead add them to either the
// regression test suite or the feature test suite (found in regression.rs and
// feature.rs, respectively).

rgtest!(head_bytes_full, |dir: Dir, mut cmd: TestCommand| {
    dir.create("sherlock", SHERLOCK);

    let expected = "\
For the Doctor Watsons of this world, as opposed to the Sherlock
";
    eqnice!(
        expected,
        cmd.arg("Sherlock").arg("sherlock").arg("--max-bytes=64").stdout()
    );
});

rgtest!(head_bytes_limit, |dir: Dir, mut cmd: TestCommand| {
    dir.create("sherlock", SHERLOCK);

    cmd.arg("-F")
        .arg("Sherlock")
        .arg("sherlock")
        // .arg("--mmap")
        .arg("--max-bytes=63")
        .arg("--trace")
        .assert_exit_code(1);
});
