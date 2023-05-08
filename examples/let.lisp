(let ([x 5]
      [y 10])
      (list x y))

(let gcd (a b)
  (if (= b 0)
      a
      (gcd b (% a b))))

(let x 1)