(def gcd
  (fn (a b)
    (if (= b 0) 
      a 
      (gcd b (% a b)))))
    
(gcd 51 85)

(def gcd 
  (fn (a b) 
    (match b
      ((0 a) a)
      (_ (gcd b (% a b))))))