use chainerror::Context;

#[test]
fn test_basic() {
    use std::path::PathBuf;
    type BoxedError = Box<dyn std::error::Error + Send + Sync>;
    fn read_config_file(path: PathBuf) -> Result<(), BoxedError> {
        // do stuff, return other errors
        let _buf = std::fs::read_to_string(&path).context(format!("Reading file: {:?}", &path))?;
        // do stuff, return other errors
        Ok(())
    }
    fn process_config_file() -> Result<(), BoxedError> {
        // do stuff, return other errors
        read_config_file("_non_existent.txt".into()).context("read the config file")?;
        // do stuff, return other errors
        Ok(())
    }

    if let Err(e) = process_config_file() {
        let os_notfound_error = std::io::Error::from_raw_os_error(2);
        let s = format!("{:?}", e);
        let lines = s.lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), 5);
        assert!(lines[0].starts_with("tests/test_basic.rs:"));
        assert_eq!(lines[1], "Caused by:");
        assert!(lines[2].starts_with("tests/test_basic.rs:"));
        assert_eq!(lines[3], "Caused by:");
        assert_eq!(lines[4], format!("{:?}", os_notfound_error));
    } else {
        panic!();
    }
}
