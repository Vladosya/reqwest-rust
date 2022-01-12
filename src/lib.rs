use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Geo {
    lat: String,
    lng: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
struct Company {
    name: String,
    catchPhrase: String,
    bs: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    id: i32,
    name: String,
    username: String,
    email: String,
    phone: String,
    website: String,
    address: Address,
    company: Company,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Users(Vec<User>);

pub struct UserRepo {
    base_url: String,
    client: reqwest::Client,
}

impl Default for UserRepo {
    fn default() -> Self {
        Self::new("https://jsonplaceholder.typicode.com".to_string())
    }
}

impl UserRepo {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }
    pub async fn get_user(&self, need_todo: String) -> anyhow::Result<User> {
        let url: String = format!("{}/users/{}", self.base_url, need_todo);
        let todo: User = self.client.get(&url).send().await?.json().await?;

        Ok(todo)
    }
    pub async fn get_users(&self) -> anyhow::Result<Users> {
        let url: String = format!("{}/users", self.base_url);
        let res: Users = self.client.get(&url).send().await?.json().await?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn it_works_get_user() {
        let repo: UserRepo = UserRepo::default();
        let user: User = match repo.get_user("1".to_string()).await {
            Ok(res) => res,
            Err(err) => panic!("Error {}", err),
        };

        assert!(!user.name.is_empty());
        assert!(!user.username.is_empty());
        assert!(!user.company.name.is_empty());
        assert!(!user.address.street.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn it_works_get_users() {
        let repo: UserRepo = UserRepo::default();
        let users: Users = match repo.get_users().await {
            Ok(res) => res,
            Err(err) => panic!("Error {}", err),
        };

        assert!(!users.0[0].name.is_empty());
        assert!(!users.0[0].username.is_empty());
        assert!(!users.0[0].company.name.is_empty());
        assert!(!users.0[0].address.street.is_empty());
    }
}
