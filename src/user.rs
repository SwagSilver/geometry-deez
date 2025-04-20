// All user credential parsing algorithms were figured out by tinkering with
// their respective fields in the in-game account registration panel

use std::time::Instant;

const NAME_LEN_MIN: usize = 3;
const NAME_LEN_MAX: usize = 14;

#[derive(PartialEq, Debug)]
pub struct Name(String);

#[derive(PartialEq, Debug)]
pub enum NameError {
    Empty,
    TooShort,
}

fn filter_chars(input: &str, predicate: impl FnMut(&char) -> bool) -> String {
    input.chars().filter(predicate).collect()
}

impl Name {
    pub fn parse(name: &str) -> Result<Self, NameError> {
        let sanitized = filter_chars(name, char::is_ascii_alphanumeric);

        if sanitized.is_empty() {
            return Err(NameError::Empty);
        }

        let sanitized_len = sanitized.len();

        if sanitized_len < NAME_LEN_MIN {
            return Err(NameError::TooShort);
        }

        if sanitized_len > NAME_LEN_MAX {
            return Ok(Self(sanitized[0..NAME_LEN_MAX].to_owned()));
        }

        Ok(Self(sanitized))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

const PASSWORD_LEN_MIN: usize = 6;
const PASSWORD_LEN_MAX: usize = 19;

#[derive(PartialEq, Debug)]
pub struct Password(String);

#[derive(PartialEq, Debug)]
pub enum PasswordError {
    Empty,
    TooShort,
}

const PASSWORD_ALLOWED_SPECIAL_CHARS: &'static str = "-_";

impl Password {
    pub fn parse(password: &str) -> Result<Self, PasswordError> {
        let sanitized = filter_chars(
            password,
            |ch| {
                ch.is_ascii_alphanumeric() ||
                PASSWORD_ALLOWED_SPECIAL_CHARS.contains(*ch)
            }
        );

        if sanitized.is_empty() {
            return Err(PasswordError::Empty);
        }

        let sanitized_len = sanitized.len();

        if sanitized_len < PASSWORD_LEN_MIN {
            return Err(PasswordError::TooShort);
        }

        if sanitized_len > PASSWORD_LEN_MAX {
            return Ok(Self(sanitized[0..PASSWORD_LEN_MAX].to_owned()));
        }

        Ok(Self(sanitized))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

const EMAIL_LEN_MIN: usize = 4;
const EMAIL_LEN_MAX: usize = 49;

#[derive(PartialEq, Debug)]
pub struct Email(String);

#[derive(PartialEq, Debug)]
pub enum EmailError {
    Empty,
    TooShort,
    Malformed,
}

const EMAIL_ALLOWED_SPECIAL_CHARS: &'static str = "-_@.";

impl Email {
    pub fn parse(email: &str) -> Result<Self, EmailError> {
        let sanitized = filter_chars(
            email,
            |ch| {
                ch.is_ascii_alphanumeric()
                || EMAIL_ALLOWED_SPECIAL_CHARS.contains(*ch)
            }
        );

        if sanitized.is_empty() {
            return Err(EmailError::Empty);
        }

        let at = if let Some(index) = Self::find_last(&sanitized, '@') {
            index
        } else {
            return Err(EmailError::Malformed);
        };

        if at == 0 {
            return Err(EmailError::Malformed);
        }

        let dot = if let Some(index) = Self::find_last(&sanitized[at..], '.') {
            index + at
        } else {
            return Err(EmailError::Malformed);
        };

        let sanitized_len = sanitized.len();

        if dot + 1 == sanitized_len {
            return Err(EmailError::Malformed);
        }

        if sanitized[0..at].chars().all(|ch| ch.is_ascii_digit()) {
            return Err(EmailError::Malformed);
        }

        let first_char = sanitized
            .chars()
            .nth(0)
            .expect("email already checked for being empty");

        if !first_char.is_ascii_alphabetic() {
            return Err(EmailError::Malformed);
        }

        if sanitized_len < EMAIL_LEN_MIN {
            return Err(EmailError::TooShort);
        }

        if sanitized_len > EMAIL_LEN_MAX {
            return Ok(Self(sanitized[0..EMAIL_LEN_MAX].to_owned()));
        }

        Ok(Self(sanitized))
    }

    fn find_last(input: &str, ch: char) -> Option<usize> {
        input
        .chars()
        .rev()
        .collect::<String>()
        .find(ch)
        .map(|i| input.len() - i - 1)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub struct SocialMediaHandles {
    youtube: Option<String>,
    // Renamed to X but the game still displays it as Twitter,
    // nothing we can do about that
    twitter: Option<String>,
    twitch: Option<String>,
}

const HANDLE_ALLOWED_SPECIAL_CHARS: &'static str = "-_,' ";

impl SocialMediaHandles {
    pub fn new(youtube: &str, twitter: &str, twitch: &str) -> Self {
        Self {
            youtube: Self::sanitize_social_media_handle(youtube),
            twitter: Self::sanitize_social_media_handle(twitter),
            twitch: Self::sanitize_social_media_handle(twitch),
        }
    }

    pub fn youtube(&self) -> Option<String> {
        self.youtube.clone()
    }

    pub fn set_youtube(&mut self, youtube: &str) -> &mut Self {
        self.youtube = Self::sanitize_social_media_handle(youtube);
        self
    }

    pub fn twitter(&self) -> Option<String> {
        self.twitter.clone()
    }

    pub fn set_twitter(&mut self, twitter: &str) -> &mut Self {
        self.twitter = Self::sanitize_social_media_handle(twitter);
        self
    }

    pub fn twitch(&self) -> Option<String> {
        self.twitch.clone()
    }

    pub fn set_twitch(&mut self, twitch: &str) -> &mut Self {
        self.twitch = Self::sanitize_social_media_handle(twitch);
        self
    }

    fn sanitize_social_media_handle(handle: &str) -> Option<String> {
        let sanitized = filter_chars(
            handle,
            |ch| {
                ch.is_ascii_alphanumeric()
                || HANDLE_ALLOWED_SPECIAL_CHARS.contains(*ch)
            }
        );

        match sanitized.as_str() {
            "" => None,
            _ => Some(sanitized),
        }
    }
}

#[derive(Default)]
pub struct IconSet {
    pub icon_id: u32,
    pub ship_id: u32,
    pub jetpack_id: u32,
    pub ball_id: u32,
    pub ufo_id: u32,
    pub wave_id: u32,
    pub robot_id: u32,
    pub spider_id: u32,
    pub swing_id: u32,
    pub glow_id: u32,
    pub death_effect_id: u32,
}

#[derive(Default)]
pub struct Stats {
    pub stars: u32,
    pub moons: u32,
    pub coins: u32,
    pub user_coins: u32,
    pub diamonds: u32,
    pub demons: u32,
    pub creator_points: u32,
    pub orbs: u32,
}

#[derive(Default)]
#[repr(u8)]
pub enum Ban {
    #[default]
    None,
    LeaderboardBan,
    CreatorBan,
    LeaderboardAndCreatorBan,
}

pub enum AllowMessagesFrom {}
pub enum AllowFriendRequestsFrom {}
pub enum DisplayCommentHistoryTo {}

pub struct User {
    pub id: u64,
    pub name: Name,
    pub email: Email,
    pub social_media_handles: SocialMediaHandles,
    pub created_at: Instant,
}

impl User {
    pub fn new(
        id: u64,
        name: Name,
        email: Email,
        social_media_handles: SocialMediaHandles,
        created_at: Instant
    ) -> Self {
        Self {
            id,
            name,
            email,
            social_media_handles,
            created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_parsing() -> Result<(), NameError> {
        assert_eq!(Name::parse("-/&#"), Err(NameError::Empty));
        assert_eq!(Name::parse("a^@*("), Err(NameError::TooShort));
        assert_eq!(Name::parse("babygronk???")?.as_str(), "babygronk");
        Ok(())
    }

    #[test]
    fn test_password_parsing() -> Result<(), PasswordError> {
        assert_eq!(Password::parse("/##><&#"), Err(PasswordError::Empty));
        assert_eq!(Password::parse("a^@*(-"), Err(PasswordError::TooShort));
        assert_eq!(Password::parse("_deez-nuts???")?.as_str(), "_deez-nuts");
        Ok(())
    }

    #[test]
    fn test_email_parsing() -> Result<(), EmailError> {
        assert_eq!(Email::parse("+%÷>)"), Err(EmailError::Empty));
        assert_eq!(Email::parse("foo@."), Err(EmailError::Malformed));
        assert_eq!(Email::parse("foo@.@"), Err(EmailError::Malformed));
        assert_eq!(Email::parse("1_@.d"), Err(EmailError::Malformed));
        assert!(Email::parse("a_@.d").is_ok());
        Ok(())
    }

    #[test]
    fn test_social_media_handles() {
        let mut handles = SocialMediaHandles::new("```", "-_,' ", "~xd");
        assert_eq!(handles.youtube(), None);
        assert_eq!(handles.twitter(), Some("-_,' ".to_string()));
        assert_eq!(handles.twitch(), Some("xd".to_string()));

        handles.set_youtube("xd").set_twitter("").set_twitch("-_,' ");
        assert_eq!(handles.youtube(), Some("xd".to_string()));
        assert_eq!(handles.twitter(), None);
        assert_eq!(handles.twitch(), Some("-_,' ".to_string()));
    }
}
