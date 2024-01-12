use once_cell::sync::Lazy;
use regex::Regex;

static FILTERS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"^javascript.*").unwrap(),
        Regex::new(r"(?i)data:image/*").unwrap(),
        Regex::new(r"(?i)\.(sha1|pom|svg|rpm|js|3ds|3g2|3gp|7z|ds_store|aac|adp|ai|aif|aiff|apk|ar|asf|au|avi|bak|bin|bk|bmp|btif|bz2|css|cab|caf|cgm|cmx|sub|swf|tar|tbz2|tga|tgz|cpio|cr2|dat|deb|djvu|dll|dmg|dmp|dng|doc|docx|dot|dotx|dra)([^A-Za-z]|$)").unwrap(),
        Regex::new(r"(?i)\.(dsk|dts|dtshd|dvb|dwg|dxf|ear|ecelp4800|ecelp7470|ecelp9600|egg|tif|tiff|tlz|flv|fpx|fst|fvt|g3|gif|gz|h261|h263|h264|ico|ief|image|img|ipa|iso|jar|jpeg|jpg|jpgv|jpm|jxr|epub|exe|f4v|fbs|fh|fla|flac|fli|eol|eot)([^A-Za-z]|$)").unwrap(),
        Regex::new(r"(?i)\.(ktx|lvp|lz|lzma|lzo|m3u|m4a|m4v|mar|mdi|mid|mj2|mka|mkv|mmr|mng|mov|movie|mp3|mp4|mp4a|mpeg|mpg|mpga|mxu|nef|npx|o|oga|ogg|ogv|otf|psd|pya|pyc|pyo|pyv|qt|rar|ras|raw|rgb|rip|rlc|rz|s3m|s7z|scm|scpt|sgi|shar|sil|smv|so)([^A-Za-z]|$)").unwrap(),
        Regex::new(r"(?i)\.(pbm|pcx|pdf|pea|pgm|pic|png|pnm|ppm|pps|ppt|pptx|ps|ts|ttf|uvh|uvi|uvm|uvp|uvs|uvu|viv|vob|war|wav|wax|wbmp|wdp|weba|webm|webp|whl|wm|wma|wmv|wmx|woff|woff2|wvx|xbm|xif|xls|xlsx|xlt|xm|xpi|xpm|xwd|xz|z|zip|zipx)([^A-Za-z]|$)").unwrap(),
    ]
});

const IGNORE_KEYWORDS: [&str; 10] = [
    "logout",
    "quit",
    "exit",
    "sigoff",
    "sigout",
    "logoff",
    "mailto:wvs@",
    "mailto:",
    "data:image",
    "javascript",
];

pub fn matching_filter(d: &str) -> bool {
    if FILTERS.iter().any(|f| f.is_match(d)) {
        return false;
    }

    let v = d.trim();
    match v {
        "" | "#" => false,
        _ => {
            for x in IGNORE_KEYWORDS {
                if v.contains(x) {
                    return false;
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::filter::matching_filter;

    #[test]
    fn matching_filter_test() {
        pub struct Value {
            v: String,
            result: bool,
        }

        let list = vec![
            Value {
                v: String::from("#"),
                result: false,
            },
            Value {
                v: String::from("mailto:"),
                result: false,
            },
            Value {
                v: String::from("data:image"),
                result: false,
            },
            Value {
                v: String::from("example.com"),
                result: true,
            },
            Value {
                v: String::from("example.com/1"),
                result: true,
            },
            Value {
                v: String::from("javascript"),
                result: false,
            },
            Value {
                v: String::from("example.com/1.sha1"),
                result: false,
            },
        ];

        for t in list {
            assert_eq!(matching_filter(t.v.as_str()), t.result)
        }
    }
}
