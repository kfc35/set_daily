use bevy::ecs::prelude::*;
use rand::{
    Rng, RngExt, SeedableRng,
    distr::{Distribution, StandardUniform},
};

extern crate alloc;
use alloc::vec::Vec;

/// Contains the current game state.
#[derive(Resource)]
pub struct GameState {
    /// The cards the user tries to make Sets out of.
    pub cards: [Card; 12],
    /// The current guess that the user is in the process of selecting.
    ///
    /// This vec must have max size 3. Once it has size 3, it is checked
    /// whether it is a valid set.
    pub current_guess: Vec<Entity>,
}

/// A card in a game of Set. Its contents can vary in four dimensions: [`Shape`],
/// [`Quantity`], [`Fill`], and [`Color`]. In a standard Set deck,
/// there is one of each unique card, for a total of 3^4 = 81 cards.
#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub shape: Shape,
    pub quantity: Quantity,
    pub fill: Fill,
    pub color: Color,
}

impl Card {
    pub fn new(shape: Shape, quantity: Quantity, fill: Fill, color: Color) -> Card {
        Card {
            shape,
            quantity,
            fill,
            color,
        }
    }
}

impl Distribution<Card> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        Card {
            shape: self.sample(rng),
            quantity: self.sample(rng),
            fill: self.sample(rng),
            color: self.sample(rng),
        }
    }
}

/// One of the four dimensions that a Set card can vary in.
///
/// Describes the shape that is on the card.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Shape {
    Diamond,
    Oval,
    Squiggle,
}

impl Distribution<Shape> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
        match rng.random_range(0..=2) {
            0 => Shape::Diamond,
            1 => Shape::Oval,
            _ => Shape::Squiggle,
        }
    }
}

impl Shape {
    pub fn sample_std_uniform_excluding<R: Rng + ?Sized>(
        rng: &mut R,
        exclude: Shape,
    ) -> (Shape, Shape) {
        let shapes = match exclude {
            Shape::Diamond => [Shape::Oval, Shape::Squiggle],
            Shape::Oval => [Shape::Diamond, Shape::Squiggle],
            Shape::Squiggle => [Shape::Diamond, Shape::Oval],
        };
        match rng.random_range(0..=1) {
            0 => (shapes[0], shapes[1]),
            _ => (shapes[1], shapes[0]),
        }
    }
}

/// One of the four dimensions that a Set card can vary in.
///
/// Describes the number of shapes that are on the card.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Quantity {
    One,
    Two,
    Three,
}

impl Distribution<Quantity> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quantity {
        match rng.random_range(0..=2) {
            0 => Quantity::One,
            1 => Quantity::Two,
            _ => Quantity::Three,
        }
    }
}

impl Quantity {
    pub fn sample_std_uniform_excluding<R: Rng + ?Sized>(
        rng: &mut R,
        exclude: Quantity,
    ) -> (Quantity, Quantity) {
        let quantities = match exclude {
            Quantity::One => [Quantity::Two, Quantity::Three],
            Quantity::Two => [Quantity::One, Quantity::Three],
            Quantity::Three => [Quantity::One, Quantity::Two],
        };
        match rng.random_range(0..=1) {
            0 => (quantities[0], quantities[1]),
            _ => (quantities[1], quantities[0]),
        }
    }
}

/// One of the four dimensions that a Set card can vary in.
///
/// Describes the inside of the shape(s) that are on the card.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fill {
    Empty,
    Dashed,
    Filled,
}

impl Distribution<Fill> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Fill {
        match rng.random_range(0..=2) {
            0 => Fill::Empty,
            1 => Fill::Dashed,
            _ => Fill::Filled,
        }
    }
}

impl Fill {
    pub fn sample_std_uniform_excluding<R: Rng + ?Sized>(
        rng: &mut R,
        exclude: Fill,
    ) -> (Fill, Fill) {
        let fills = match exclude {
            Fill::Empty => [Fill::Dashed, Fill::Filled],
            Fill::Dashed => [Fill::Empty, Fill::Filled],
            Fill::Filled => [Fill::Empty, Fill::Dashed],
        };
        match rng.random_range(0..=1) {
            0 => (fills[0], fills[1]),
            _ => (fills[1], fills[0]),
        }
    }
}

