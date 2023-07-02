(define (gcd a b)
  (if (eq b 0)
      a
      (gcd b (remainder a b))))
