(def gcd 
  (fn (a b) 
    (if (eq b 0) 
      (a) 
      (gcd b (mod a b)))))

(def gcd (fn (a b) (if (eq b 0) (a) (gcd b (mod a b)))))

(if (> 3 5) (+ 4 5) (- 3 6))

(def gtr (fn (a b) (if (> a b) (+ 4 5) (- 3 6))))