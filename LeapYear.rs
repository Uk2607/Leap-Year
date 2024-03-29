//Leap-Year
//Condition for a year to be a leap year.
fn is_leap(year: i32) -> bool {
    let factor = |x| year % x == 0;
    factor(4) && (!factor(100) || factor(400))
}
