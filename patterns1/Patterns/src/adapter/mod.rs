
pub trait Cat {
    fn roar(&self);
}

pub struct Dog {
    pub name: String,
}

impl Dog {
    fn bark(&self) {
        println!("{} barks", self.name);
    }
}

// Composition / Adapter 
// Compose a Dog instance and implement a Cat 
// trait so that calls are proxied to the wrapped dog 
pub struct DogToCatAdapter { 
    pub dog: Dog, // concrete type
}
impl Cat for DogToCatAdapter {
    fn roar(&self) {
       self.dog.bark(); 
    }
}


#[test]
fn test_adapter() {
    let adog = Dog { name: "Poppy".to_string() };
    let adapted_dog = DogToCatAdapter { dog: adog }; 
    let cats = vec![adapted_dog];
    cats[0].roar();
}
