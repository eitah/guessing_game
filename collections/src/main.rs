fn main() {
  #[allow(unused_mut)]
  let mut v = vec![1, 2, 3, 4, 5];

  #[allow(unused_variables)]
  let first = &v[0];

  // adding a new element onto the end of the vector might require allocating
  // new memory and copying the old elements to the new space, if there isn’t
  // enough room to put all the elements next to each other where the vector
  // currently is. Therefore uncommenting this is not allowed.
  // v.push(6);

  // println!("The first element is: {}", first);

  let mut v = vec![100, 32, 57];
  for i in &mut v {
      *i += 50;
  }

    println!("vec  is: {:?}", v);

}
