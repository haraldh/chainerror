use chainerror::prelude::v1::*;
use std::error::Error;
use std::io;

#[test]
fn test_iter() -> Result<(), Box<dyn Error + Send + Sync>> {
    use std::fmt::Write;
    let err: Result<(), _> = Err(io::Error::from(io::ErrorKind::NotFound));
    let err = err.context("1");
    let err = err.context("2");
    let err = err.context("3");
    let err = err.context("4");
    let err = err.context("5");
    let err = err.context("6");
    let err = err.err().unwrap();

    let mut res = String::new();

    for e in err.iter() {
        write!(res, "{}", e)?;
    }
    assert_eq!(res, "654321entity not found");

    let io_error: Option<&io::Error> = err
        .iter()
        .filter_map(<dyn Error>::downcast_ref::<io::Error>)
        .next();

    assert_eq!(io_error.unwrap().kind(), io::ErrorKind::NotFound);

    Ok(())
}

#[test]
fn test_iter_alternate() -> Result<(), Box<dyn Error + Send + Sync>> {
    let err: Result<(), _> = Err(io::Error::from(io::ErrorKind::NotFound));
    let err = err.context("1");
    let err = err.context("2");
    let err = err.context("3");
    let err = err.context("4");
    let err = err.context("5");
    let err = err.context("6");
    let err = err.err().unwrap();

    let res = format!("{:#}", err);

    assert_eq!(res, format!("6\nCaused by:\n  5\nCaused by:\n  4\nCaused by:\n  3\nCaused by:\n  2\nCaused by:\n  1\nCaused by:\n  {:#}", io::Error::from(io::ErrorKind::NotFound)));

    let io_error: Option<&io::Error> = err
        .iter()
        .filter_map(<dyn Error>::downcast_ref::<io::Error>)
        .next();

    assert_eq!(io_error.unwrap().kind(), io::ErrorKind::NotFound);

    Ok(())
}

#[test]
fn test_find_cause() -> Result<(), Box<dyn Error + Send + Sync>> {
    let err: Result<(), _> = Err(io::Error::from(io::ErrorKind::NotFound));
    let err = err.context("1");
    let err = err.context("2");
    let err = err.context("3");
    let err = err.context("4");
    let err = err.context("5");
    let err = err.context("6");
    let err = err.err().unwrap();

    let io_error: Option<&io::Error> = err.find_cause::<io::Error>();

    assert_eq!(io_error.unwrap().kind(), io::ErrorKind::NotFound);

    Ok(())
}

#[test]
fn test_root_cause() -> Result<(), Box<dyn Error + Send + Sync>> {
    let err: Result<(), _> = Err(io::Error::from(io::ErrorKind::NotFound));
    let err = err.context("1");
    let err = err.context("2");
    let err = err.context("3");
    let err = err.context("4");
    let err = err.context("5");
    let err = err.context("6");
    let err = err.err().unwrap();

    let err: Option<&(dyn std::error::Error + 'static)> = err.root_cause();
    let io_error: Option<&io::Error> = err.and_then(<dyn Error>::downcast_ref::<io::Error>);

    assert_eq!(io_error.unwrap().kind(), io::ErrorKind::NotFound);

    Ok(())
}
