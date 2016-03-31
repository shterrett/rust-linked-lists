use third::List;

pub fn map<A, B>(f: fn(&A) -> B, ref from: &List<A>) -> List<B> {
    match from.head() {
        Some(val) => {
            map(f, &from.tail()).append(f(&val))
        }
        None => {
            List::new()
        }
    }
}

pub fn reduce<A, B>(f: fn(B, &A) -> B, initial: B, ref from: &List<A>) -> B {
    let mut iter = from.iter();
    let mut accum = initial;
    while let Some(next) = iter.next() {
        accum = f(accum, &next)
    }
    accum
}

#[cfg(test)]
mod test {
  use super::{map, reduce};
  use third::List;

  fn add_one(&n: &i32) -> i32 {
      n + 1
  }

  fn to_s(&n: &i32) -> String {
      n.to_string()
  }

  fn sum(accum: i32, next: &i32) -> i32 {
      accum + next
  }

  fn reverse(accum: List<i32>, next: &i32) -> List<i32> {
      accum.append(*next)
  }

  #[test]
  fn add() {
      let list = List::new().append(1).append(2).append(3).append(4);

      let new_list = map(add_one, &list);

      let mut new_iter = new_list.iter();
      assert_eq!(new_iter.next(), Some(&5));
      assert_eq!(new_iter.next(), Some(&4));
      assert_eq!(new_iter.next(), Some(&3));
      assert_eq!(new_iter.next(), Some(&2));
      assert_eq!(new_iter.next(), None);

      let mut old_iter = list.iter();
      assert_eq!(old_iter.next(), Some(&4));
      assert_eq!(old_iter.next(), Some(&3));
      assert_eq!(old_iter.next(), Some(&2));
      assert_eq!(old_iter.next(), Some(&1));
      assert_eq!(old_iter.next(), None);
  }

  #[test]
  fn convert_to_string() {
      let list = List::new().append(1).append(2).append(3).append(4);

      let new_list = map(to_s, &list);

      let mut new_iter = new_list.iter();
      assert_eq!(new_iter.next(), Some(&"4".to_string()));
      assert_eq!(new_iter.next(), Some(&"3".to_string()));
      assert_eq!(new_iter.next(), Some(&"2".to_string()));
      assert_eq!(new_iter.next(), Some(&"1".to_string()));
      assert_eq!(new_iter.next(), None);

      let mut old_iter = list.iter();
      assert_eq!(old_iter.next(), Some(&4));
      assert_eq!(old_iter.next(), Some(&3));
      assert_eq!(old_iter.next(), Some(&2));
      assert_eq!(old_iter.next(), Some(&1));
      assert_eq!(old_iter.next(), None);
  }

  #[test]
  fn sum_list() {
      let list = List::new().append(1).append(2).append(3).append(4);

      let total = reduce(sum, 0, &list);
      assert_eq!(total, 10);

      let mut iter = list.iter();
      assert_eq!(iter.next(), Some(&4));
      assert_eq!(iter.next(), Some(&3));
      assert_eq!(iter.next(), Some(&2));
      assert_eq!(iter.next(), Some(&1));
      assert_eq!(iter.next(), None);
  }

  #[test]
  fn reverse_list() {
      let list = List::new().append(1).append(2).append(3).append(4);

      let doubled = reduce(reverse, List::new(), &list);

      let mut doubled_iter = doubled.iter();
      assert_eq!(doubled_iter.next(), Some(&1));
      assert_eq!(doubled_iter.next(), Some(&2));
      assert_eq!(doubled_iter.next(), Some(&3));
      assert_eq!(doubled_iter.next(), Some(&4));
      assert_eq!(doubled_iter.next(), None);

      let mut iter = list.iter();
      assert_eq!(iter.next(), Some(&4));
      assert_eq!(iter.next(), Some(&3));
      assert_eq!(iter.next(), Some(&2));
      assert_eq!(iter.next(), Some(&1));
      assert_eq!(iter.next(), None);
  }
}
