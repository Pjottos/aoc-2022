use criterion::Criterion;
use reqwest::{blocking::ClientBuilder, cookie};

use std::{cell::RefCell, env, fmt::Debug, fs, marker::PhantomData, sync::Arc};

pub struct Harness<E, M> {
    day: Option<u32>,
    extract_func: Option<E>,
    input_text: Option<String>,
    criterion: Option<RefCell<Criterion>>,
    _phantom: PhantomData<M>,
}

impl<'a, ET: 'a, EF: Fn(&'a str) -> ET> Harness<EF, ET> {
    pub fn begin() -> Self {
        let criterion = env::args()
            .any(|arg| arg == "--bench")
            .then(|| RefCell::new(Criterion::default().with_output_color(true)));

        Self {
            day: None,
            extract_func: None,
            input_text: None,
            criterion,
            _phantom: PhantomData,
        }
    }

    pub fn day(&mut self, day: u32) -> &mut Self {
        self.day = Some(day);
        self
    }

    pub fn input_override<S: Into<String>>(&mut self, input_override: S) -> &mut Self {
        self.input_text = Some(input_override.into());
        self
    }

    pub fn extract(&mut self, extract_func: EF) -> &Self {
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

        self.extract_func = Some(extract_func);
        self
    }

    pub fn run_part<F, R>(&'a self, part_num: u32, part_func: F) -> &Self
    where
        F: Fn(ET) -> R,
        R: Debug,
    {
        let extract_func = self
            .extract_func
            .as_ref()
            .expect("extract function should be set when running a part");
        let input_text = self.input_text.as_ref().unwrap();

        let res = part_func(extract_func(input_text));
        println!("Part {}: {:?}", part_num, res);

        if let Some(criterion) = self.criterion.as_ref() {
            criterion.borrow_mut().bench_function(
                &format!("day {} part {}", self.day.unwrap(), part_num),
                |b| b.iter(|| part_func(extract_func(input_text))),
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
