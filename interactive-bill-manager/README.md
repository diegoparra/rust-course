# Interactive bill manager project


## User Stories:

- L1 - I want to add bills, including the name and amount owed
- L1 - I want to view existing bills


  # Instructions:
    - We need a struct to store the bill 
    - We need a struct to store a Vec of bill
    - We need to implement a function to create a new vector of empty bills that returns `Self`
    - We need to implement a function to add a new bill
      - It needs to receive a mutable reference of self, and a bill of type `Bill`
    - We need to implement a function to get_all bills 
      - It needs to receive a borrowed reference of self
      - It needs to return a borrowed Vec of `Bill`
    - We need to create a function to get_input and it returns a `String`
      - Create an empty variable of type `String`
      - We should stay running this part using while from the io::stdin()
      - use is_err() and return a error message
      - if everything goes well return the buffer and trim it
    - Create a main_menu function
      - inside this function create another function called `show` printing all the menu options
      - create a new mutable empty bills 
      - create a `loop` and start the show() function
      - collect the user input using the function `get_input`
      - run a match over the user input
        - opt 1 call the function `add_bill_menu`
        - opt 2 call the function `view_bill_menu`
        - all the others `break`
    - Create a add_bill_menu function, the signature should create a variable bills of type: `&mut Bills`
      - get the bill name calling the function get_input
      - create a variable amount and assing to it the value of `get_bill_amount`
      - create a new bill variable of type `Bill` and assing the values that was created from the bill name and amount
      - call the add method from the bills passing the value created for the bill
      - Print that the bill was created
    - Create a new function called get_bill_amount that returns a `f64`
      - Print a message asking for the amount
      - start a loop
        - inside the loop create a variable `input` as type `String` inside loop and assing the value from the `get_input` function
        - create a variable parsed_input as type `Result<f64, _>` and assing the result from input.parse()
        - match over the parsed_input
          Ok(amount) => return the amount
          Err(_) => print the message: "enter a number"
    - Create a view_bill_menu function, the function signature should has `bills` of type `&Bills`
      - iterate over bills using the `for` loop and the method `.get_all()` and print the bill
