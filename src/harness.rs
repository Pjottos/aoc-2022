use criterion::Criterion;
use reqwest::{blocking::ClientBuilder, cookie};

use std::{
    alloc::{alloc, dealloc, handle_alloc_error, Layout},
    cell::RefCell,
    env,
    fmt::Debug,
    fs,
    marker::PhantomData,
    ptr::NonNull,
    sync::Arc,
};

pub struct Harness<E, M> {
    day: Option<u32>,
    extract_func: Option<E>,
    input_blob: Option<(NonNull<u8>, usize)>,
    criterion: Option<RefCell<Criterion>>,
    _phantom: PhantomData<M>,
}

impl<E, M> Drop for Harness<E, M> {
    fn drop(&mut self) {
        self.free_input_blob();
    }
}

impl<E, M> Harness<E, M> {
    fn input_layout(len: usize) -> Layout {
        let align = 64;
        let size = (len + align - 1) & !(align - 1);
        Layout::from_size_align(size, align).unwrap()
    }

    fn free_input_blob(&mut self) {
        if let Some((ptr, len)) = self.input_blob.take() {
            unsafe { dealloc(ptr.as_ptr(), Self::input_layout(len)) }
        }
    }

    fn create_input_blob(&mut self, data: &[u8]) {
        assert!(!data.is_empty());

        self.free_input_blob();
        let layout = Self::input_layout(data.len());
        let ptr = unsafe { alloc(layout) };
        let ptr = NonNull::new(ptr).unwrap_or_else(|| handle_alloc_error(layout));
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), ptr.as_ptr(), data.len());
        }
        self.input_blob = Some((ptr, data.len()));
    }
}

impl<'a, ET: 'a, EF: Fn(&'a str) -> ET> Harness<EF, ET> {
    pub fn begin() -> Self {
        let criterion = env::args()
            .any(|arg| arg == "--bench")
            .then(|| RefCell::new(Criterion::default().with_output_color(true)));

        Self {
            day: None,
            extract_func: None,
            input_blob: None,
            criterion,
            _phantom: PhantomData,
        }
    }

    pub fn day(mut self, day: u32) -> Self {
        self.day = Some(day);
        self
    }

    pub fn input_override<I: AsRef<str>>(mut self, input_override: I) -> Self {
        self.create_input_blob(input_override.as_ref().as_bytes());
        self
    }

    pub fn extract(mut self, extract_func: EF) -> Self {
        let day = self.day.unwrap();
        if self.input_blob.is_none() {
            let input_path = format!("inputs/{}.txt", day);
            let text = fs::read_to_string(&input_path).unwrap_or_else(|_| {
                let text = download_input(day);
                fs::write(&input_path, &text).unwrap();
                text
            });
            self.create_input_blob(text.as_bytes());
        }

        self.extract_func = Some(extract_func);
        self
    }

    pub fn run_part<F, R>(self, part_num: u32, part_func: F) -> Self
    where
        F: Fn(ET) -> R,
        R: Debug,
    {
        let extract_func = self
            .extract_func
            .as_ref()
            .expect("extract function should be set when running a part");
        let input_text = unsafe {
            let (ptr, size) = self.input_blob.unwrap();
            // SAFETY: utf8 validity is enforced when loading the input
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr.as_ptr(), size))
        };

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