/// One of the four dimensions that a Set card can vary in.
///
/// Describes the color of the shape(s) that are on the card.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Blue,
    Gold,
    Pink,
}

impl Distribution<Color> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.random_range(0..=2) {
            0 => Color::Blue,
            1 => Color::Gold,
            _ => Color::Pink,
        }
    }
}

impl Color {
    pub fn sample_std_uniform_excluding<R: Rng + ?Sized>(
        rng: &mut R,
        exclude: Color,
    ) -> (Color, Color) {
        let colors = match exclude {
            Color::Blue => [Color::Gold, Color::Pink],
            Color::Gold => [Color::Blue, Color::Pink],
            Color::Pink => [Color::Blue, Color::Gold],
        };
        match rng.random_range(0..=1) {
            0 => (colors[0], colors[1]),
            _ => (colors[1], colors[0]),
        }
    }
}

/// Initializes the game state.
pub fn initialize_game_state(mut commands: Commands) {
    let state = GameState {
        cards: randomize_initial_cards(),
        current_guess: vec![],
    };
    commands.insert_resource(state);
}

fn randomize_initial_cards() -> [Card; 12] {
    let mut rng = rand_pcg::Pcg32::from_seed([
        22, 6, 255, 8, 110, 150, 35, 88, 140, 203, 92, 174, 244, 39, 4, 5,
    ]);

    let [first_card, second_card, third_card] = generate_first_set(&mut rng);

    // after you have a first set, have to figure out how to get the next set
    // -- use a card from the same set as the first: results in 5 cards
    //    this cannot result in a new set.
    // -- get a new set: results in 6 cards.

    // check if any new sets can be made with the new cards added
    // by gathering any potential 2 cards where a third card added would make a set.

    // pick whether a set should be made with an existing card or not.

    // If it should, choose a card, and then pick how the next two cards should be chosen.
    // If it should not, generate another random set

    // Remember to shuffle the cards at the end.
    [
        first_card,
        second_card,
        third_card,
        first_card,
        first_card,
        first_card,
        first_card,
        first_card,
        first_card,
        first_card,
        first_card,
        first_card,
    ]
}

/// Randomly generates the first Set of cards.
fn generate_first_set<R: Rng + ?Sized>(mut rng: &mut R) -> [Card; 3] {
    // The first card is randomly generated.
    let first_card: Card = rng.sample(StandardUniform);
    // Decide how the next two cards in the same set should be chosen
    let (first_set_same_shape,
      first_set_same_quantity,
      first_set_same_fill,
      first_set_same_color): (bool, bool, bool, bool) =
      (rng.random(), rng.random(), rng.random(), rng.random());
    // For each aspect, the cards have to either be all the same or all different in that given aspect.
    let (second_card_shape, third_card_shape) = if first_set_same_shape {
        (first_card.shape, first_card.shape)
    } else {
        Shape::sample_std_uniform_excluding(&mut rng, first_card.shape)
    };
    let (second_card_quantity, third_card_quantity) = if first_set_same_quantity {
        (first_card.quantity, first_card.quantity)
    } else {
        Quantity::sample_std_uniform_excluding(&mut rng, first_card.quantity)
    };
    let (second_card_fill, third_card_fill) = if first_set_same_fill {
        (first_card.fill, first_card.fill)
    } else {
        Fill::sample_std_uniform_excluding(&mut rng, first_card.fill)
    };
    let (second_card_color, third_card_color) = if first_set_same_color {
        (first_card.color, first_card.color)
    } else {
        Color::sample_std_uniform_excluding(&mut rng, first_card.color)
    };
    let second_card = Card {
        shape: second_card_shape,
        quantity: second_card_quantity,
        fill: second_card_fill,
        color: second_card_color,
    };
    let third_card = Card {
        shape: third_card_shape,
        quantity: third_card_quantity,
        fill: third_card_fill,
        color: third_card_color,
    };

    [first_card, second_card, third_card]
}
