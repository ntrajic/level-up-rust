use std::fs;
use std::path;

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn is_readable(&self) -> bool {
        fs::File::open(self).is_ok()
    }

    fn is_writeable(&self) -> bool {
        fs::metadata(self)
            .map(|m| !m.permissions().readonly())
            .unwrap_or(false)
    }

    fn exists(&self) -> bool {
        self.exists()
    }
}

fn main() {
    // 
}

#[test]
fn writeable() {
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();


    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
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
//
// running 2 tests
// ..
// test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


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