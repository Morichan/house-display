extern crate reqwest;
extern crate serde_json;

use std::io::Read;
use url::Url;
use chrono::Local;
use std::fs;
use std::error::Error;
use serde_json::Value;

pub struct ApiAgent {
    ekispert_url: String,
    ekispert_api: String,
    access_key: String,
    access_key_file_path: String,
}

impl ApiAgent {
    pub fn new() -> Self {
        ApiAgent {
            ekispert_url: "https://roote.ekispert.net/ja/result".to_string(),
            ekispert_api: "http://api.ekispert.jp/v1/json/search/course/light".to_string(),
            access_key: String::new(),
            access_key_file_path: "access_key.txt".to_string(),
        }
    }

    pub fn search_train_time(&mut self) -> String {
        let mut resp = reqwest::get(&self.create_url(EkispertUrl::CreatingBasedOnWeb))
            .unwrap();
        let mut s = String::new();

        resp.read_to_string(&mut s).unwrap();
        println!("{}", resp.url().as_str());

        return s;
    }

    fn create_url(&mut self, url: EkispertUrl) -> String {
        match url {
            EkispertUrl::GettingFromAPI => self.create_url_to_use_api(),
            EkispertUrl::CreatingBasedOnWeb => self.create_url_to_scrape(),
        }
    }

    fn create_url_to_scrape(&self) -> String {
        let mut url = Url::parse(&self.ekispert_url).unwrap();
        let now = self.now_time();

        url.query_pairs_mut().clear()
            .append_pair("dep", "豊島園(都営線)")
            .append_pair("dep_code", "22836")
            .append_pair("arr", "都庁前")
            .append_pair("arr_code", "29213")

            .append_pair("yyyymm", &now.year_and_month)
            .append_pair("day", &now.day)
            .append_pair("hour", &now.hour)
            .append_pair("minute10", &now.min10)
            .append_pair("minute1", &now.min1)

            .append_pair("locale", "ja")
            .append_pair("connect", "true")
            .append_pair("highway", "true")
            .append_pair("liner", "true")
            .append_pair("local", "true")
            .append_pair("plane", "true")
            .append_pair("shinkansen", "true")
            .append_pair("ship", "true")

            .append_pair("sort", "time")
            .append_pair("submit_btn", "検索")
            .append_pair("surcharge", "3")
            .append_pair("ticket_type", "0")
            .append_pair("transfer", "2")
            .append_pair("type", "dep")
            .append_pair("utf8", "✓");

        return String::from(url.as_str());
    }

    fn create_url_to_use_api(&mut self) -> String {
        let mut api = Url::parse(&self.ekispert_api).unwrap();
        let now = self.now_time();

        self.read_access_key();

        api.query_pairs_mut().clear()
            .append_pair("key", &self.access_key)
            .append_pair("from", "22836")
            .append_pair("to", "29213")
            .append_pair("date", &format!("{}{}", &now.year_and_month, &now.day))
            .append_pair("time", &format!("{}{}{}", &now.hour, &now.min10, &now.min1));

        let mut resp = reqwest::get(api.as_str())
            .unwrap();
        let mut s = String::new();

        resp.read_to_string(&mut s).unwrap();

        let v: Value = serde_json::from_str(&s).unwrap();

        return v["ResultSet"]["ResourceURI"].to_string().replace('"', "");
    }

    fn now_time(&self) -> Now {
        let local_now = Local::now();
        let minutes_string = local_now.format("%M").to_string();
        let mut minutes = minutes_string.chars();

        return Now {
            year_and_month: local_now.format("%Y%m").to_string(),
            day: local_now.format("%-d").to_string(),
            hour: local_now.format("%-H").to_string(),
            min10: minutes.nth(0).unwrap().to_string(), // 10の位
            min1: minutes.nth(0).unwrap().to_string(), // 1の位
        };
    }

