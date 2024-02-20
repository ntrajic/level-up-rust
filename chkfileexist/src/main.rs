use std::path;

trait FileMetadata {                                // implement interface/trait FileMetadata w/ 3 functions
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn is_readable(&self) -> bool {
        todo!();
    }

    fn is_writeable(&self) -> bool {
        todo!();
    }

    fn exists(&self) -> bool {
        todo!();
    }
}

fn main() {
    // 
}

#[test]
fn writeable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);

    fs::remove_file(f.path()).unwrap();
}



// $ make test
// cargo test --quiet
// error[E0432]: unresolved import `tempfile`
//   --> src/main.rs:32:9
//    |
// 32 |     use tempfile;
//    |         ^^^^^^^^ no `tempfile` in the root
//
// error[E0432]: unresolved import `tempfile`
//   --> src/main.rs:43:9
//    |
// 43 |     use tempfile;
//    |         ^^^^^^^^ no `tempfile` in the root
//
// For more information about this error, try `rustc --explain E0432`.
// error: could not compile `chkfileexist` (bin "chkfileexist" test) due to 2 previous errors
// make: *** [Makefile:16: test] Error 101