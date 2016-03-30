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



#[cfg(test)]
mod test {
  use super::map;
  use third::List;

  fn add_one(&n: &i32) -> i32 {
      n + 1
  }

  fn to_s(&n: &i32) -> String {
      n.to_string()
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
}
