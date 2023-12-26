#[derive(Debug)]
pub struct Person {
  pub name: String,
  pub age: u8,
  pub likes: String,
}

pub struct PersonBuilder {
  name: String,
  age: u8,
  likes: String,
}

/*
* Note that the Builder functions return Self for fn chaining before
* calling build()
*/
impl PersonBuilder {
  pub fn new() -> Self {
    PersonBuilder {
      name: String::new(),
      age: 0,
      likes: String::new(),
    }
  }

  pub fn name(mut self, a_name: String) -> Self {
    self.name = a_name;
    self
  }

  pub fn age(mut self, age: u8) -> Self {
    self.age = age;
    self
  }

  pub fn likes(mut self, likes: String) -> Self {
    self.likes = likes;
    self
  }

  pub fn build(self) -> Result<Person, String> {
    // Validation of complex invarients between various
    // builder-params can go here
    if self.name == String::from("Homer") && self.age > 40 {
      return Err(String::from("Homer can't be that old"));
    }

    Ok(Person {
      name: self.name,
      age: self.age,
      likes: self.likes,
    })
  }
}

#[test]
fn test_builer_success() {
  let homer_result = PersonBuilder::new()
    .name("Homer".to_string())
    .likes("Pie".to_string())
    .age(30)
    .build();

  let _ = match homer_result {
    Ok(p) => p,
    Err(err) => panic!("Invalid person {}", err),
  };
}

#[test]
#[should_panic]
fn test_builer_err() {
  let homer_result = PersonBuilder::new()
    .name("Homer".to_string())
    .likes("Pie".to_string())
    .age(50)
    .build();
  match homer_result {
    Ok(p) => p,
    Err(err) => panic!("Invalid person {}", err),
  };
}
