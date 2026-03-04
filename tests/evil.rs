#![allow(unused)]

/// You can call `?` on a [`Result`] from a context that returns [`evil::Result`]
#[test]
#[should_panic]
fn std_result_context_evil_result() {
    fn inner() -> evil::Result<()> {
        let x: Result<(), ()> = Err(());

        x?;

        evil::Ok(())
    }

    inner();
}

/// You can call `?` on an [`Option`] from a context that returns [`evil::Result`]
#[test]
#[should_panic]
fn std_option_context_evil_result() {
    fn inner() -> evil::Result<()> {
        let x: Option<()> = None;

        x?;

        evil::Ok(())
    }

    inner();
}

/// You can call `?` on an [`evil::Result`] from a context that returns [`evil::Result`]
#[test]
fn evil_result_context_evil_result() -> evil::Result<()> {
    let x: evil::Result<()> = evil::Ok(());

    x?;

    evil::Result::Ok(())
}

/// You can call `?` on a [`evil::Result`] from a context that returns [`Result`]
#[test]
fn evil_result_context_std_result() -> Result<(), ()> {
    let x: evil::Result<()> = evil::Ok(());

    x?;

    std::result::Result::Ok(())
}

/// Extract the value from a [`Result`] using `?`
#[test]
fn extract_value_from_std_result() -> evil::Result<()> {
    let x: Result<i32, &str> = Ok(42);

    let value: i32 = x?;

    assert_eq!(value, 42);
    evil::Ok(())
}

/// Extract the value from an [`Option`] using `?`
#[test]
fn extract_value_from_std_option() -> evil::Result<()> {
    let x: Option<String> = Some("hello".to_string());

    let value: String = x?;

    assert_eq!(value, "hello");
    evil::Ok(())
}

/// Extract the value from a [`evil::Result`] using `?`
#[test]
fn extract_value_from_evil_result() -> evil::Result<()> {
    let x: evil::Result<f64> = evil::Ok(3.0);

    let value: f64 = x?;

    assert_eq!(value, 3.0);
    evil::Ok(())
}
