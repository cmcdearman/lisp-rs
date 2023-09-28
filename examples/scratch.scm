;;;; A file for feature/syntax ideas

;; You can use `let` to bind values to names. Note
;; that we don't call these variables because they
;; can't vary. They are immutable.
(let x 10)

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

;; you can use `match` to pattern match or `if` for conditionals
(let (gcd a b)
  (match b
    ((0) a)
    ((_) (gcd b (% a b)))))

(let (fib n)
  (match n
    ((0) 0)
    ((1) 1)
    ((_) (+ (fib (- n 1)) (fib (- n 2))))))

(let ack (m n)
  (cond ((= m 0) (+ n 1))
        ((= n 0) (ack (- m 1) 1))
        (ack (- m 1) (ack m (- n 1)))))

(let (map f xs)
  (if (empty? xs) nil
      (cons (f (car xs)) (map f (cdr xs)))))

(let (fib n)
  (if (<= n 1) 
      n
      (+ (fib (- n 1)) (fib (- n 2)))))

(println (fib 45))

;; Macros
(lets ((a 10) 
      (b 5))
  (+ a b))

;; `struct` is a macro that defines a struct.
;; Under the hood, it's just a map. 
(struct point (x y))

(let (gcd a b) 
  (if (= b 0) a
      (gcd b (% a b)))
  (gcd 24 18))
  
;; class Ord <: Eq + PartialOrd = 
;;   let cmp self other = raise :NotImplementedError
;; end

(class Eq ()
  (define (eq self other)
    (raise :NotImplementedError)))

(class Ord (Eq PartialOrd)
  (define (cmp self other)
    (raise :NotImplementedError)))