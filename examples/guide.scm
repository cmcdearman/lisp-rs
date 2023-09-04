;; global variables
(let x 10)
;; global constants
(const x 10)

;; `let` can bind to lambdas
(let gcd (lambda (a b)
  (if (= b 0)
    a
    (gcd b (% a b)))))

;; `let` can also bind application forms
(let (gcd a b)
  (if (= b 0)
      a
      (gcd b (% a b))))

;; you can use `match` to pattern match
(let (gcd a b)
  (match b
    (0 a)
    (_ (gcd b (% a b)))))

(let (fib n)
  (match n
    (0 0)
    (1 1)
    (_ (+ (fib (- n 1)) (fib (- n 2))))))

;; or you can match to declare functions
(let (fib 0) 0)
(let (fib 1) 1)
(let (fib n) (+ (fib (- n 1)) (fib (- n 2))))

(let (fib n)
  (if ((<= n 1) n)
  (+ (fib (- n 1)) (fib (- n 2)))))

(println (fib 100000))

;; Macros
(lets ((a 10) 
      (b 5))
  (+ a b))

(struct point (x y))

