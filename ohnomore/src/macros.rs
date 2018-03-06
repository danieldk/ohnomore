macro_rules! ok_or {
    ($expr:expr, $stmt:stmt) => (match $expr {
        Some(val) => val,
        None => {$stmt}
    })
}
