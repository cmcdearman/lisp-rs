(def x 10)

(def gcd (lambda (a b)
  (if (= b 0) 
      a
      (gcd b (mod a b)))))

(def (gcd a b)
  (if (= b 0) 
      a
      (gcd b (mod a b))))

(def (map f xs)
  (if (empty? xs) nil
      (pair (f (head xs)) (map f (tail xs)))))

(let ((a 10)
      (b 5))
  (+ a b))

(if (= 1 2) 
    (print "1 is equal to 2")
    (print "1 is not equal to 2"))