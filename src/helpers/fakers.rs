use std::collections::HashMap;

use fake::{Dummy, Fake, Faker};

pub struct InfoFaker;

impl Dummy<InfoFaker> for HashMap<String, serde_json::Value> {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &InfoFaker, _: &mut R) -> Self {
        HashMap::new()
    }
}

pub struct UuidFaker;

impl Dummy<UuidFaker> for uuid::Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UuidFaker, rng: &mut R) -> Self {
        uuid::Uuid::from_u128(Faker.fake_with_rng(rng))
    }
}

impl Dummy<Faker> for crate::idesscd::Base64 {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(faker: &Faker, _: &mut R) -> Self {
        crate::idesscd::Base64::from(faker.fake::<String>())
    }
}
