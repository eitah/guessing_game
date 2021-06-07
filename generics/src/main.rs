//  The prompt says: Another way we could implement largest is
// for the function to return a reference to a T value in the slice.
// If we change the return type to &T instead of T, thereby changing
//  the body of the function to return a reference, we wouldn’t need
// the Clone or Copy trait bounds and we could avoid heap allocations.
// So I cleared the Copy here and tried everything I could think of to return
// a reference to &T but nothing even lets me compile the code.
// What am i doing wrong?
fn largest<T: PartialOrd>(list: &[T]) -> &T {
  let mut largest = list[0];

  for &item in list {
      if item > largest {
          largest = item;
      }
  }

  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest(&char_list);
  println!("The largest char is {}", result);
}
