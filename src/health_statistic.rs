#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_user() {
        let weight = 155.2_f32;
        let age = 32_u32;
        let mut user = User::new(String::from("Bob"), age, weight);
        assert_eq!(weight, user.weight());
        assert_eq!(age, user.age());
        let new_age = 33;
        user.set_age(new_age);
        assert_eq!(user.age, new_age);
        let new_weight = 166_f32;
        user.set_weight(new_weight);
        assert_eq!(user.weight, new_weight);
    }
}

pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> User {
        User { name, age, weight }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn weight(&self) -> f32 {
        self.weight
    }

    pub fn set_age(&mut self, age: u32) {
        self.age = age;
    }

    pub fn set_weight(&mut self, weight: f32) {
        self.weight = weight
    }
}
