;; global variables
(let a 10)
;; global constants
(const a 10)

;; `let` can bind to lambdas
(let gcd (lambda (a b)
  (if (= b 0)
    a
    (gcd b (% a b)))))

;; `fn` is a macro that expands to bind a lambda to a symbol
(fn gcd (a b)
  (if (= b 0)
      a
      (gcd b (% a b))))

(fn gcd (a b)
  (match b
    (0 a)
    (_ (gcd b (% a b)))))

(let ((a 10) 
      (b 5))
  (gcd a b))
