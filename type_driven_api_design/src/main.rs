// Following along with
//
// "Type-Driven API Design in Rust" by Will Crichton
// https://www.youtube.com/watch?v=bnnacleqg6k
//

// Having the compiler enforce what methods can be called
// and in what order the can be called at compile time.
// Use traits to define, organize your API, and prevent
// users from calling your API incorrectly

const CLEAR_TERMINAL_CODE: &str = "\x1B[2J\x1B[1;1H";

fn main() {
    let v = vec![1, 2, 3];
    //progress_vector_specific(v.clone());
    //progress_vector_generic(v.clone(), expensive_calculation);

    // use std::collections::HashSet;
    // //let mut h: HashSet<i32> = HashSet::from_iter(v.iter());
    // let mut h: HashSet<i32> = HashSet::new();
    // h.insert(0);
    // progress_iter_messy(v.iter(), expensive_calculation);
    // progress_iter_clean(h.iter(), expensive_calculation);

    // datastructure that represents iterator it's given
    // for n in Progress::new(v.iter()) {
    //     expensive_calculation(n)
    // }

    // Extend preexisting types using traits so we can attach methods to types we don't own.
    // for n in v.iter().progress() {
    //     expensive_calculation(n)
    // }

    // There were possible before restricting Iter to Iterator
    //let x = 1.progress();
    //for _ in x {} // Causes runtime error
    //let y = "blah".progress();

    // Bounded iterator
    // for n in v.iter().progress_two().with_bound() {
    //     expensive_calculation(n)
    // }

    // Unbounded iterator causes error because with with_bound requires ExtactSizedIterator to call.
    //for n in { 0.. }.progress_two().with_bound() {
    // for n in { 0.. }.progress_two() {
    //     expensive_calculation(&n)
    // }

    // Cavate
    // The complier will catch errors but the error messages are complier errors so they
    // can be a lot harder for a user of your API to figure out than building a descriptive
    // runtime error.

    let brkts = ('<', '>');
    for n in v
        .iter()
        .progress_three()
        .with_bound_three()
        .with_delims(brkts)
    {
        expensive_calculation(n)
    }

    // for n in v
    //     .iter()
    //     .progress_three()
    //     // Can't call with_delims because it works on ProgressThree sturcts with a bounded state
    //     .with_delims(brkts)
    // {
    //     expensive_calculation(n)
    // }

    // for n in { 0.. }
    //     .progress_three()
    //     // Fails because we can't call this without the iterator having ExactSizeIterator
    //     .with_bound_three()
    // {
    //     expensive_calculation(&n)
    // }
}

use std::{thread::sleep, time::Duration};
fn expensive_calculation<T>(n: &T) {
    sleep(Duration::from_secs(1));
}

// Specific to a vector of a specific type
fn progress_vector_specific(v: Vec<i32>) {
    let mut i = 1;
    for n in v.iter() {
        println!("{} {}", CLEAR_TERMINAL_CODE, "*".repeat(i));
        i += 1;
        expensive_calculation(&n);
    }
}

// Any vector of type T
fn progress_vector_generic<T>(v: Vec<T>, f: fn(&T) -> ()) {
    let mut i = 1;
    for n in v.iter() {
        println!("{} {}", CLEAR_TERMINAL_CODE, "*".repeat(i));
        i += 1;
        f(n);
    }
}

// Anything iterable (Iterator Trait)
fn progress_iter_messy<T, Iter>(iter: Iter, f: fn(T) -> ())
where
    Iter: Iterator<Item = T>,
{
    let mut i = 1;
    for n in iter {
        println!("{} {}", CLEAR_TERMINAL_CODE, "*".repeat(i));
        i += 1;
        f(n);
    }
}

// Anything iterable (Iterator Trait) cleaned up
fn progress_iter_clean<Iter>(iter: Iter, f: fn(Iter::Item) -> ())
where
    Iter: Iterator,
{
    let mut i = 1;
    for n in iter {
        println!("{} {}", CLEAR_TERMINAL_CODE, "*".repeat(i));
        i += 1;
        f(n);
    }
}

// Progress Version 1

struct Progress<Iter> {
    iter: Iter,
    i: usize,
}

// "Quantified" - For all types Iter impliement Progress of Iter
impl<Iter> Progress<Iter> {
    pub fn new(iter: Iter) -> Self {
        Progress { iter, i: 0 }
    }
}

