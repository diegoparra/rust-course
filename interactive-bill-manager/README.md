# Interactive bill manager project


## User Stories:

- L1 - I want to add bills, including the name and amount owed
- L1 - I want to view existing bills


  # Instructions:
    - We need a struct to store the bill 
    - We need a struct to store a Vec of bill
    - We need to implement a function to create a new vector of bills
    - We need to implement a function to add a new bill
    - We need to implement a function to get_all bills 
    - We need to create a function to get_input and it returns a `String`
      - We should stay run this part using while from the io::stdin()
      - use is_err() and return a error message
      - if everything goes well return the buffer and trim it
    - Create a main_menu function
      - inside this function create another show function printing all the menu options
      - create a new mutable empty bills 
      - create a `loop` and start the show() function
      - collect the user input
      - run a match over the user input
        - opt 1 add a new bill
        - opt 2 view bill menu
        - all the others `break`
    - Create a add_bill_menu function, it should receive e `&mut Bills`
      - get the bill name calling the function get_input
      - create a new function outside of this one, called get_bill_amount
        - create a loop and call the get_input
        - convert the input from `String` to `Result<f64, _>`, you can use the .parse() function
        - match the parsed_input
          Ok() => return the amount
          Err(_) => print enter a number
      - create a new bill object of type `Bill` and assing the values that was created from the bill name and amount
      - call the add method from the bills passing the value created for the bill
      - Print that the bill was created
    - Create a view_bill_menu function, it should receive a borrowed Bills
      - iterate over bills using the for loop and print the bill
