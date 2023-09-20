use crate::if_ultimate_version;
use crate::gui::defaults::*;
use crate::gui::tools::Message;
use crate::instruments::DataStore;

use iced::widget::Column;
use serde::{Deserialize, Serialize};

pub enum AuthResult {
    Successful(u8),
    Invalid(String)
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Authorization {
    login: String,
    password: String
}

impl Authorization {
    pub fn from(login: String, password: String) -> Self {
        Authorization { login, password }
    }

    pub fn is_authorized(&self) -> bool {
        // TODO: Implement server api.

        false
    }
}

impl Default for Authorization {
    fn default() -> Self {
        let unreg = "unregistered";
        Self::from(unreg.to_owned(), unreg.to_owned())
    }
}

pub fn get_auth_new_page<'a>(data: &DataStore, auth: &Authorization) -> Option<Column<'a, Message>> {
    if_ultimate_version! {{
        if auth.is_authorized() {
            return None;
        }

        let disclamer = get_default_text(
            "Для пользования Ultimate версией приложения Вам необходимо иметь \
            логин и пароль лицензионного продукта. Введите их в поля ниже.".to_owned()
        );
        let login_field = get_default_text_input(
            "Логин:", &data.container.cell_1, Message::UpdateCell1
        );
        let password_field = get_default_text_input(
            "Пароль:", &data.container.cell_2, Message::UpdateCell2
        );
        let login_button = get_default_button(
            "Войти", Message::CheckAuth
        );

        let auth_page = get_default_column()
            .push(disclamer)
            .push(login_field)
            .push(password_field)
            .push(login_button);

        Some(auth_page)
    } else {
        None
    }}
}
