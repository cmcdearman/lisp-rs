(def gcd 
  (fn (a b) 
    (if (b == 0) 
      (a) 
      (gcd b (mod a b)))))

(def gcd (fn (a b) (if (b == 0) (a) (gcd b (mod a b)))))