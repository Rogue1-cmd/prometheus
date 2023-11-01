//The program prints the values of the Fibonacci numbers and finds 
// the sum of even valued numbers in the series.
fn main() {

    let mut f1 = 1;
    let mut f2 = 1;
    let mut sum = 0;

    loop {

        let num = f1 + f2;
        if num > 4000000{
            break;
        }
        if num % 2 == 0 {
            sum = sum + num;
        println! (" Num: {}", num);
        }

        f1 = f2;
        f2 = num; 
    }
    println! ("Sum of Even valued terms: {}", sum)
}

