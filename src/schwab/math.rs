/* Math to support the investment calculations.
 * lambda = The amount of new money to invest. 
 * tau = The current value of the total investments.
 * x_1 = target investment goal in percent of a given collection. 1 >= x_1 >= 0
 * x_1_o = current investment value in percent of a given collection 1 >= x_1_o >= 0
 * x_1 + x_2 + ... x_n = 1 where all values are 1 >= and >= 0. 
 * x_1_o + x_2_o + ... x_n_o = 1 where all values are 1 >= and >= 0.
 * theta = lambda + tau
 * a_1 = x_1 * theta - x_1_o * tau | This value a_1 is the amount we are going to invest for a given
 * collection. If a_1 >= 0 then that collection will not be invested with that new money. If a_1 >=
 * lambda then the collection that has the biggest an will recieve the entire amount of new money
 * to be invested. 
 * a_1 + a_2 + ... + a_n = lambda
 * (a_1 + t * x_1_o) + (a_2 + t * x_2_o) + ... + (a_n + t * x_n_o) = theta
 **/

// Unit test the hell out of this stuff
