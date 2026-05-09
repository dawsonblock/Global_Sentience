//! Tests for Archive backends and memory claims.
//! Requirements: jsonl_archive_never_mv2, archive_extension_validation

use memory::archive::ArchiveError;

#[test]
fn archive_error_extension_rejected() {
    // Requirement: ArchiveError must have ExtensionRejected variant
    let err = ArchiveError::ExtensionRejected("test.mv2".to_string());

    let display = format!("{}", err);
    assert!(display.contains("extension rejected"));

    let debug = format!("{:?}", err);
    assert!(debug.contains("ExtensionRejected"));
}

#[test]
fn archive_error_not_implemented() {
    // Requirement: ArchiveError must have NotImplemented variant
    let err = ArchiveError::NotImplemented;

    let display = format!("{}", err);
    assert!(display.contains("not implemented"));

    let debug = format!("{:?}", err);
    assert!(debug.contains("NotImplemented"));
}

#[test]
fn archive_error_io_error() {
    // Requirement: ArchiveError must support IO errors
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let err = ArchiveError::Io(io_err);

    let display = format!("{}", err);
    assert!(display.len() > 0); // IO error message should be present

    let debug = format!("{:?}", err);
    assert!(debug.contains("Io"));
}
