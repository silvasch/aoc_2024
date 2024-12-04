#[macro_export]
macro_rules! read_input {
    ($day:literal) => {
        include_str!(concat!(
            location_macros::workspace_dir!(),
            "/inputs/day-",
            $day,
            ".txt"
        ))
    };
}
