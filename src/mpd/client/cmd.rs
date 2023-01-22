pub enum Command {
    Add { uri: String },
    Load { name: String },
    Pause,
    Stop,
    Playid { song_id: Option<i64> },
    Next,
    Previous,
    Idle { subsystems: Vec<String> },
    Noidle,
    Clear,
    Deleteid { songid: i64 },
    Count { filter: String },
    Lsinfo { uri: String },
    Search { filter: String },
    Playlistinfo,
    Listplaylistinfo { name: String },
    Rm { name: String },
    Playlistdelete { name: String, songpos: usize, },
    Listplaylists,
    Status,
    Password { str: String },
    CommandList(Vec<Command>),
    Update { uri: Option<String> },
    Seekcur { time: String },
    Setvol { vol: u8 },
    Albumart { uri: String, offset: usize },
    Readpicture { uri: String, offset: usize },
    Repeat { state: String },
    Consume { state: String },
    Random { state: String },
    Single { state: String },
}

trait ToStringOrEmpty<T: ToString> {
    fn to_string_or_empty(&self) -> String;
}

impl<T: ToString> ToStringOrEmpty<T> for Option<T> {
    fn to_string_or_empty(&self) -> String {
        self.as_ref().map(ToString::to_string)
            .unwrap_or_else(|| "".to_owned())
    }
}

fn escape(arg: &str) -> String {
    let mut result = String::with_capacity(arg.len());

    for c in arg.chars() {
        if let '"' | '\\' = c {
            result.push('\\');
        }

        result.push(c);
    }

    result
}

fn quote(arg: &str) -> String {
    let arg = escape(arg);

    format!("\"{arg}\"")
}

impl Command {
    const ADD_VALUE: &'static str = "add";
    const LOAD_VALUE: &'static str = "load";
    const PLAYID_VALUE: &'static str = "playid";
    const NEXT_VALUE: &'static str = "next";
    const PREVIOUS_VALUE: &'static str = "previous";
    const IDLE_VALUE: &'static str = "idle";
    const PAUSE_VALUE: &'static str = "pause";
    const STOP_VALUE: &'static str = "stop";
    const NOIDLE_VALUE: &'static str = "noidle";
    const CLEAR_VALUE: &'static str = "clear";
    const DELETEID_VALUE: &'static str = "deleteid";
    const COUNT_VALUE: &'static str = "count";
    const LSINFO_VALUE: &'static str = "lsinfo";
    const SEARCH_VALUE: &'static str = "search";
    const STATUS_VALUE: &'static str = "status";
    const PASSWORD_VALUE: &'static str = "password";
    const PLAYLISTINFO_VALUE: &'static str = "playlistinfo";
    const LISTPLAYLISTINFO_VALUE: &'static str = "listplaylistinfo";
    const LISTPLAYLISTS_VALUE: &'static str = "listplaylists";
    const RM_VALUE: &'static str = "rm";
    const PLAYLISTDELETE_VALUE: &'static str = "playlistdelete";
    const UPDATE_VALUE: &'static str = "update";
    const SEEKCUR_VALUE: &'static str = "seekcur";
    const SETVOL_VALUE: &'static str = "setvol";
    const ALBUMART_VALUE: &'static str = "albumart";
    const READPICTURE_VALUE: &'static str = "readpicture";
    const REPEAT_VALUE: &'static str = "repeat";
    const CONSUME_VALUE: &'static str = "consume";
    const RANDOM_VALUE: &'static str = "random";
    const SINGLE_VALUE: &'static str = "single";
    const COMMAND_LIST_BEGIN_VALUE: &'static str = "command_list_begin";
    const COMMAND_LIST_END_VALUE: &'static str = "command_list_end";

