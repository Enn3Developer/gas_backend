use serde_derive::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub enum PriceUnit {
    KG,
    PZ,
}

#[derive(Serialize, Deserialize)]
pub struct Food {
    id: u32,
    image: String,
    name: String,
    price: f32,
    // price_unit: PriceUnit,
    desc: String,
}

#[derive(Serialize, Deserialize)]
pub struct Foods {
    foods: Vec<Food>,
}

impl Food {
    pub fn new(
        id: u32,
        image: String,
        name: String,
        price: f32,
        // price_unit: PriceUnit,
        desc: String,
    ) -> Self {
        Self {
            id,
            image,
            name,
            price,
            // price_unit,
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
    if page.contains("$$FOODS$$") {
        if let Ok(food_data) = fs::read_to_string("data/food.toml") {
            let food_template = include_str!("../html/foods");
            let foods: Foods = toml::from_str(&food_data).unwrap();
            let mut content = vec![];
            for food in foods.foods {
                let f = food_template.replace("$$IMAGE$$", &food.image);
                let f = f.replace("$$NAME$$", &food.name);
                let f = f.replace("$$PRICE$$", &format!("{:.2}", food.price));
                content.push(f);
            }
            page.replace("$$FOODS$$", &content.join("\n"))
        } else {
            page
        }
    } else {
        page
    }
}
