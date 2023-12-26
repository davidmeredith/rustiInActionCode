pub trait Pizza {
  fn get_cost(&self) -> f64;
}

pub struct BasePizza {
  pub cost: f64,
}
impl Pizza for BasePizza {
  fn get_cost(&self) -> f64 {
    self.cost
  }
}

pub struct ToppingDecorator<'a> {
  // base_pizza references a Pizza trait, hence dyn
  // for the dynamic dispatch and the lifetime ref &'a
  pub base_pizza: &'a dyn Pizza,
  pub topping_cost: f64,
}

// Like with generics, lifetime names for struct FIELDS
// always need to be declared after the impl keyword
// & then after the structs name, because those lifetimes
// are part of the struct's type.
impl<'a> Pizza for ToppingDecorator<'a> {
  fn get_cost(&self) -> f64 {
    self.base_pizza.get_cost() + self.topping_cost
  }
}
// For non-trait impl methods, input references might be
// independent of the struct's fields, and given the lifetime
// elison rules, lifetime annotations often aren't necessary
// in these method signatures.
impl<'a> ToppingDecorator<'a> {
  fn get_cost_and_print(&self, printme: &str) -> f64 {
    println!("{}", printme);
    self.base_pizza.get_cost() + self.topping_cost
  }
}

pub struct ExtraCheeseDecorator<'a> {
  // base_pizza references a Pizza trait, hence dyn
  // for the dynamic dispatch and the lifetime ref &'a
  pub base_pizza: &'a dyn Pizza,
  pub extra_cheese_cost: f64,
}
// Like with generics, lifetime names for struct FIELDS
// always need to be declared after the impl keyword
// & then after the structs name, because those lifetimes
// are part of the struct's type.
impl<'a> Pizza for ExtraCheeseDecorator<'a> {
  fn get_cost(&self) -> f64 {
    self.base_pizza.get_cost() + self.extra_cheese_cost
  }
}

#[test]
fn test_pizza() {
  let pizza = BasePizza { cost: 5.0 };
  let topped_pizza = ToppingDecorator {
    base_pizza: &pizza,
    topping_cost: 2.0,
  };
  let total = topped_pizza.get_cost();
  assert_eq!(total, 7.0);
  topped_pizza.get_cost_and_print(&String::from("my message"));

  let pizza = BasePizza { cost: 5.0 };
  let extra_cheesy_pizza = ExtraCheeseDecorator {
    base_pizza: &pizza,
    extra_cheese_cost: 1.5,
  };
  let total = extra_cheesy_pizza.get_cost();
  assert_eq!(total, 6.5);
}
