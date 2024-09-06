use uuid::Uuid;

#[derive(Clone)]
pub struct TokenData {
    token: String,
    user_id: Uuid,
    role: i16,
}

impl TokenData {
    pub fn new(token: String, user_id: Uuid, role: i16) -> Self {
        Self {
            token,
            user_id,
            role,
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn role(&self) -> i16 {
        self.role
    }
}
