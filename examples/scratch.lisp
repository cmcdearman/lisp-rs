;;;; A file for feature/syntax ideas

;;; ==================================================================
;;; *                         Special Forms                          *
;;; ==================================================================

;; `def`
;; `let`
;; `if`
;; `lambda`
;; `quote`
;; `quasiquote`
;; `unquote`
;; `unquote-splice`
;; `macro`

;; You can use `def` to bind values to names. Note
;; that we don't call these variables because they
;; can't vary. They are immutable.
(def x 10)

;; `def` can bind to lambdas
(def gcd (lambda (a b)
  (if (= b 0) 
      a
      (gcd b (mod a b)))))

;; `def` can also bind application forms
(def (gcd a b)
  (if (= b 0) 
      a
      (gcd b (mod a b))))

(def (map f xs)
  (if (empty? xs) nil
      (pair (f (head xs)) (map f (tail xs)))))

(def (fib n)
  (if (<= n 1) 
      n
      (+ (fib (- n 1)) (fib (- n 2)))))

(display (fib 45))

;; `let` can bind multiple names at once
(let ((a 10)
      (b 5))
  (+ a b))

;; `let` can also bind functions
(let ((gcd a b)
  (if (= b 0) 
      a
      (gcd b (mod a b))))
  (gcd 10 5))

;;; ==================================================================
;;; *                            Macros                              *
;;; ==================================================================

;; `macro` is used to define macros. Macros are
;; like functions, but they are evaluated at compile
;; time. They can be used to define new syntax.

;; `begin`
(macro (begin . body)
  (if (null? body)
      nil
      (if (null? (tail body))
          (head body)
          `(def ((result ,(head body)))
             (if (null? result)
                 (begin . ,(tail body))
                 result
                 (begin . ,body))))))

;; `loop` is a macro that expands into a `def` that
;; binds a name to a lambda that calls itself.
(macro (loop ())
  '(def (loop . x)
     (begin . ,body)
     (loop)))

;; `for` is a macro that expands into a `loop` that
;; binds a name to a range of numbers.
(macro (for i from to . body)
  `(loop (def i from)
     (if (<= i to)
         (begin . ,body)
         (inc! i))))

(macro (for-each x in . body)
  `(loop (def x in)
     (if (not (empty? x))
         (begin . ,body)
         (for-each . ,body))))

(macro (backwards . body)
  (pair 'begin
	(reverse body)))

(macro (cond . clauses)
  (if (null? clauses)
      nil
      (def (clause (head clauses))
  `(if ,(head clause)
       (begin . ,(tail clause))
       (cond . ,(tail clauses))))))

(macro (while condition . body)
  `(def loop ()
     (cond (,condition
	    (begin . ,body)
	    (loop)))))

(macro (when test . expr)
  (list 'if test (pair 'progn expr)))

;; Macro calls are like function calls, but the arguments
;; are not evaluated. Instead, they are passed to the macro
;; as unevaluated forms.

;; you can use `match` built-in macro to pattern match or `if` for conditionals
(def (gcd a b)
  (match b
    ((0) a)
    ((_) (gcd b (mod a b)))))

(def (fib n)
  (match n
    ((0) 0)
    ((1) 1)
    ((_) (+ (fib (- n 1)) (fib (- n 2))))))

;; Here's a match with guards
(def (fib n)
  (match n
    ((guard _) (<= n 1) n)
    ((_) (+ (fib (- n 1)) (fib (- n 2)))))) 

(def (ack m n)
  (cond ((= m 0) (+ n 1))
        ((= n 0) (ack (- m 1) 1))
        (ack (- m 1) (ack m (- n 1)))))

(when (= 1 1)
  (println "1 is equal to 1"))

;; `begin` is a special form that evaluates each of its
;; arguments in order and returns the value of the last
(begin (println "Hello, World!")
       (println "Goodbye, World!")
       10)

;;; ==================================================================
;;; *                         Data Types                             *
;;; ==================================================================

;; Records with `data` are immutable product types with named fields.
(data (point x y))

;; Instantiating a record is just like calling a function.
(def p (point 1 2)) 

(class Eq ()
  (def (eq self other)
    (raise :NotImplementedError)))

(class Ord (Eq PartialOrd)
  (def (cmp self other)
    (raise :NotImplementedError)))

(class Stack ()
  (def (push self x)
    (raise :NotImplementedError))
  (def (pop self)
    (raise :NotImplementedError))
  (def (peek self)
    (raise :NotImplementedError))
  (def (empty? self)
    (raise :NotImplementedError)))

;; instantiate a class
(def s (Stack))

;; method calls are done with `send`
(send s push 1)