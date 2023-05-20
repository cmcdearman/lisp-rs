(pub mod list
  (pub data List a = Nil | Cons a (List a))
  (pub data Cons)

  (pub fn length xs =
    (match xs with
      Nil -> 0
      Cons x xs' -> 1 + length xs'))

  (pub fn map f xs =
    (match xs with
      Nil -> Nil
      Cons x xs' -> Cons (f x) (map f xs')))) 