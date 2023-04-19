use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Food {
    image: String,
    name: String,
    price: f32,
    desc: String,
}

#[derive(Serialize, Deserialize)]
pub struct Foods {
    foods: Vec<Food>,
}

impl Food {
    pub fn new(image: String, name: String, price: f32, desc: String) -> Self {
        Self {
            image,
            name,
            price,
            desc,
        }
    }
}

impl Foods {
    pub fn new() -> Self {
        Self { foods: vec![] }
    }

    pub fn add(&mut self, food: Food) {
        self.foods.push(food);
    }
}

pub fn common(page: String) -> String {
    let page = page.replace("$$FOOTER$$", include_str!("../html/footer"));
    let page = page.replace("$$HEAD$$", include_str!("../html/head"));

    page
}
