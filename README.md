# Guess The Number Terminal Game

### Guess the number game implemented in Rust.


I am still learning Rust thus I am using this project to explore several programming concepts in a new language.

In this project dependencies used
-  `rand::Rng` generates a random number which is the number to be guessed by the player.
- `std :: io :: BufRead` enables reading from the standard input.

---

#### Code explanation

In the lines



```
let parsed = line.ok().as_deref().map(str::parse::<i64>);
if let Some(Ok(guess)) = parsed {
	....
}

```


The code uses conditional pattern-matching to ignore the lines that may cause an error



The first line creates a `Result<i64>, ...>` object because it might fail at the reading or parsing steps.

The next line only matches on `Some(Ok(guess))` whenever a line results in a value that does not match, it skips the if statement, it is a great way to ignore errors.


This games continues looping as long as the value in the guess is not equal the random number assigned to it.




##### Source

[Learn Rust by writing a simple game by Moshe Zadka](https://opensource.com/article/20/12/learn-rust)


### Author

[Personal Website](https://yaseribrahim3808.ml/)

----
#### Contact me

- [Github - @yaseribrahim](https://www.github.com/yessur3808)
- [Twitter - @CurlyCoffee3808](https://twitter.com/curlycoffee3808)
- [LinkedIn - Yaser Ibrahim](https://www.linkedin.com/in/yaser-ibrahim-57963884)
- [Email Me @ yaser3808@gmail.com](mailto:yaser3808@gmail.com)



