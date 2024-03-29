use chrono::{Duration, Local, NaiveDate};
use rand::prelude::SliceRandom;
use rand::Rng;

use crate::common::constant::{EMAIL_SUFFIX, NAMES, NAMES_EN, PHONE_PREFIX, SURNAMES, SURNAMES_EN};

pub fn random_phone() -> String {
    let numbers: Vec<char> = "0123456789".chars().collect();
    let mut result = String::with_capacity(8);
    let mut rng = rand::thread_rng();
    for _ in 0..8 {
        result.push(*numbers.choose(&mut rng).unwrap());
    }
    result.insert_str(0, PHONE_PREFIX.choose(&mut rand::thread_rng()).unwrap());
    result
}

pub fn random_pin_yin() -> String {
    format!(
        "{}{}",
        NAMES_EN.choose(&mut rand::thread_rng()).unwrap(),
        SURNAMES_EN.choose(&mut rand::thread_rng()).unwrap()
    )
}

pub fn random_email() -> String {
    random_pin_yin() + EMAIL_SUFFIX.choose(&mut rand::thread_rng()).unwrap()
}

pub fn random_name() -> String {
    format!(
        "{}{}",
        NAMES.choose(&mut rand::thread_rng()).unwrap(),
        SURNAMES.choose(&mut rand::thread_rng()).unwrap()
    )
}

pub fn random_password() -> String {
    let numbers: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ#@!$%^&*"
        .chars()
        .collect();
    let mut result = String::with_capacity(8);
    let mut rng = rand::thread_rng();
    for _ in 0..16 {
        result.push(*numbers.choose(&mut rng).unwrap());
    }

    result
}

pub fn smart_text(label: &str) -> String {
    match label {
        s if s.contains("密码") || s.contains("password") => random_password(),
        s if s.contains("邮箱") || s.contains("email") => random_email(),
        s if s.contains("生日") || s.contains("日期") => random_date(),
        s if s.contains("手机") || s.contains("电话") || s.contains("phone") => random_phone(),
        _ => random_name(),
    }
}

pub fn random_date() -> String {
    let start_date = NaiveDate::from_ymd_opt(1999, 1, 1).unwrap();
    let end_date = Local::now().date_naive();

    let mut rng = rand::thread_rng();
    let random_days = rng.gen_range(0..=(end_date - start_date).num_days());

    let random_date = start_date + Duration::days(random_days);
    random_date.format("%Y-%m-%d").to_string()
}

#[cfg(test)]
mod tests {
    use crate::common;
    use crate::common::form::{
        random_date, random_email, random_name, random_password, random_phone, random_pin_yin,
        smart_text,
    };

    #[test]
    fn random_x() {
        common::load("form", "files/form.toml");
        println!("{}", random_pin_yin());
        println!("{}", random_name());
        println!("{}", random_email());
        println!("{}", random_phone());
        println!("{}", random_password());
        println!("{}", random_date());

        for x in &["密码", "手机", "邮箱", "日期"] {
            println!("{}", smart_text(x));
        }
    }

    #[test]
    fn random_phone_test() {
        common::load("form", "files/form.toml");
        assert_eq!(random_phone().len(), 11)
    }
}
