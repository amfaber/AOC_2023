pub use once_cell::sync::Lazy;

const YEAR: usize = 2023;

const SESSION: &str = "session=53616c7465645f5fa4433fff50e0bb42860fd1404d06449e35a209e907b04e72ea1d15c5a67e0acf44d0228a527ea9f6db550cbd0facc1801a7ea5cba82e2974";

pub fn get_day_input(day: usize) -> String {
    match std::fs::read_to_string("input.txt"){
        Ok(output) => return output,
        Err(_) => {},
    }
    
    let url = format!("https://adventofcode.com/{YEAR}/day/{day}/input")
        .parse::<reqwest::Url>()
        .unwrap();
    let client = reqwest::Client::new();
    let tokio_rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap();
    let response = tokio_rt.block_on(async {
        client
            .get(url)
            .header("cookie", SESSION)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    });
    std::fs::write("input.txt", &response).unwrap();
    response
}


#[macro_export]
macro_rules! lazy_input {
    ($day:expr) => {
        static INPUT: Lazy<String> = Lazy::new(|| get_day_input($day));
    };
}