    fn read_access_key(&mut self) {
        if let Err(e) = self.read_file(&self.access_key_file_path.to_string()) {
            panic!("Error: {}", e);
        }

        let file_lines: Vec<&str> = self.access_key.split_whitespace().collect();

        if let Some(line) = file_lines.get(0) {
            self.access_key = line.to_string();
        };
    }

    fn read_file(&mut self, filename: &str) -> Result<(), Box<Error>> {
        self.access_key = fs::read_to_string(filename)?;

        Ok(())
    }
}

struct Now {
    year_and_month: String,
    day: String,
    hour: String,
    min10: String,
    min1: String,
}

enum EkispertUrl {
    GettingFromAPI,
    CreatingBasedOnWeb,
}

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

#[cfg(test)]
speculate! {
    before {
        let mut obj = ApiAgent::new();
    }

    it "should get now time" {
        let expected = Local::now();

        let actual = obj.now_time();

        assert_eq!(expected.format("%Y%m").to_string(), actual.year_and_month);
        assert_eq!(expected.format("%-d").to_string(), actual.day);
        assert_eq!(expected.format("%-H").to_string(), actual.hour);
        assert_eq!(expected.format("%M").to_string(), format!("{}{}", actual.min10, actual.min1));
    }

    it "should create Ekispert URL" {
        let test_time = obj.now_time();
        let expected = Url::parse(&format!(
                "https://roote.ekispert.net/ja/result?dep=豊島園%28都営線%29&dep_code=22836&arr=都庁前&arr_code=29213&yyyymm={}&day={}&hour={}&minute10={}&minute1={}&locale=ja&connect=true&highway=true&liner=true&local=true&plane=true&shinkansen=true&ship=true&sort=time&submit_btn=検索&surcharge=3&ticket_type=0&transfer=2&type=dep&utf8=%E2%9C%93",
                test_time.year_and_month,
                test_time.day,
                test_time.hour,
                test_time.min10,
                test_time.min1)).unwrap();

        let actual = Url::parse(&obj.create_url(EkispertUrl::CreatingBasedOnWeb))
            .unwrap();

        assert_eq!(expected, actual);
    }

    #[ignore]
    it "should create Ekispert API URL" {
        let test_time = obj.now_time();
        let expected = Url::parse(&format!(
                "https://roote.ekispert.net/result?arr=%E9%83%BD%E5%BA%81%E5%89%8D&arr_code=29213&connect=true&dep=%E8%B1%8A%E5%B3%B6%E5%9C%92(%E9%83%BD%E5%96%B6%E7%B7%9A)&dep_code=22836&express=true&highway=true&hour={}&liner=true&local=true&minute={}{}&plane=true&shinkansen=true&ship=true&sleep=false&sort=time&surcharge=3&type=dep&via1=&via1_code=&via2=&via2_code=&yyyymmdd={}{}",
                test_time.hour,
                test_time.min10,
                test_time.min1,
                test_time.year_and_month,
                test_time.day)).unwrap();

        let actual = Url::parse(&obj.create_url(EkispertUrl::GettingFromAPI))
            .unwrap();

        assert_eq!(expected, actual);
    }

    #[ignore]
    it "should search same web site" {
        let mut resp = reqwest::get(&obj.create_url(EkispertUrl::CreatingBasedOnWeb))
            .unwrap();
        let mut expected = String::new();
        resp.read_to_string(&mut expected).unwrap();

        let mut resp = reqwest::get(&obj.create_url(EkispertUrl::GettingFromAPI))
            .unwrap();
        let mut actual = String::new();
        resp.read_to_string(&mut actual).unwrap();

        assert_eq!(expected, actual);
    }

    #[should_panic(expected = "File not found: ")]
    it "should throw exception to read nothing file" {

        if let Err(e) = obj.read_file("This_is_nothing.txt") {
            panic!("File not found: {}", e);
        }
    }

    it "should read file" {

        if let Err(e) = obj.read_file("access_key.txt") {
            panic!("Error: {}", e);
        }
    }

    it "is not have newline char from end" {

        obj.read_access_key();
        let actual = &obj.access_key;

        assert!(!actual.ends_with("\n"));
    }
}
