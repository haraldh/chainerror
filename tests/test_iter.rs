use chainerror::*;
use std::error::Error;
use std::io;

#[cfg(not(feature = "display-cause"))]
#[test]
fn test_iter() -> Result<(), Box<dyn Error + Send + Sync>> {
    use std::fmt::Write;
    let err = io::Error::from(io::ErrorKind::NotFound);
    let err = cherr!(err, "1");
    let err = cherr!(err, "2");
    let err = cherr!(err, "3");
    let err = cherr!(err, "4");
    let err = cherr!(err, "5");
    let err = cherr!(err, "6");

    let mut res = String::new();

    for e in err.iter() {
        write!(res, "{}", e.to_string())?;
    }
    assert_eq!(res, "654321entity not found");

    let io_error: Option<&io::Error> = err
        .iter()
        .filter_map(Error::downcast_ref::<io::Error>)
        .next();

    assert_eq!(io_error.unwrap().kind(), io::ErrorKind::NotFound);

    Ok(())
}

#[cfg(feature = "display-cause")]
#[test]
fn test_iter() -> Result<(), Box<dyn Error + Send + Sync>> {
    let err = io::Error::from(io::ErrorKind::NotFound);
    let err = cherr!(err, "1");
    let err = cherr!(err, "2");
    let err = cherr!(err, "3");
    let err = cherr!(err, "4");
    let err = cherr!(err, "5");
    let err = cherr!(err, "6");

    let res = err.to_string();

    assert_eq!(res, "6\nCaused by:\n5\nCaused by:\n4\nCaused by:\n3\nCaused by:\n2\nCaused by:\n1\nCaused by:\nentity not found");

    let io_error: Option<&io::Error> = err
        .iter()
        .filter_map(Error::downcast_ref::<io::Error>)
        .next();

    assert_eq!(io_error.unwrap().kind(), io::ErrorKind::NotFound);

    Ok(())
}

#[test]
fn test_find_cause() -> Result<(), Box<dyn Error + Send + Sync>> {
    let err = io::Error::from(io::ErrorKind::NotFound);
    let err = cherr!(err, "1");
    let err = cherr!(err, "2");
    let err = cherr!(err, "3");
    let err = cherr!(err, "4");
    let err = cherr!(err, "5");
    let err = cherr!(err, "6");

    let io_error: Option<&io::Error> = err.find_cause::<io::Error>();

    assert_eq!(io_error.unwrap().kind(), io::ErrorKind::NotFound);

    Ok(())
}

#[test]
fn test_root_cause() -> Result<(), Box<dyn Error + Send + Sync>> {
    let err = io::Error::from(io::ErrorKind::NotFound);
    let err = cherr!(err, "1");
    let err = cherr!(err, "2");
    let err = cherr!(err, "3");
    let err = cherr!(err, "4");
    let err = cherr!(err, "5");
    let err = cherr!(err, "6");

    let err: Option<&(dyn std::error::Error + 'static)> = err.root_cause();
    let io_error: Option<&io::Error> = err.and_then(Error::downcast_ref::<io::Error>);

    assert_eq!(io_error.unwrap().kind(), io::ErrorKind::NotFound);

    Ok(())
}
