pub fn today_as_string() -> String {
    let today = chrono::Local::now();
    today.format("%d %B %Y").to_string()
}
