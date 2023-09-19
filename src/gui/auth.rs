use crate::if_ultimate_version;
use crate::instruments::DataStore;
use super::defaults::*;
use super::tools::Message;

use iced::widget::Column;

pub type MasterKey = String;

pub enum AuthResult {
    Successful(u8),
    Invalid(String)
}

pub struct Authorization {
    master_key: MasterKey
}

impl From<MasterKey> for Authorization {
    fn from(value: MasterKey) -> Self {
        Authorization {
            master_key: value
        }
    }
}

impl Authorization {
    pub fn is_authorized(&self) -> bool {
        // TODO: Implement server api.

        true
    }
}

pub fn get_auth_new_page<'a>(data: &DataStore, auth: &Authorization) -> Option<Column<'a, Message>> {
    if_ultimate_version! {{
        if auth.is_authorized() {
            return None;
        }

        let disclamer = get_default_text(
            "Для пользования Ultimate версией приложения Вам необхрдимо иметь \
            мастер-ключ продукта. Введите его в поле ниже.".to_owned()
        );
        let master_key_field = get_default_text_input(
            "Введите Ваш мастер-ключ:",
            &data.container.cell_1,
            Message::UpdateCell1
        );
        let login_button = get_default_button(
            "Войти", Message::CheckAuth
        );

        let auth_page = get_default_column()
            .push(disclamer)
            .push(master_key_field)
            .push(login_button);

        Some(auth_page)
    } else {
        None
    }}
}
