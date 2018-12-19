use std::error::Error;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::path::Path;

use chainerror::prelude::*;

#[derive(Clone, PartialEq, Debug)]
enum ParseError {
    InvalidValue(u32),
    InvalidParameter(String),
    NoOpen,
    NoClose,
}

impl ::std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            ParseError::InvalidValue(a) => write!(f, "InvalidValue: {}", a),
            ParseError::InvalidParameter(a) => write!(f, "InvalidParameter: '{}'", a),
            ParseError::NoOpen => write!(f, "No opening '{{' in config file"),
            ParseError::NoClose => write!(f, "No closing '}}' in config file"),
        }
    }
}

fn parse_config(c: String) -> Result<(), Box<Error>> {
    if !c.starts_with('{') {
        Err(cherr!(ParseError::NoOpen))?;
    }
    if !c.ends_with('}') {
        Err(cherr!(ParseError::NoClose))?;
    }
    let c = &c[1..(c.len() - 1)];
    let v = c
        .parse::<u32>()
        .map_err(|e| cherr!(e, ParseError::InvalidParameter(c.into())))?;
    if v > 100 {
        Err(cherr!(ParseError::InvalidValue(v)))?;
    }
    Ok(())
}

derive_str_cherr!(ConfigFileError);
derive_str_cherr!(SeriousError);
derive_str_cherr!(FileError);
derive_str_cherr!(AppError);

fn file_reader(_filename: &Path) -> Result<(), Box<Error>> {
    Err(IoError::from(IoErrorKind::NotFound)).map_err(mstrerr!(FileError, "File reader error"))?;
    Ok(())
}

fn read_config(filename: &Path) -> Result<(), Box<Error>> {
    if filename.eq(Path::new("global.ini")) {
        // assume we got an IO error
        file_reader(filename).map_err(mstrerr!(
            ConfigFileError,
            "Error reading file {:?}",
            filename
        ))?;
    }
    // assume we read some buffer
    if filename.eq(Path::new("local.ini")) {
        let buf = String::from("{1000}");
        parse_config(buf)?;
    }

    if filename.eq(Path::new("user.ini")) {
        let buf = String::from("foo");
        parse_config(buf)?;
    }

    if filename.eq(Path::new("user2.ini")) {
        let buf = String::from("{foo");
        parse_config(buf)?;
    }

    if filename.eq(Path::new("user3.ini")) {
        let buf = String::from("{foo}");
        parse_config(buf)?;
    }

    if filename.eq(Path::new("custom.ini")) {
        let buf = String::from("{10}");
        parse_config(buf)?;
    }

    if filename.eq(Path::new("essential.ini")) {
        Err(cherr!(SeriousError("Something is really wrong".into())))?;
    }

    Ok(())
}

fn read_verbose_config(p: &str) -> Result<(), Box<Error>> {
    eprintln!("Reading '{}' ... ", p);
    read_config(Path::new(p)).map_err(mstrerr!(AppError, "{}", p))?;
    eprintln!("Ok reading {}", p);
    Ok(())
}

fn start_app(debug: bool) -> Result<(), Box<Error>> {
    for p in &[
        "global.ini",
        "local.ini",
        "user.ini",
        "user2.ini",
        "user3.ini",
        "custom.ini",
        "essential.ini",
    ] {
        if let Err(e) = read_verbose_config(p) {
            assert!(e.is_chain::<AppError>());
            let app_err = e.downcast_chain_ref::<AppError>().unwrap();

            if app_err.find_kind::<SeriousError>().is_some() {
                // Bail out on SeriousError
                eprintln!("---> Serious Error:\n{:?}", e);
                Err(cherr!(e, AppError("Seriously".into())))?;
            } else if let Some(cfg_error) = app_err.find_kind::<ConfigFileError>() {
                if debug {
                    eprintln!("{:?}\n", cfg_error);
                } else {
                    // Deep Error handling
                    if let Some(chioerror) = cfg_error.find_kind::<IoError>() {
                        let ioerror = chioerror.kind();
                        match ioerror.kind() {
                            IoErrorKind::NotFound => {
                                eprint!("Ignoring missing file");
                                if let Some(root_cause) = cfg_error.root_cause() {
                                    eprint!(", because of: {}\n", root_cause);
                                }
                                eprintln!();
                            }
                            _ => Err(cherr!(e, AppError("Unhandled IOError".into())))?,
                        }
                    } else {
                        eprintln!("ConfigFileError for: {}", e);
                    }
                }
            } else {
                if debug {
                    eprintln!("Error reading:\n{:?}\n", e)
                } else {
                    eprintln!("Error reading: {}\n", e)
                }
            }
        }
        eprintln!();
    }
    Ok(())
}


fn main() -> Result<(), Box<Error>> {
    eprintln!("Display:\n");
    let e = start_app(false).unwrap_err();
    assert!(e.is_chain::<AppError>());
    eprintln!("\n\n==================================");
    eprintln!("====    Debug output");
    eprintln!("==================================\n");
    let r = start_app(true);

    eprintln!("\n\n==================================");
    eprintln!("====    Main return output");
    eprintln!("==================================\n");
    r
}
