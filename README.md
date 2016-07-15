# poker
Rust poker hand ranker

A program that generates two hands of 5 cards and compares them, determining which of the two is the better hand.
Written in Rust to learn and play with the language.
Inspired by a similar program written by Donne Martin (https://github.com/donnemartin/poker).

###### TODO:
* Implement code check for low ace straight
* In the case that both hands have high card, use the highest card between the hands to determine the winner
* Use borrowing for the following:
  * in main.rs when calling draw_hand method
  * in hand_ranker.rs for sort_cards_desc
