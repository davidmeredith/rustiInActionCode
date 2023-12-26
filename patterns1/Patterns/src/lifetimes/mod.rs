
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


#[test]
fn test_lifetimes_ok() {
  let string1 = String::from("long string is long");
  {
      let string2 = String::from("xyz");
      let result = longest(string1.as_str(), string2.as_str());
      println!("The longest string is {}", result);
  }
}

#[test]
fn test_does_not_compile(){
  // let string1 = String::from("long string is long");
  // let result;
  // {
  //     let string2 = String::from("xyz");
  //     // result will have lifetime of the shortest fn arg i.e., string2
  //     result = longest(string1.as_str(), string2.as_str()); // string2 does not live long enough
  //
  // } // string2 is dropped here, so the 'result' reference 
  //   // would be left dangling 
  //
  // println!("The longest string is {}", result);
}
