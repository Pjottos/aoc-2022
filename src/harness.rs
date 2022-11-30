use criterion::Criterion;
use reqwest::{blocking::ClientBuilder, cookie};

use std::{cell::RefCell, env, fmt::Debug, fs, sync::Arc};

pub struct Harness<E> {
    day: Option<u32>,
    extracted_input: Option<E>,
    input_text: Option<String>,
    criterion: Option<RefCell<Criterion>>,
}

impl<'a, E> Harness<E> {
    pub fn begin() -> Self {
        let criterion = env::args()
            .any(|arg| arg == "--bench")
            .then(|| RefCell::new(Criterion::default().with_output_color(true)));

        Self {
            day: None,
            extracted_input: None,
            input_text: None,
            criterion,
        }
    }

    pub fn day(&'a mut self, day: u32) -> &'a mut Self {
        self.day = Some(day);
        self
    }

    pub fn input_override<S: Into<String>>(&'a mut self, input_override: S) -> &'a mut Self {
        self.input_text = Some(input_override.into());
        self
    }

    pub fn extract<X>(&'a mut self, extractor: X) -> &'a Self
    where
        X: Fn(&'a str) -> E,
    {
        let day = self.day.unwrap();
        if self.input_text.is_none() {
            let input_path = format!("inputs/{}.txt", day);
            let text = fs::read_to_string(&input_path).unwrap_or_else(|_| {
                let text = download_input(day);
                fs::write(&input_path, &text).unwrap();
                text
            });
            self.input_text = Some(text);
        }
        let text = self.input_text.as_ref().unwrap();

        self.extracted_input = Some(extractor(text));
        if let Some(criterion) = self.criterion.as_ref() {
            criterion
                .borrow_mut()
                .bench_function(&format!("day {} extract", day), |b| {
                    b.iter(|| extractor(text))
                });
        }

        self
    }

    pub fn run_part<F, R>(&'a self, part_num: u32, func: F) -> &'a Self
    where
        F: Fn(&E) -> R,
        R: Debug,
    {
        let input = self
            .extracted_input
            .as_ref()
            .expect("input not extracted yet");

        let res = func(input);
        println!("Part {}: {:?}", part_num, res);

        if let Some(criterion) = self.criterion.as_ref() {
            criterion.borrow_mut().bench_function(
                &format!("day {} part {}", self.day.unwrap(), part_num),
                |b| b.iter(|| func(input)),
            );
        }

        self
    }
}

fn download_input(day: u32) -> String {
    const YEAR: u32 = 2022;

    let jar = Arc::new(cookie::Jar::default());
    let session = fs::read_to_string("session")
        .expect("`session` file needs to be readable when downloading inputs");
    jar.add_cookie_str(&session, &"https://adventofcode.com".parse().unwrap());
    let client = ClientBuilder::new()
        .cookie_provider(jar)
        .gzip(true)
        .build()
        .unwrap();

    client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            YEAR, day
        ))
        .send()
        .unwrap()
        .error_for_status()
        .unwrap()
        .text()
        .unwrap()
}
