fn swapnum(a:i32,b:i32)->(i32, i32){
    (b,a)
}

fn main() {
  
    let x = 5;
    let y = 10;
    println!("Before swapping: x = {}, y = {}", x, y);
    let (x, y) = swapnum(x, y);
    println!("After swapping: x = {}, y = {}", x, y);
   }
  
   
   #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swapnum() {
        // Arrange
        let x = 5;
        let y = 10;

        
        let (result_x, result_y) = swapnum(x, y);

        // Assert
        assert_eq!(result_x, 10); // After swapping, x should be equal to the original value of y
        assert_eq!(result_y, 5);  // After swapping, y should be equal to the original value of x
    }
}