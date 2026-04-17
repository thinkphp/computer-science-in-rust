use std::io::{self, BufRead, Write};
use std::thread;
use std::time::Duration;

struct TOH {
    size: usize,
    moves: u32,
    towers: [Vec<usize>; 3],
}

impl TOH {
    fn new(difficulty: usize) -> Result<Self, String> {
        if difficulty < 3 || difficulty > 10 {
            return Err("Incorrect Difficulty. Please provide a number between 3 and 10.".to_string());
        }
        let mut towers: [Vec<usize>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        for i in (1..=difficulty).rev() {
            towers[0].push(i);
        }
        Ok(TOH { size: difficulty, moves: 0, towers })
    }

    fn make_row(&self, disc: usize) -> String {
        if disc == 0 {
            let spaces = " ".repeat(self.size + 2);
            return format!("{}||{}", spaces, spaces);
        }
        if disc == self.size + 1 {
            let bar = "=".repeat((self.size + 2) * 2);
            return format!(" {} ", bar);
        }
        let left_pad  = " ".repeat(self.size - disc);
        let stars     = "*".repeat(disc);
        let right_pad = " ".repeat(self.size - disc);
        format!("  {}{}**{}{}  ", left_pad, stars, stars, right_pad)
    }

    fn render(&self) {
        // Top bar (empty rod tip)
        let top = self.make_row(0);
        println!("{}{}{}", top, top, top);

        // Rows from top to bottom
        for r in (0..self.size).rev() {
            let row: String = (0..3)
                .map(|t| {
                    let disc = if r < self.towers[t].len() { self.towers[t][r] } else { 0 };
                    self.make_row(disc)
                })
                .collect();
            println!("{}", row);
        }

        // Bottom bar (base)
        let bot = self.make_row(self.size + 1);
        println!("{}{}{}\n", bot, bot, bot);
    }

    fn valid_move(&self, from: usize, to: usize) -> bool {
        if from > 2 || to > 2          { return false; }
        if self.towers[from].is_empty() { return false; }
        match self.towers[to].last() {
            None       => true,
            Some(&top) => self.towers[from].last().map_or(false, |&d| d < top),
        }
    }

    fn do_move(&mut self, from: usize, to: usize) {
        if self.valid_move(from, to) {
            println!("Moving disc from tower {} to tower {}\n", from, to);
            let disc = self.towers[from].pop().unwrap();
            self.towers[to].push(disc);
            self.moves += 1;
            self.render();
        } else {
            println!("Invalid Move");
        }
    }

    fn won(&self) -> bool {
        self.towers[1].len() == self.size || self.towers[2].len() == self.size
    }

    // ─── Algoritmul recursiv Towers of Hanoi ─────────────────────────────
    // Mută `n` discuri de pe `src` pe `dst` folosind `aux` ca tijă auxiliară.
    // Număr de mutări: 2^n - 1 (optim).
    fn solve_recursive(&mut self, n: usize, src: usize, dst: usize, aux: usize) {
        if n == 0 {
            return;
        }
        self.solve_recursive(n - 1, src, aux, dst); // mută n-1 pe auxiliar
        self.do_move(src, dst);                      // mută discul cel mai mare
        thread::sleep(Duration::from_millis(500));   // pauză vizuală
        self.solve_recursive(n - 1, aux, dst, src); // mută n-1 de pe auxiliar pe dest
    }

    fn auto_solve(&mut self) {
        let optimal = (1u32 << self.size) - 1;
        println!("\n=== AUTO-SOLVE ===");
        println!("Mută toate discurile de pe tija 0 pe tija 2.");
        println!("Număr optim de mutări: {}\n", optimal);
        self.render();
        thread::sleep(Duration::from_millis(800));

        let n = self.size;
        self.solve_recursive(n, 0, 2, 1); // src=0, dst=2, aux=1

        println!("Puzzle rezolvat automat în {} mutări!", self.moves);
        if self.moves == optimal {
            println!("Soluție optimă. Excelent!");
        }
    }
    // ─────────────────────────────────────────────────────────────────────

    fn play(&mut self) {
        let letter = (b'A' + (rand_letter() % 26)) as char;
        println!("\n\nWelcome back to another exciting game of Towers of Hanoi!");
        println!("This game is brought to you by the letter '{}'", letter);
        println!("\nThis game's difficulty is set to {}", self.size);
        println!("Move discs by entering the source and destination tower.");
        println!("Commands: '0 1'  →  move  |  'solve'  →  auto-solve  |  'quit'  →  exit");
        println!("Good luck and have fun!\n");

        self.render();

        let stdin = io::stdin();
        loop {
            if self.won() { break; }

            print!("Enter your move (Ex: '0 1'): ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            if stdin.lock().read_line(&mut line).is_err() { break; }
            let line = line.trim();

            match line.to_lowercase().as_str() {
                "quit" => {
                    println!("Quitting the game...");
                    return;
                }
                "solve" => {
                    self.auto_solve();
                    return;
                }
                _ => {}
            }

            // Parsează "X Y"
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                println!("Invalid input. Try: '0 1'");
                continue;
            }
            match (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                (Ok(from), Ok(to)) => self.do_move(from, to),
                _ => println!("Invalid numbers. Towers are 0, 1, 2."),
            }
        }

        if self.won() {
            println!("You've solved the puzzle!\nCongratulations!");
            println!("It took you {} moves.", self.moves);
            let optimal = (1u32 << self.size) - 1;
            if self.moves == optimal {
                println!("This was an optimal solution. Great Job!");
            } else {
                println!("The optimal solution for this difficulty is {} moves.", optimal);
            }
        }
    }
}

// Pseudo-random fără dependențe externe (seed din SystemTime)
fn rand_letter() -> u8 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    (nanos % 256) as u8
}

fn main() {
    print!("Enter a difficulty (3 - 10): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");

    let difficulty: usize = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Please enter a valid number.");
            return;
        }
    };

    match TOH::new(difficulty) {
        Ok(mut game) => game.play(),
        Err(e) => eprintln!("{}", e),
    }
}