    pub fn into_prepared(self) -> String {
        use Command::*;

        match self {
            Add { uri } => {
                format!("{} {}", Command::ADD_VALUE, quote(&uri))
            }
            Load { name } => {
                format!("{} {}", Command::LOAD_VALUE, quote(&name))
            }
            Playid { song_id } => {
                format!("{} {}", Command::PLAYID_VALUE, song_id.to_string_or_empty())
            }
            Pause => {
                Command::PAUSE_VALUE.to_owned()
            }
            Stop => {
                Command::STOP_VALUE.to_owned()
            }
            Next => {
                Command::NEXT_VALUE.to_owned()
            }
            Previous => {
                Command::PREVIOUS_VALUE.to_owned()
            }
            Idle { subsystems } => {
                let subsystems = subsystems.into_iter()
                    .map(|s| quote(&s))
                    .collect::<Vec<_>>()
                    .join(" ");

                format!("{} {}", Command::IDLE_VALUE.to_owned(), subsystems)
            }
            Noidle => {
                Command::NOIDLE_VALUE.to_owned()
            }
            Clear => {
                Command::CLEAR_VALUE.to_owned()
            }
            Deleteid { songid } => {
                format!("{} {songid}", Command::DELETEID_VALUE)
            }
            Count { filter } => {
                format!("{} {}", Command::COUNT_VALUE, quote(&filter))
            }
            Lsinfo { uri } => {
                format!("{} {}", Command::LSINFO_VALUE, quote(&uri))
            }
            Search { filter } => {
                format!("{} {}", Command::SEARCH_VALUE, quote(&filter))
            }
            Status => {
                Command::STATUS_VALUE.to_owned()
            }
            Password { str } => {
                format!("{} {}", Command::PASSWORD_VALUE, quote(&str))
            }
            Playlistinfo => {
                Command::PLAYLISTINFO_VALUE.to_owned()
            }
            Listplaylistinfo { name } => {
                format!("{} {}", Command::LISTPLAYLISTINFO_VALUE, quote(&name))
            }
            Listplaylists => {
                Command::LISTPLAYLISTS_VALUE.to_owned()
            }
            Rm { name } => {
                format!("{} {}", Command::RM_VALUE, quote(&name))
            }
            Playlistdelete { name, songpos } => {
                format!("{} {} {songpos}", Command::PLAYLISTDELETE_VALUE, quote(&name))
            }
            Update { uri } => {
                if let Some(uri) = uri {
                    format!("{} {}", Command::UPDATE_VALUE, quote(&uri))
                } else {
                    Command::UPDATE_VALUE.to_owned()
                }
            }
            Seekcur { time } => {
                format!("{} {}", Command::SEEKCUR_VALUE, quote(&time))
            }
            Setvol { vol } => {
                format!("{} {vol}", Command::SETVOL_VALUE)
            }
            Albumart { uri, offset } => {
                format!("{} {} {offset}", Command::ALBUMART_VALUE, quote(&uri))
            }
            Readpicture { uri, offset } => {
                format!("{} {} {offset}", Command::READPICTURE_VALUE, quote(&uri))
            }
            Repeat { state } => {
                format!("{} {}", Command::REPEAT_VALUE, quote(&state))
            }
            Consume { state } => {
                format!("{} {}", Command::CONSUME_VALUE, quote(&state))
            }
            Random { state } => {
                format!("{} {}", Command::RANDOM_VALUE, quote(&state))
            }
            Single { state } => {
                format!("{} {}", Command::SINGLE_VALUE, quote(&state))
            }
            CommandList(xs) => {
                format!(
                    "{}\n{}{}",
                    Command::COMMAND_LIST_BEGIN_VALUE,
                    xs.into_iter()
                        .map(|x| format!("{}\n", x.into_prepared()))
                        .collect::<String>(),
                    Command::COMMAND_LIST_END_VALUE,
                )
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_construct_command_list() {
        let expected = "
            command_list_begin\n\
            clear\n\
            add \"test/dir\"\n\
            playid 2\n\
            command_list_end
        ".trim();

        let actual = Command::CommandList(
            vec![
                Command::Clear,
                Command::Add {
                    uri: "test/dir".to_owned(),
                },
                Command::Playid {
                    song_id: Some(2),
                },
            ],
        ).into_prepared();

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_quote_argument() {
        let expected = r#""(Artist == \"foo\\'bar\\\"\")""#;

        let actual = quote(r#"(Artist == "foo\'bar\"")"#);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_construct_find() {
        let expected = r#"search "(Artist == \"foo\\'bar\\\"\")""#;

        let actual = Command::Search {
            filter: r#"(Artist == "foo\'bar\"")"#.to_owned()
        }.into_prepared();

        assert_eq!(actual, expected);
    }
}
