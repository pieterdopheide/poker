# poker
Rust poker hand ranker

A program that generates two hands of 5 cards and compares them, determining which of the two is the better hand.
Written in Rust to learn and play with the language.
Inspired by a similar program written by Donne Martin (https://github.com/donnemartin/poker).

##### TODO:
* ~~Implement code check for low ace straight~~
* In the case that both hands have high card, use the highest card between the hands to determine the winner
* Use borrowing for the following:
  * ~~in main.rs for the draw_hand method~~
  * ~~in hand_ranker.rs for the sort_cards_desc method~~
  * ~~in hand_ranker.rs for the perpare method~~
* ~~Add unit tests~~
