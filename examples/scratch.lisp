;; let binding
;; bindings are immutable by default
(let a 1)

;; mutable binding
(let! a 1)

;; if expression
;; Every if expression must have a then and an else clause.
;; The types of the then and else clause must be the same.
(if (= 1 2) 
    (println "1 is equal to 2")
    (println "1 is not equal to 2"))

;; let binding to function
(let (fib n)
  (let (loop n a b)
    (if (= n 0)
      a
      (loop (- n 1) b (+ a b)))
    (loop n 0 1)))

;; let binding to function with expression body
(let (gcd a b)
  (if (= b 0) 
      a
      (gcd b (mod a b)))
  (gcd 10 5))

;; type hints
(let (gcd (a : Int) (b : Int) : Int)
  (if (= b 0) 
      a
      (gcd b (mod a b)))
  (gcd 10 5))

(let (map f xs)
  (if (empty? xs) ()
      (pair (f (head xs)) (map f (tail xs)))))

(let (fact n)
  (if (= n 0)
      1
      (* n (fact (- n 1)))))

(let (ack m n)
  (cond ((= m 0) (+ n 1))
        ((= n 0) (ack (- m 1) 1))
        (t (ack (- m 1) (ack m (- n 1))))))

;; lists
'(1 2 3)
[1 2 3]

;; quasiquote/unquote
`(1 2 ,(+ 1 2))
[1 2 (+ 1 2)]

;; quasiquote/unquote-splicing
`(1 2 ,@(list 3 4))

;; vectors
#[1 2 3]

;; sets
#{ 1 2 3 }

;; maps
{ :a 1 :b 2 }

;; User defined types
;; product type
(type (Point 
  (x : Int) 
  (y : Int)))

;; product type with type parameters
(type (Pair T 
  (head : T) 
  (tail : (Pair T))))

;; sum type
(type (Shape
  (Circle (radius : Int))
  (Rectangle (width : Int) (height : Int))
  (Triangle (base : Int) (height : Int))))

;; sum type with type parameters
(type (Option T (Some T) (None)))

(type (Result T E 
  (Ok T) 
  (Err E)))

;; sum type with complex type parameters
(type (List T
  (Pair (head : T) (tail : (List T)))
  (Empty)))

;; modules
(module List
  (type List T
    (Pair (head : T) (tail : (List T)))
    (Empty))

  (let empty (Empty))

  (let (empty? xs)
    (match xs
      (Empty true)
      (Pair false)))

  (let (map f xs)
    (if (empty? xs) ()
        (pair (f (head xs)) (map f (tail xs)))))
  
  (let (foldl f acc xs) 
    (if (empty? xs)
        acc
        (foldl f (f acc (head xs)) (tail xs))))
  
  (let (foldr f acc xs)
    (if (empty? xs)
        acc
        (f (head xs) (foldr f acc (tail xs)))))

  (let (filter f xs)
    (if (empty? xs)
        ()
        (if (f (head xs))
            (pair (head xs) (filter f (tail xs)))
            (filter f (tail xs)))))

  (let (reverse xs)
    (foldl (lambda (acc x) (pair x acc)) () xs)))

;; module usage
(List.map (lambda (x) (* x x)) [1 2 3])

