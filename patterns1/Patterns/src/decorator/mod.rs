
pub trait Pizza {
   fn get_cost(&self) -> f64;
}

pub struct BasePizza {
   pub cost: f64
}
impl Pizza for BasePizza {
  fn get_cost(&self) -> f64 {
      self.cost
  }
}

pub struct ToppingDecorator<'a> {
   // base_pizza type is a trait, hence parameterised type &'a
   pub base_pizza: &'a dyn Pizza, 
   pub topping_cost: f64
}
impl<'a> Pizza for ToppingDecorator<'a> {
   fn get_cost(&self) -> f64 {
      self.base_pizza.get_cost() + self.topping_cost
   }
}

pub struct ExtraCheeseDecorator<'a> {
   // base_pizza type is a trait, hence parameterised type &'a
   pub base_pizza: &'a dyn Pizza, 
   pub extra_cheese_cost: f64,
}
impl<'a> Pizza for ExtraCheeseDecorator<'a> {
    fn get_cost(&self) -> f64 {
       self.base_pizza.get_cost() + self.extra_cheese_cost 
    }
}

#[test]
fn test_pizza() {
   let pizza = BasePizza { cost: 5.0 };
   let topped_pizza = ToppingDecorator { base_pizza: &pizza, topping_cost: 2.0, };
   let total = topped_pizza.get_cost();
   assert_eq!(total, 7.0);

   let pizza = BasePizza { cost: 5.0 };
   let extra_cheesy_pizza = ExtraCheeseDecorator { base_pizza: &pizza, extra_cheese_cost: 1.5, };
   let total = extra_cheesy_pizza.get_cost();
   assert_eq!(total, 6.5);
}