// Turn into iterator
impl<Iter> Iterator for Progress<Iter>
where
    Iter: Iterator,
{
    fn next(&mut self) -> Option<Self::Item> {
        println!("{} {}", CLEAR_TERMINAL_CODE, "*".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }
    type Item = Iter::Item;
}

// Extend preexisting types using traits so we can attach methods to types we don't own.
// Looks kind of like C# extension methods.

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}

// For all types Iter implement the trait ProgressIteratorExt for that quantified (Iter) type.
// impl<Iter> ProgressIteratorExt for Iter {
//     fn progress(self) -> Progress<Self> {
//         Progress::new(self)
//     }
// }

// Improved version requires Iter to be a iteratable type.
// Prevents 0.progress() and blah".progress() above
impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

// Progress Version 2 (Detect if iterator is fixed size)
// Only show with_bounded as an option when the iterator has the
// fixed size trait

struct ProgressTwo<Iter> {
    iter: Iter,
    i: usize,
    bound: Option<usize>, // Store bound if we have one
}

impl<Iter> ProgressTwo<Iter> {
    pub fn new(iter: Iter) -> Self {
        ProgressTwo {
            iter,
            i: 0,
            bound: None,
        } // Assume no bound
    }
}

// Only implement this method where iterator has ExactSizeIterator trait
// Can only call this method when you are iterating over something that has a bound
impl<Iter> ProgressTwo<Iter>
where
    Iter: ExactSizeIterator,
{
    // Mutable version of self
    pub fn with_bound(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }
}

impl<Iter> Iterator for ProgressTwo<Iter>
where
    Iter: Iterator,
{
    fn next(&mut self) -> Option<Self::Item> {
        println!("{}", CLEAR_TERMINAL_CODE);

        match self.bound {
            Some(bound) => println!("[{}{}]", "*".repeat(self.i), " ".repeat(bound - self.i)),
            None => println!("{}", "*".repeat(self.i)),
        }

        self.i += 1;
        self.iter.next()
    }
    type Item = Iter::Item;
}

trait ProgressTwoIteratorExt: Sized {
    fn progress_two(self) -> ProgressTwo<Self>;
}

impl<Iter> ProgressTwoIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress_two(self) -> ProgressTwo<Self> {
        ProgressTwo::new(self)
    }
}

// Progress Version 3 (Type state)
// Using Type state to keep track of if this is a bounded progress bar.  If there
// We only show with_delims as an option if with_bounds was called beforehand.

// Unbounded state
struct Unbounded;

// Bounded state
struct Bounded {
    bound: usize,
    delims: (char, char),
}

struct ProgressThree<Iter, Bound> {
    iter: Iter,
    i: usize,
    bound: Bound,
    delims: (char, char),
}

trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &ProgressThree<Iter, Self>);
}

// This is mixing ProgressDisplay for unbounded with ProgressThree
impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &ProgressThree<Iter, Self>) {
        println!("{}", "*".repeat(progress.i));
    }
}

// This is mixing ProgressDisplay for bounded with ProgressThree
impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &ProgressThree<Iter, Self>) {
        println!(
            "{}{}{}{}",
            self.delims.0,
            "*".repeat(progress.i),
            " ".repeat(self.bound - progress.i),
            self.delims.1
        );
    }
}

// Initial state is unbounded
impl<Iter> ProgressThree<Iter, Unbounded> {
    pub fn new(iter: Iter) -> Self {
        ProgressThree {
            iter,
            i: 0,
            bound: Unbounded,
            delims: ('[', ']'),
        } // Assume no bound
    }
}

// We are changing the type state of the progress
// bar from Unbounded to Bounded when with_bounded
// is called
impl<Iter> ProgressThree<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    // Note that we are no longer returning Self (version 2) but
    // a bounded version of the ProgressThree type.
    pub fn with_bound_three(mut self) -> ProgressThree<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']'),
        };
        // Return distict structure/new type
        ProgressThree {
            i: self.i,
            iter: self.iter,
            bound,
            delims: self.delims,
        }
    }
}

impl<Iter, Bound> Iterator for ProgressThree<Iter, Bound>
where
    Iter: Iterator,
    Bound: ProgressDisplay,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}", CLEAR_TERMINAL_CODE);

        self.bound.display(&self);
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressThreeIteratorExt: Sized {
    fn progress_three(self) -> ProgressThree<Self, Unbounded>;
}

impl<Iter> ProgressThreeIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress_three(self) -> ProgressThree<Self, Unbounded> {
        ProgressThree::new(self)
    }
}

impl<Iter> ProgressThree<Iter, Bounded> {
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}
