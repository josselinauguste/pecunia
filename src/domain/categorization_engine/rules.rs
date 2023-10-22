use super::Category;

pub struct Rule {
    pub pattern: String,
    pub category: Category,
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            pattern: "BISTRO MONTES".to_string(),
            category: Category::LunchExtra,
        },
        Rule {
            pattern: "THE CENTRAL PUB".to_string(),
            category: Category::LunchExtra,
        },
    ]
}
