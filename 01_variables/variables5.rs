fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // shadowing
    // let 可以重新定義變數
    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}