;; macros
(macro (cond clauses)
  (if (empty? clauses)
      ()
      (let (clause (head clauses))
        (if (= (head clause) 'else)
            (list 'begin (tail clause))
            (list 'if (head clause) (list 'begin (tail clause)) (cond (tail clauses)))))))

(macro (when test body)
  `(if ,test (begin ,@body) ()))

(macro (while test body)
  `(let (loop)
     (if ,test
         (begin ,@body (loop))
         ())))

;; example uses
(while (< i 10)
  (println i)
  (set! i (+ i 1)))

;; bindings are immutable by default
(def a 1)

;; mutable binding
(def! a 1)

;; if expression
;; Every if expression must have a then and an else clause.
;; The types of the then and else clause must be the same.
(if (= 1 2) 
    (println "1 is equal to 2")
    (println "1 is not equal to 2"))

;; lambda expression using `fn`
(fn (x) (+ x 1))

;; `defn` macro for defining functions
(defn fib (n)
  (let ((loop (fn (n a b)
          (if (= n 0)
              a
              (loop (- n 1) b (+ a b))))))
    (loop n 0 1)))

;; benchmark:
;; fib from 1 to 40
(defn main (args)
  (let (loop n)
    (if (= n 40)
      []
      (begin
        (println (fib n))
        (loop (+ n 1))))
    (loop 1)))

;; `defn` is a contraction for `def` and `fn`. The following are equivalent:
(defn f (x) (+ x 1))
(def f (fn (x) (+ x 1)))

;; `let` expression
(let ((a 1) 
      (b 2))
  (+ a b))

;; higher order function
(defn map (f xs)
  (if (empty? xs) []
      (pair (f (head xs)) (map f (tail xs)))))

;; pattern types
;; `Pair` is a pattern type that matches a pair of values
;; Here's what it looks like:
;; (Pair x xs)
;; `Range` is a pattern type that matches a range of values
;; Here's what it looks like:
;; (.. start end)

;; pattern matching
(defn map (f xs)
  (match xs
    ([] [])
    ((:: x xs) (pair (f x) (map f xs)))))

;; (: map (-> (-> a b) [a] [b]))
(defn map (f []) [])
(defn map (f (:: x xs)) (:: (f x) (map f xs)))

(defn fib (0) 0)
(defn fib (1) 1)
(defn fib (n) (+ (fib (- n 1)) (fib (- n 2))))

;; overload operator by using match guards
(defn == ((Pair a b) (Pair c d))
  (and (== a c) (== b d)))

(defn + (rational? a rational? b) 
  (let ((gcd (gcd (a.denom) (b.denom))))
    (Rational (+ (* a.numer (/ b.denom gcd)) 
                 (* b.numer (/ a.denom gcd))) 
                 (* a.denom (/ b.denom gcd)))))

;; type annotations
;; (: fib (-> Int Int))
(defn fib (n)
  (match n
    ((0) 0)
    ((1) 1)
    ((n) (+ (fib (- n 1)) (fib (- n 2))))))

(defn fact (n)
  (if (= n 0)
      1
      (* n (fact (- n 1)))))

;; lists
'(1 2 3)
[1 2 3]

;; quasiquote/unquote
`(1 2 ,(+ 1 2))
[1 2 (+ 1 2)]

;; quasiquote/unquote-splicing
`(1 2 ,@(list 3 4))

;; vectors
#[1 2 3]

;; sets
#{1 2 3}

;; records are just maps with symbols as keys
(type (Point 
       (x : Int) 
       (y : Int)))

;; record creation
(def p (Point 1 2))

;; record access
p.x

;; sum type
(type Shape
  (Circle radius)
  (Rect width height))

(type Bool True False)

;; generic sum type
(type (Option T)
  None
  (Some T))

;; A product type is just a sum type with only one constructor
(type (Pair a b)
  (Pair a b))

;; maps bound
(def person { 'name "John" 'age 30})
;; map access
(display person.name)
;; => "John"
(display person.age)
;; => 30

;; named-argument currying
(def sum (foldl (= f +) (= init 0)))

;; macros
(defmacro (cond & clauses)
  `(let (test (head clauses))
     (if (head test)
         (begin (tail test))
         (cond ,@(tail clauses)))))

(defmacro (when test &body)
  `(if ,test (begin ,@body) ()))

(defmacro (when test &body)
  `(if ,test (begin ,@body) ()))

(defmacro (while test & body)
  `(let (loop)
     (if ,test
         (begin ,@body (loop))
         ([]))))

;; example uses
(while (< i 10)
  (println i)
  (set! i (+ i 1)))

(def (ack m n)
  (cond ((= m 0) (+ n 1))
        ((= n 0) (ack (- m 1) 1))
        ('t (ack (- m 1) (ack m (- n 1))))))

;; modules
(module List
  (def (pair head tail) { 'head head 'tail tail})
  (def empty '())

  (def (empty? xs)
    (= xs empty))

  (def (push-front xs x)
    (pair x xs))

  (def (map f xs)
    (if (empty? xs) ()
        (pair (f (head xs)) (map f (tail xs)))))
  
  (def (foldl f acc xs) 
    (if (empty? xs)
        acc
        (foldl f (f acc (head xs)) (tail xs))))
  
  (def (foldr f acc xs)
    (if (empty? xs)
        acc
        (f (head xs) (foldr f acc (tail xs)))))

  (def (filter f xs)
    (if (empty? xs)
        ()
        (if (f (head xs))
            (pair (head xs) (filter f (tail xs)))
            (filter f (tail xs)))))

  (def (rev xs)
    (foldl (fn (acc x) (pair x acc)) () xs)))

;; module usage
(use List)
(List.map (fn (x) (* x x)) [1 2 3])