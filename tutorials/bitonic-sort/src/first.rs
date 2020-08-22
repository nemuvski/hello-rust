pub fn sort(x: &mut [u32], up: bool) {
  if x.len() > 1 {
    let mid_point = x.len() / 2;
    sort(&mut x[..mid_point], true);
    sort(&mut x[mid_point..], false);
    sub_sort(x, up);
  }
}

fn sub_sort(x: &mut [u32], up: bool) {
  if x.len() > 1 {
    compare_and_swap(x, up);
    let mid_point = x.len() / 2;
    sub_sort(&mut x[..mid_point], up);
    sub_sort(&mut x[mid_point..], up);
  }
}

fn compare_and_swap(x: &mut [u32], up: bool) {
  let mid_point = x.len() / 2;
  for i in 0..mid_point {
    if (x[i] > x[mid_point + i]) == up {
      x.swap(i, mid_point + i);
    }
  }
}

// cargo testを実行したときのみコンパイルされる.
#[cfg(test)]
mod tests {
  use super::sort;

  // #[test]のついた関数はcargo testコマンドを実行した時に実行される.
  #[test]
  fn sort_u32_asc() {
    let mut x = vec![10, 30, 20, 5, 100, 35, 50, 88];
    sort(&mut x, true);
    assert_eq!(x, vec![5, 10, 20, 30, 35, 50, 88, 100]);
  }
  #[test]
  fn sort_u32_desc() {
    let mut x = vec![10, 30, 20, 5, 100, 35, 50, 88];
    sort(&mut x, false);
    assert_eq!(x, vec![100, 88, 50, 35, 30, 20, 10, 5]);
  }
}

