/*
 * Lifetimes ensure that references are valid as long as we need them to be.
 * Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
 * The main aim of lifetimes is to prevent dangling references.
 *
 * Lifetimes are only relevant to references.
 * They are used to enforce the following constraint: the returned reference will be valid as long as
 * both the parameters are valid (In practice, it means that the lifetime of the ref returned by
 * a function is the same as the smaller of the args lifetimes passed to the function).
 * This is the relationship between lifetimes of the parameters and the return value.
 *
 *
 * If we changed the implementation of the longest function to always return the first parameter
 * rather than the longest string slice, we wouldn’t need to specify a lifetime on the y arg:
 *
 *    fn longest<'a>(x: &'a str, y: &str) -> &'a str {
 *      x
 *    }
 *
 */

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
fn test_does_not_compile() {
  // let string1 = String::from("long string is long");
  // let result;
  // {
  //     let string2 = String::from("xyz");
  //     // result will have lifetime of the shortest fn arg i.e., string2
  //     result = longest(string1.as_str(), string2.as_str()); // string2 does not live long enough
  //
  // } // string2 is dropped here, so 'result' ref would be dangling
  //
  // println!("The longest string is {}", result);
}
