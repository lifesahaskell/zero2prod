use validator::Validate;

#[derive(Debug, Validate)]
pub struct SubscriberEmail {
    #[validate(email)]
    email: String
}


impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        let email = SubscriberEmail {email: s.clone()};

        match email.validate() {
            Ok(_) => Ok(email),
            Err(_) => Err(format!("{} is not a valid subscriber email.", s))
        }
    }
}

impl AsRef<str> for SubscriberEmail{
    fn as_ref(&self) -> &str {
        &self.email
    }
}



#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claims::assert_err;
    use claims::assert_ok;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    
    #[test]
    fn email_missing_domain_is_rejected() {
        let email = "ursula@.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn valid_emails_are_parsed_successfully() {
        let email = SafeEmail().fake();
        assert_ok!(SubscriberEmail::parse(email));
    }

}
