use bson::DateTime;
use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;

pub struct Credentials {
    pub name: String,
    pub token: String,
}

pub struct Cookies(HtmlDocument);
impl Cookies {
    pub fn set_credentials(name: &str, token: &str, expiration: &DateTime) -> Result<(), ()> {
        Cookies::new().map_or(Err(()), |cookies| {
            cookies.set("name", name, expiration)?;
            cookies.set("token", token, expiration)?;
            Ok(())
        })
    }
    pub fn credentials() -> Result<Credentials, ()> {
        Cookies::new().map_or(Err(()), |cookies| {
            Ok(Credentials {
                name: cookies.get("name")?,
                token: cookies.get("token")?,
            })
        })
    }

    fn new() -> Result<Self, ()> {
        Ok(Self(
            web_sys::window()
                .ok_or(())?
                .document()
                .ok_or(())?
                .dyn_into::<HtmlDocument>()
                .map_err(|_| ())?,
        ))
    }

    fn get(&self, key: &str) -> Result<String, ()> {
        let cookies = self.0.cookie().map_err(|_| ())?;

        let cookie = match cookies
            .split(';')
            .find(|cookie| cookie.trim_start().starts_with(&format!("{}=", key)))
        {
            Some(some) => some,
            None => return Err(()),
        };

        match cookie.split_once('=') {
            Some(value) => Ok(value.1.to_string()),
            None => Err(()),
        }
    }

    fn set(&self, key: &str, value: &str, expiration: &DateTime) -> Result<(), ()> {
        self.0
            .set_cookie(&format!(
                "{}={};expires={};path=/",
                key,
                value,
                expiration.try_to_rfc3339_string().map_err(|_| ())?
            ))
            .map_err(|_| ())
    }
}
