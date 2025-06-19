use crate::cli::Cli;
use clap::Parser;
use rand::prelude::*;
use std::fmt::{Display, Formatter, Write};
use std::fs::File;
use std::io::Write as IoWrite;

pub mod cli;

fn main() {
    let args = Cli::parse();

    let mut board = Board::new((args.width, args.height), args.num_predators, args.num_prey);

    let mut data = Vec::with_capacity(args.num_steps);
    for _ in 0..args.num_steps {
        board.step();

        let point = [board.num_predators(), board.num_prey(), board.num_empty()];
        data.push(point);
    }

    let mut data_file = File::create("data.json").unwrap();
    let json = serde_json::to_string(&data).unwrap();
    data_file.write_all(json.as_bytes()).unwrap();
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Board {
    size: (usize, usize),
    fields: Vec<Option<Field>>,
    on_turn: Field,
    num_predators: usize,
    num_prey: usize,
}

impl Board {
    fn new(size: (usize, usize), num_predators: usize, num_prey: usize) -> Board {
        assert!(size.0 > 0);
        assert!(size.1 > 0);
        assert!(size.0 * size.1 >= num_predators + num_prey);

        let mut fields = Vec::with_capacity(size.0 * size.1);
        for _ in 0..num_predators {
            fields.push(Some(Field::Predator));
        }
        for _ in 0..num_prey {
            fields.push(Some(Field::Prey));
        }
        for _ in 0..(size.0 * size.1 - num_predators - num_prey) {
            fields.push(None);
        }
        fields.shuffle(&mut rand::rng());

        Board {
            size,
            fields,
            on_turn: Field::Predator,
            num_predators,
            num_prey,
        }
    }

    fn get_field(&self, position: (usize, usize)) -> &Option<Field> {
        let index = position.1 * self.size.0 + position.0;
        assert!(index < self.size.0 * self.size.1);

        &self.fields[index]
    }

    fn set_field(&mut self, position: (usize, usize), value: Option<Field>) {
        let index = position.1 * self.size.0 + position.0;
        assert!(index < self.size.0 * self.size.1);

        let old = self.fields[index];
        match old {
            Some(Field::Prey) => self.num_prey -= 1,
            Some(Field::Predator) => self.num_predators -= 1,
            None => {}
        }

        match value {
            Some(Field::Prey) => self.num_prey += 1,
            Some(Field::Predator) => self.num_predators += 1,
            None => {}
        }

        self.fields[index] = value;
    }

    fn set_index(&mut self, index: usize, value: Option<Field>) {
        let old = self.fields[index];
        match old {
            Some(Field::Prey) => self.num_prey -= 1,
            Some(Field::Predator) => self.num_predators -= 1,
            None => {}
        }

        match value {
            Some(Field::Prey) => self.num_prey += 1,
            Some(Field::Predator) => self.num_predators += 1,
            None => {}
        }

        self.fields[index] = value;
    }

    fn step(&mut self) {
        let rand_pos = (
            rand::random_range(0..self.size.0),
            rand::random_range(0..self.size.1),
        );

        match self.on_turn {
            Field::Predator => {
                match self.get_field(rand_pos) {
                    Some(field) if *field == Field::Prey => {
                        // prey found which means turn this field into a predator
                        self.set_field(rand_pos, Some(Field::Predator));
                    }
                    _ => {
                        // no prey found which means kill one predator
                        for i in 0..self.fields.len() {
                            let field = self.fields[i];
                            if matches!(field, Some(Field::Predator)) {
                                self.set_index(i, None);
                                break;
                            }
                        }
                    }
                }
            }
            Field::Prey => {
                match self.get_field(rand_pos) {
                    Some(field) if *field == Field::Prey => {
                        // field is prey which means check adjacent fields for empty fields
                        let up = if rand_pos.1 > 0 {
                            Some((rand_pos.0, rand_pos.1 - 1))
                        } else {
                            None
                        };
                        let down = if rand_pos.1 < self.size.1 - 1 {
                            Some((rand_pos.0, rand_pos.1 + 1))
                        } else {
                            None
                        };
                        let left = if rand_pos.0 > 0 {
                            Some((rand_pos.0 - 1, rand_pos.1))
                        } else {
                            None
                        };
                        let right = if rand_pos.0 < self.size.0 - 1 {
                            Some((rand_pos.0 + 1, rand_pos.1))
                        } else {
                            None
                        };

                        for pos in [up, down, left, right].into_iter().flatten() {
                            if self.get_field(pos).is_none() {
                                // field is empty which means populate it
                                self.set_field(pos, Some(Field::Prey));
                                break;
                            }
                        }
                    }
                    None => {
                        // the empty field which means populate it
                        self.set_field(rand_pos, Some(Field::Prey));
                    }
                    _ => {}
                }
            }
        }

        self.on_turn = match self.on_turn {
            Field::Predator => Field::Prey,
            Field::Prey => Field::Predator,
        }
    }

    #[inline]
    fn num_predators(&self) -> usize {
        self.num_predators
    }

    #[inline]
    fn num_prey(&self) -> usize {
        self.num_prey
    }

    #[inline]
    fn num_empty(&self) -> usize {
        self.size.0 * self.size.1 - self.num_predators() - self.num_prey()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = String::with_capacity(self.size.0 * self.size.1 * 2);

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                match self.get_field((x, y)) {
                    Some(field) => out.push_str(&field.to_string()),
                    None => out.push('.'),
                }
                out.push(' ');
            }
            out.push('\n');
        }

        f.write_str(&out)
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Field {
    Predator,
    Prey,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Predator => f.write_char('R'),
            Field::Prey => f.write_char('B'),
        }
    }
}
