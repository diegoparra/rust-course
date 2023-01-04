# User Input Lesson

## 
 - Create a function `get_input` that returns io::Result<String>.
  - This function needs to get the user input and store it into a "buffer".
  - Return it removing all the blank spaces using the `trim` method as type `String`.


## On Main function
  - Create a empty vec to store all the inputs
  - Create a counter variable to stop the loop when achieve the desired number.
  - Start a `while` loop until the counter hits number 2
  - Create a `match` from the `get_input` function
  - Treat the Ok() pushing the `word` into the vec we've created before and increase the counter variable
  - Treat the Err() just printing it out.

  - Create a `for` loop over the `vec` we've created before
  - Print it out the original input and another one transforming it into uppercase
