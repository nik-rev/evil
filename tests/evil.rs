/// You can call `?` on a [`Result`] from a context that returns [`evil::Result`]
#[test]
fn std_result_context_evil_result() -> evil::Result<()> {
    let x: Result<(), ()> = Err(());

    x?;

    evil::Ok(())
}

/// You can call `?` on an [`Option`] from a context that returns [`evil::Result`]
#[test]
fn std_option_context_evil_result() -> evil::Result<()> {
    let x: Option<()> = None;

    x?;

    evil::Ok(())
}

/// You can call `?` on an [`evil::Result`] from a context that returns [`evil::Result`]
#[test]
fn evil_result_context_evil_result() -> evil::Result<()> {
    let x: evil::Result<()> = std_option_context_evil_result();

    x?;

    evil::Result::Ok(())
}

/// You can call `?` on a [`evil::Result`] from a context that returns [`Result`]
#[test]
fn evil_result_context_std_result() -> Result<(), ()> {
    let x: evil::Result<()> = std_option_context_evil_result();

    x?;

    std::result::Result::Ok(())
}
