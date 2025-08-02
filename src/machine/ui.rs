pub mod note {
    pub struct Tag {
        data: String
    }
    impl Tag {
        fn read(&self) -> String {
            format!("[{}]", self.data.clone())
        }
    }
    impl From<&str> for Tag {
        fn from(value: &str) -> Tag {
            Tag { data: String::from(value) }
        }
    }
    impl From<String> for Tag {
        fn from(value: String) -> Tag {
            Tag { data: value }
        }
    }

    pub struct Note {
        tag: Tag,
        content: String
    }
    impl Note {
        fn get_presentable_string(&self) -> String {
            format!("{} {}", self.tag.read(), self.content.clone())
        }

        pub fn present(&self) {
            println!("{}", self.get_presentable_string())
        }
        pub fn present_error(&self) {
            eprintln!("{}", self.get_presentable_string())
        }

        pub fn from(tag: &str, content: String) -> Note {
            Note {
                tag: Tag::from(tag),
                content: String::from(content)
            }
        }
    }

    pub trait Issuer {
        /// Issue a note
        fn issue(&self) -> Note;
    }

}