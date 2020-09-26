# Camel Up Oracle
The game [Camel up][camel-up] is a game of chance. This project provides you with an oracle to answer questions.

## Camel Up
Camel Up is a

>  board game for two to eight players. It was designed by Steffen Bogen and illustrated by Dennis Lohausen, and published in 2014 by Pegasus Spiele. Players place bets on a camel race in the desert; the player who wins the most money is the winner of the game.

It is a lovely game that is well balanced between players of all ages.

## Examples
### Yellow in the lead, red in pursuit
![Yellow in a considerable lead](http://fifth-postulate.nl/camel-up-oracle/1.jpg)

Look at the above situation. Yellow has taken a considerable lead in the race, with red straining to catch up. Both the red and the yellow dice are in play. Thinking long and hard one can come to the following conclusion.

For the next throw one of six things can happen. 

1. Red comes up one
2. Red comes up two
3. Red comes up three
4. Yellow comes up one
5. Yellow comes up two
6. Yellow comes up three

Only in one situation will Red win, i.e. when red comes up three. Whatever happens next the red camel will be on top of the yellow one and red will be victorious.

Even in this race, the reasoning is complex. Let's use this crate to verify our reasoning.

first announce the `camel_up` crate to your dependencies.

```yaml
camel_up = *
```

Next announce that we would like to use it.


```rust
external crate camel_up;
```

We will make use of the prelude to import a few useful names.

```rust
use camel_up::prelude::*;
```

In our `main` function we will recreate the race. The `Race` struct implements the [`FromStr`][fromstr] trait. This allows one to parse a string that describes a race into a `Race`. Every camel is designated by the first letter of their color. Positions are marked via a comma `,`. So our race is described by `"r,,,y"`. Use that description in making a race.

```rust
let race = "r,,,y".parse::<Race>().expect("to parse");
```

Similarly, The remaining dice can be created as well.

```rust
let dice = "ry".parse::<Dice>().expect("to parse");
```

With a `Race` and a set of `Dice` we can project how the race will be run.

```rust
let result = project(&race, &dice);
```

It returns a `Chances` mapping camels to their chance of winning. We can turn it into a `Vec`.

```rust
let mut ordered: Vec<(Camel, Fraction)> =
    result.winner.values().map(|(k, v)| (*k, *v)).collect();
```

which can be sorted by decreasing chance.

```rust
ordered.sort_by(|(_, left), (_, right)| right.cmp(&left));
```

We can use that to output those chances, winner will be in the front.

```rust
for (camel, fraction) in ordered {
    print!("({:?},{})", camel, fraction);
}
println!()
```

Running this will output

```plain
(Yellow,5/6)(Red,1/6)
```

Which supports our earlier hard work.

### Tower
The red camel is the underdog in the above situation. Can we change the odds? The red camel could ask their friends to help. By building a unit of camels the chances might change in favor of the red camel.

Changing the above [example][example] one can verify that Red needs all there friends to have a better chance of winning and beating Yellow.

### General
Questions like the ones above can be answered in general. The main executable can be fed a description of a race and set of remaining dice and project who will be in the lead.

```plain
cargo run -- --race="gr,,y" --dice="gry"
```

[camel-up]: https://en.wikipedia.org/wiki/Camel_Up
[fromstr]: https://doc.rust-lang.org/std/str/trait.FromStr.html
[example]: https://github.com/fifth-postulate/camel-up-oracle/blob/master/examples/tower.rs 