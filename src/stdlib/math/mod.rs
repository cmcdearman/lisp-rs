pub fn gcd(x: Integer, y: Integer) -> Integer {
//    x = abs(x);	// don't get confused by negative values
// 	y = abs(y);
// 	while (y) {
// 		int t = y;
// 		y = x%y;
// 		x = t;
// 	}
// 	return x; 
    x = x.abs()
}
// Euclid's algorithm