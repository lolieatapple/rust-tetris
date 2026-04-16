use macroquad::prelude::*;

// ── Window ──────────────────────────────────────────────────────────────────
const WIN_W: i32 = 560;
const WIN_H: i32 = 700;

// ── Board ───────────────────────────────────────────────────────────────────
const COLS: usize = 10;
const ROWS: usize = 20;
const CELL: f32 = 30.0;
const BOARD_X: f32 = 30.0;
const BOARD_Y: f32 = 50.0;

// ── Timing (seconds) ───────────────────────────────────────────────────────
const DAS_DELAY: f32 = 0.17;
const ARR_RATE: f32 = 0.05;
const LOCK_DELAY: f32 = 0.5;

// ── Tetromino data ─────────────────────────────────────────────────────────
type Cells = [(i32, i32); 4];
type Rotations = [Cells; 4];

const I_PIECE: Rotations = [
    [(0, 0), (0, 1), (0, 2), (0, 3)],
    [(0, 2), (1, 2), (2, 2), (3, 2)],
    [(2, 0), (2, 1), (2, 2), (2, 3)],
    [(0, 1), (1, 1), (2, 1), (3, 1)],
];
const O_PIECE: Rotations = [
    [(0, 0), (0, 1), (1, 0), (1, 1)],
    [(0, 0), (0, 1), (1, 0), (1, 1)],
    [(0, 0), (0, 1), (1, 0), (1, 1)],
    [(0, 0), (0, 1), (1, 0), (1, 1)],
];
const T_PIECE: Rotations = [
    [(0, 1), (1, 0), (1, 1), (1, 2)],
    [(0, 1), (1, 1), (1, 2), (2, 1)],
    [(1, 0), (1, 1), (1, 2), (2, 1)],
    [(0, 1), (1, 0), (1, 1), (2, 1)],
];
const S_PIECE: Rotations = [
    [(0, 1), (0, 2), (1, 0), (1, 1)],
    [(0, 0), (1, 0), (1, 1), (2, 1)],
    [(1, 1), (1, 2), (2, 0), (2, 1)],
    [(0, 0), (1, 0), (1, 1), (2, 1)],
];
const Z_PIECE: Rotations = [
    [(0, 0), (0, 1), (1, 1), (1, 2)],
    [(0, 2), (1, 1), (1, 2), (2, 1)],
    [(1, 0), (1, 1), (2, 1), (2, 2)],
    [(0, 1), (1, 0), (1, 1), (2, 0)],
];
const L_PIECE: Rotations = [
    [(0, 2), (1, 0), (1, 1), (1, 2)],
    [(0, 1), (1, 1), (2, 1), (2, 2)],
    [(1, 0), (1, 1), (1, 2), (2, 0)],
    [(0, 0), (0, 1), (1, 1), (2, 1)],
];
const J_PIECE: Rotations = [
    [(0, 0), (1, 0), (1, 1), (1, 2)],
    [(0, 1), (0, 2), (1, 1), (2, 1)],
    [(1, 0), (1, 1), (1, 2), (2, 2)],
    [(0, 1), (1, 1), (2, 0), (2, 1)],
];

const ALL_PIECES: [Rotations; 7] = [I_PIECE, O_PIECE, T_PIECE, S_PIECE, Z_PIECE, L_PIECE, J_PIECE];

// SRS wall kick data (JLSTZ)
const KICKS_JLSTZ: [[(i32, i32); 5]; 8] = [
    [(0, 0), (0, -1), (-1, -1), (2, 0), (2, -1)],   // 0→R
    [(0, 0), (0, 1), (1, 1), (-2, 0), (-2, 1)],      // R→0
    [(0, 0), (0, 1), (1, 1), (-2, 0), (-2, 1)],      // R→2
    [(0, 0), (0, -1), (-1, -1), (2, 0), (2, -1)],    // 2→R
    [(0, 0), (0, 1), (-1, 1), (2, 0), (2, 1)],       // 2→L
    [(0, 0), (0, -1), (1, -1), (-2, 0), (-2, -1)],   // L→2
    [(0, 0), (0, -1), (1, -1), (-2, 0), (-2, -1)],   // L→0
    [(0, 0), (0, 1), (-1, 1), (2, 0), (2, 1)],       // 0→L
];

// SRS wall kick data (I-piece)
const KICKS_I: [[(i32, i32); 5]; 8] = [
    [(0, 0), (0, -2), (0, 1), (1, -2), (-2, 1)],     // 0→R
    [(0, 0), (0, 2), (0, -1), (-1, 2), (2, -1)],     // R→0
    [(0, 0), (0, -1), (0, 2), (-2, -1), (1, 2)],     // R→2
    [(0, 0), (0, 1), (0, -2), (2, 1), (-1, -2)],     // 2→R
    [(0, 0), (0, 2), (0, -1), (1, 2), (-2, -1)],     // 2→L
    [(0, 0), (0, -2), (0, 1), (-1, -2), (2, 1)],     // L→2
    [(0, 0), (0, 1), (0, -2), (-2, 1), (1, -2)],     // L→0
    [(0, 0), (0, -1), (0, 2), (2, -1), (-1, 2)],     // 0→L
];

fn piece_color(kind: usize) -> Color {
    match kind {
        0 => Color::new(0.30, 0.85, 0.90, 1.0),  // I - soft cyan
        1 => Color::new(0.95, 0.85, 0.35, 1.0),  // O - warm yellow
        2 => Color::new(0.70, 0.40, 0.90, 1.0),  // T - soft purple
        3 => Color::new(0.40, 0.85, 0.50, 1.0),  // S - soft green
        4 => Color::new(0.92, 0.38, 0.42, 1.0),  // Z - soft red
        5 => Color::new(0.95, 0.65, 0.30, 1.0),  // L - warm orange
        6 => Color::new(0.40, 0.50, 0.92, 1.0),  // J - soft blue
        _ => WHITE,
    }
}

fn ghost_color(kind: usize) -> Color {
    let c = piece_color(kind);
    Color::new(c.r, c.g, c.b, 0.25)
}

// ── Particle ───────────────────────────────────────────────────────────────
struct Particle {
    pos: Vec2,
    vel: Vec2,
    color: Color,
    life: f32,
}

// ── Floating text ──────────────────────────────────────────────────────────
struct FloatingText {
    pos: Vec2,
    text: String,
    color: Color,
    life: f32,
}

// ── 7-bag randomizer ───────────────────────────────────────────────────────
struct Bag {
    pieces: Vec<usize>,
}

impl Bag {
    fn new() -> Self {
        Self { pieces: Vec::new() }
    }

    fn next(&mut self) -> usize {
        if self.pieces.is_empty() {
            self.refill();
        }
        self.pieces.pop().unwrap()
    }

    fn refill(&mut self) {
        self.pieces = vec![0, 1, 2, 3, 4, 5, 6];
        for i in (1..7).rev() {
            let j = rand::gen_range(0, i + 1);
            self.pieces.swap(i, j);
        }
    }
}

// ── Game state ─────────────────────────────────────────────────────────────
#[derive(PartialEq)]
enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}

struct Game {
    board: [[Option<usize>; COLS]; ROWS],
    current_kind: usize,
    current_rot: usize,
    current_x: i32,
    current_y: i32,
    bag: Bag,
    hold_piece: Option<usize>,
    hold_used: bool,
    next_pieces: Vec<usize>,

    gravity_timer: f32,
    lock_timer: f32,
    lock_resets: u32,

    das_dir: i32,
    das_timer: f32,
    arr_timer: f32,

    score: u32,
    lines_cleared: u32,
    level: u32,
    combo: i32,
    back_to_back: bool,

    state: GameState,
    particles: Vec<Particle>,
    floating_texts: Vec<FloatingText>,
    screen_shake: f32,

    clearing_rows: Vec<usize>,
    clear_anim_timer: f32,

    stars: Vec<(f32, f32, f32)>,
}

impl Game {
    fn new() -> Self {
        let mut bag = Bag::new();
        let kind = bag.next();
        let next_pieces: Vec<usize> = (0..5).map(|_| bag.next()).collect();

        let mut stars = Vec::new();
        for _ in 0..80 {
            stars.push((
                rand::gen_range(0.0, WIN_W as f32),
                rand::gen_range(0.0, WIN_H as f32),
                rand::gen_range(0.3, 1.0),
            ));
        }

        let mut game = Game {
            board: [[None; COLS]; ROWS],
            current_kind: kind,
            current_rot: 0,
            current_x: 0,
            current_y: 0,
            bag,
            hold_piece: None,
            hold_used: false,
            next_pieces,
            gravity_timer: 0.0,
            lock_timer: 0.0,
            lock_resets: 0,
            das_dir: 0,
            das_timer: 0.0,
            arr_timer: 0.0,
            score: 0,
            lines_cleared: 0,
            level: 1,
            combo: -1,
            back_to_back: false,
            state: GameState::Playing,
            particles: Vec::new(),
            floating_texts: Vec::new(),
            screen_shake: 0.0,
            clearing_rows: Vec::new(),
            clear_anim_timer: 0.0,
            stars,
        };
        game.spawn_piece(kind);
        game
    }

    fn gravity_interval(&self) -> f32 {
        match self.level {
            1 => 1.0,
            2 => 0.793,
            3 => 0.618,
            4 => 0.473,
            5 => 0.355,
            6 => 0.262,
            7 => 0.190,
            8 => 0.135,
            9 => 0.094,
            10 => 0.064,
            11..=12 => 0.043,
            13..=15 => 0.028,
            16..=18 => 0.018,
            19..=28 => 0.011,
            _ => 0.007,
        }
    }

    fn fits(&self, kind: usize, rot: usize, x: i32, y: i32) -> bool {
        let cells = ALL_PIECES[kind][rot];
        for (dr, dc) in cells {
            let r = dr + y;
            let c = dc + x;
            if c < 0 || c >= COLS as i32 || r >= ROWS as i32 {
                return false;
            }
            if r < 0 {
                continue;
            }
            if self.board[r as usize][c as usize].is_some() {
                return false;
            }
        }
        true
    }

    fn spawn_piece(&mut self, kind: usize) {
        self.current_kind = kind;
        self.current_rot = 0;
        self.current_x = 3;
        self.current_y = -1;
        self.lock_timer = 0.0;
        self.lock_resets = 0;
        self.gravity_timer = 0.0;

        if !self.fits(kind, 0, self.current_x, self.current_y) {
            self.state = GameState::GameOver;
        }
    }

    fn next_piece(&mut self) {
        let kind = self.next_pieces.remove(0);
        self.next_pieces.push(self.bag.next());
        self.hold_used = false;
        self.spawn_piece(kind);
    }

    fn hold(&mut self) {
        if self.hold_used {
            return;
        }
        self.hold_used = true;
        let old_kind = self.current_kind;
        if let Some(held) = self.hold_piece {
            self.hold_piece = Some(old_kind);
            self.spawn_piece(held);
        } else {
            self.hold_piece = Some(old_kind);
            self.next_piece();
        }
    }

    fn try_move(&mut self, dx: i32, dy: i32) -> bool {
        let nx = self.current_x + dx;
        let ny = self.current_y + dy;
        if self.fits(self.current_kind, self.current_rot, nx, ny) {
            self.current_x = nx;
            self.current_y = ny;
            true
        } else {
            false
        }
    }

    fn try_rotate(&mut self, clockwise: bool) {
        let old_rot = self.current_rot;
        let new_rot = if clockwise {
            (old_rot + 1) % 4
        } else {
            (old_rot + 3) % 4
        };

        let kick_index = match (old_rot, clockwise) {
            (0, true) => 0,
            (1, false) => 1,
            (1, true) => 2,
            (2, false) => 3,
            (2, true) => 4,
            (3, false) => 5,
            (3, true) => 6,
            (0, false) => 7,
            _ => 0,
        };

        let kicks = if self.current_kind == 0 {
            &KICKS_I[kick_index]
        } else {
            &KICKS_JLSTZ[kick_index]
        };

        for &(kick_dy, kick_dx) in kicks {
            let nx = self.current_x + kick_dx;
            let ny = self.current_y - kick_dy;
            if self.fits(self.current_kind, new_rot, nx, ny) {
                self.current_x = nx;
                self.current_y = ny;
                self.current_rot = new_rot;
                if self.lock_resets < 15 {
                    self.lock_timer = 0.0;
                    self.lock_resets += 1;
                }
                return;
            }
        }
    }

    fn hard_drop(&mut self) {
        let mut dropped = 0;
        while self.try_move(0, 1) {
            dropped += 1;
        }
        self.score += dropped * 2;
        self.lock_piece();
    }

    fn ghost_y(&self) -> i32 {
        let mut gy = self.current_y;
        while self.fits(self.current_kind, self.current_rot, self.current_x, gy + 1) {
            gy += 1;
        }
        gy
    }

    fn lock_piece(&mut self) {
        let cells = ALL_PIECES[self.current_kind][self.current_rot];
        for (dr, dc) in cells {
            let r = dr + self.current_y;
            let c = dc + self.current_x;
            if r >= 0 && r < ROWS as i32 && c >= 0 && c < COLS as i32 {
                self.board[r as usize][c as usize] = Some(self.current_kind);
            }
        }

        // lock particles
        let color = piece_color(self.current_kind);
        for (dr, dc) in cells {
            let r = dr + self.current_y;
            let c = dc + self.current_x;
            let px = BOARD_X + c as f32 * CELL + CELL * 0.5;
            let py = BOARD_Y + r as f32 * CELL + CELL * 0.5;
            for _ in 0..3 {
                self.particles.push(Particle {
                    pos: vec2(px, py),
                    vel: vec2(rand::gen_range(-40.0, 40.0), rand::gen_range(-60.0, -10.0)),
                    color,
                    life: rand::gen_range(0.3, 0.6),
                });
            }
        }

        self.check_lines();
    }

    fn check_lines(&mut self) {
        let mut full_rows = Vec::new();
        for r in 0..ROWS {
            if self.board[r].iter().all(|c| c.is_some()) {
                full_rows.push(r);
            }
        }

        if full_rows.is_empty() {
            self.combo = -1;
            self.next_piece();
            return;
        }

        self.clearing_rows = full_rows;
        self.clear_anim_timer = 0.3;
    }

    fn finish_clear(&mut self) {
        let full_rows = std::mem::take(&mut self.clearing_rows);
        let n = full_rows.len() as u32;

        // spawn particles
        for &r in &full_rows {
            let color = piece_color(rand::gen_range(0, 7));
            for c in 0..COLS {
                let px = BOARD_X + c as f32 * CELL + CELL * 0.5;
                let py = BOARD_Y + r as f32 * CELL + CELL * 0.5;
                for _ in 0..4 {
                    self.particles.push(Particle {
                        pos: vec2(px, py),
                        vel: vec2(
                            rand::gen_range(-120.0, 120.0),
                            rand::gen_range(-100.0, -20.0),
                        ),
                        color,
                        life: rand::gen_range(0.4, 0.9),
                    });
                }
            }
        }

        // remove rows — build new board by filtering out cleared rows
        let mut new_board = [[None; COLS]; ROWS];
        let mut dest = ROWS - 1;
        for r in (0..ROWS).rev() {
            if !full_rows.contains(&r) {
                new_board[dest] = self.board[r];
                dest = dest.wrapping_sub(1);
            }
        }
        self.board = new_board;

        // scoring
        self.combo += 1;
        let is_tetris = n == 4;
        let base = match n {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };
        let mut points = base * self.level;

        if is_tetris {
            if self.back_to_back {
                points = points * 3 / 2;
            }
            self.back_to_back = true;
        } else {
            self.back_to_back = false;
        }

        if self.combo > 0 {
            points += 50 * self.combo as u32 * self.level;
        }

        self.score += points;
        self.lines_cleared += n;
        self.level = (self.lines_cleared / 10) + 1;

        self.screen_shake = match n {
            1 => 0.08,
            2 => 0.12,
            3 => 0.18,
            4 => 0.3,
            _ => 0.0,
        };

        let text = match n {
            1 => "SINGLE".to_string(),
            2 => "DOUBLE".to_string(),
            3 => "TRIPLE".to_string(),
            4 => "TETRIS!".to_string(),
            _ => format!("{}!", n),
        };
        let mid_r = full_rows[full_rows.len() / 2] as f32;
        let text_color = if is_tetris {
            Color::new(1.0, 0.9, 0.0, 1.0)
        } else {
            WHITE
        };
        self.floating_texts.push(FloatingText {
            pos: vec2(BOARD_X + COLS as f32 * CELL + 20.0, BOARD_Y + mid_r * CELL),
            text,
            color: text_color,
            life: 1.2,
        });
        // show points earned
        self.floating_texts.push(FloatingText {
            pos: vec2(BOARD_X + COLS as f32 * CELL + 20.0, BOARD_Y + mid_r * CELL + 22.0),
            text: format!("+{}", points),
            color: Color::new(1.0, 0.85, 0.3, 1.0),
            life: 1.2,
        });

        if self.combo > 0 {
            self.floating_texts.push(FloatingText {
                pos: vec2(
                    BOARD_X + COLS as f32 * CELL + 20.0,
                    BOARD_Y + mid_r * CELL + 44.0,
                ),
                text: format!("COMBO x{}", self.combo),
                color: Color::new(0.3, 1.0, 0.5, 1.0),
                life: 1.0,
            });
        }

        self.next_piece();
    }

    fn update(&mut self) {
        let dt = get_frame_time().min(0.033);

        // line clear animation
        if !self.clearing_rows.is_empty() {
            self.clear_anim_timer -= dt;
            if self.clear_anim_timer <= 0.0 {
                self.finish_clear();
            }
            self.update_particles(dt);
            self.update_floating_texts(dt);
            return;
        }

        if self.screen_shake > 0.0 {
            self.screen_shake -= dt;
        }

        // ── DAS / movement ─────────────────────────────────────────────
        let left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
        let right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
        let dir = if left && !right {
            -1
        } else if right && !left {
            1
        } else {
            0
        };

        if dir != 0 {
            if dir != self.das_dir {
                self.das_dir = dir;
                self.das_timer = 0.0;
                self.arr_timer = 0.0;
                if self.try_move(dir, 0) && self.lock_resets < 15 {
                    self.lock_timer = 0.0;
                    self.lock_resets += 1;
                }
            } else {
                self.das_timer += dt;
                if self.das_timer >= DAS_DELAY {
                    self.arr_timer += dt;
                    while self.arr_timer >= ARR_RATE {
                        self.arr_timer -= ARR_RATE;
                        if self.try_move(dir, 0) && self.lock_resets < 15 {
                            self.lock_timer = 0.0;
                            self.lock_resets += 1;
                        }
                    }
                }
            }
        } else {
            self.das_dir = 0;
            self.das_timer = 0.0;
            self.arr_timer = 0.0;
        }

        // ── Rotation ───────────────────────────────────────────────────
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            self.try_rotate(true);
        }
        if is_key_pressed(KeyCode::Z) {
            self.try_rotate(false);
        }

        // ── Hold ───────────────────────────────────────────────────────
        if is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::LeftShift) {
            self.hold();
        }

        // ── Hard drop ──────────────────────────────────────────────────
        if is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::Down)
            || is_key_pressed(KeyCode::S)
        {
            self.hard_drop();
            self.update_particles(dt);
            self.update_floating_texts(dt);
            return;
        }

        // ── Gravity ────────────────────────────────────────────────────
        self.gravity_timer += dt;
        while self.gravity_timer >= self.gravity_interval() {
            self.gravity_timer -= self.gravity_interval();
            if self.try_move(0, 1) {
                self.lock_timer = 0.0;
            }
        }

        // ── Lock delay ─────────────────────────────────────────────────
        let on_ground =
            !self.fits(self.current_kind, self.current_rot, self.current_x, self.current_y + 1);
        if on_ground {
            self.lock_timer += dt;
            if self.lock_timer >= LOCK_DELAY {
                self.lock_piece();
            }
        } else {
            self.lock_timer = 0.0;
        }

        self.update_particles(dt);
        self.update_floating_texts(dt);
    }

    fn update_particles(&mut self, dt: f32) {
        for p in &mut self.particles {
            p.pos += p.vel * dt;
            p.vel *= 0.96;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);
    }

    fn update_floating_texts(&mut self, dt: f32) {
        for t in &mut self.floating_texts {
            t.pos.y -= 30.0 * dt;
            t.life -= dt;
        }
        self.floating_texts.retain(|t| t.life > 0.0);
    }

    fn draw(&self) {
        let shake = if self.screen_shake > 0.0 {
            vec2(rand::gen_range(-3.0, 3.0), rand::gen_range(-3.0, 3.0))
        } else {
            Vec2::ZERO
        };
        let sx = shake.x;
        let sy = shake.y;

        // background
        clear_background(Color::new(0.08, 0.07, 0.14, 1.0));
        let t = get_time() as f32;
        for &(x, y, b) in &self.stars {
            let flicker = 0.5 + 0.5 * ((t * 1.5 + b * 10.0).sin());
            let alpha = b * flicker * 0.4;
            draw_circle(x + sx, y + sy, 1.0, Color::new(0.7, 0.75, 1.0, alpha));
        }

        let bx = BOARD_X + sx;
        let by = BOARD_Y + sy;
        let bw = COLS as f32 * CELL;
        let bh = ROWS as f32 * CELL;

        // board glow (outer soft border)
        for i in 0..4 {
            let g = i as f32;
            let a = 0.06 - i as f32 * 0.015;
            draw_rectangle_lines(
                bx - 3.0 - g, by - 3.0 - g,
                bw + 6.0 + g * 2.0, bh + 6.0 + g * 2.0,
                1.5,
                Color::new(0.4, 0.45, 0.8, a),
            );
        }

        // board background
        draw_rectangle(bx, by, bw, bh, Color::new(0.05, 0.045, 0.10, 1.0));

        // subtle grid dots at intersections
        for r in 1..ROWS {
            for c in 1..COLS {
                let x = bx + c as f32 * CELL;
                let y = by + r as f32 * CELL;
                draw_circle(x, y, 0.8, Color::new(0.25, 0.25, 0.40, 0.25));
            }
        }

        // board border (crisp single line)
        draw_rectangle_lines(bx, by, bw, bh, 1.5, Color::new(0.3, 0.32, 0.55, 0.7));

        // placed blocks
        for r in 0..ROWS {
            let flashing = self.clearing_rows.contains(&r);
            for c in 0..COLS {
                if let Some(kind) = self.board[r][c] {
                    let x = BOARD_X + c as f32 * CELL + sx;
                    let y = BOARD_Y + r as f32 * CELL + sy;
                    if flashing {
                        let flash = ((self.clear_anim_timer * 18.0).sin() * 0.5 + 0.5).abs();
                        let fc = piece_color(kind);
                        let bright = Color::new(
                            (fc.r + 0.4).min(1.0),
                            (fc.g + 0.4).min(1.0),
                            (fc.b + 0.4).min(1.0),
                            0.5 + flash * 0.5,
                        );
                        draw_rectangle(x + 1.5, y + 1.5, CELL - 3.0, CELL - 3.0, bright);
                    } else {
                        draw_cell(x, y, piece_color(kind));
                    }
                }
            }
        }

        if self.clearing_rows.is_empty() {
            // ghost piece
            let gy = self.ghost_y();
            let cells = ALL_PIECES[self.current_kind][self.current_rot];
            let gc = ghost_color(self.current_kind);
            let pc = piece_color(self.current_kind);
            for (dr, dc) in cells {
                let r = dr + gy;
                let c = dc + self.current_x;
                if r >= 0 && r < ROWS as i32 && c >= 0 && c < COLS as i32 {
                    let x = BOARD_X + c as f32 * CELL + sx;
                    let y = BOARD_Y + r as f32 * CELL + sy;
                    draw_rectangle(x + 2.0, y + 2.0, CELL - 4.0, CELL - 4.0, gc);
                    draw_rectangle_lines(x + 2.0, y + 2.0, CELL - 4.0, CELL - 4.0, 1.5, Color::new(pc.r, pc.g, pc.b, 0.4));
                }
            }

            // current piece
            let color = piece_color(self.current_kind);
            for (dr, dc) in cells {
                let r = dr + self.current_y;
                let c = dc + self.current_x;
                if r >= 0 && r < ROWS as i32 && c >= 0 && c < COLS as i32 {
                    let x = BOARD_X + c as f32 * CELL + sx;
                    let y = BOARD_Y + r as f32 * CELL + sy;
                    draw_cell(x, y, color);
                }
            }
        }

        // particles
        for p in &self.particles {
            let alpha = (p.life * 2.0).min(1.0);
            let c = Color::new(p.color.r, p.color.g, p.color.b, alpha);
            draw_circle(p.pos.x + sx, p.pos.y + sy, 2.5, c);
        }

        // floating texts
        for ft in &self.floating_texts {
            let alpha = (ft.life * 1.5).min(1.0);
            let c = Color::new(ft.color.r, ft.color.g, ft.color.b, alpha);
            draw_text(&ft.text, ft.pos.x + sx, ft.pos.y + sy, 22.0, c);
        }

        // ── Side panels ────────────────────────────────────────────────
        let panel_x = BOARD_X + COLS as f32 * CELL + 30.0;
        let label_color = Color::new(0.55, 0.55, 0.75, 0.8);
        let value_color = Color::new(0.9, 0.92, 1.0, 1.0);

        // Hold
        draw_text("HOLD", panel_x + sx, BOARD_Y + 14.0 + sy, 18.0, label_color);
        // hold box background
        draw_rectangle(
            panel_x - 4.0 + sx, BOARD_Y + 22.0 + sy,
            MINI_BOX_W + 8.0, MINI_BOX_H + 6.0,
            Color::new(0.10, 0.09, 0.18, 0.6),
        );
        draw_rectangle_lines(
            panel_x - 4.0 + sx, BOARD_Y + 22.0 + sy,
            MINI_BOX_W + 8.0, MINI_BOX_H + 6.0,
            1.0,
            Color::new(0.3, 0.32, 0.55, 0.4),
        );
        if let Some(kind) = self.hold_piece {
            draw_mini_piece(
                panel_x + sx,
                BOARD_Y + 25.0 + sy,
                kind,
                if self.hold_used { 0.3 } else { 1.0 },
            );
        } else {
            let hint = "Press C";
            let hm = measure_text(hint, None, 14, 1.0);
            draw_text(
                hint,
                panel_x - 4.0 + (MINI_BOX_W + 8.0 - hm.width) * 0.5 + sx,
                BOARD_Y + 22.0 + (MINI_BOX_H + 6.0) * 0.5 + 5.0 + sy,
                14.0,
                Color::new(0.35, 0.35, 0.50, 0.5),
            );
        }

        // Next pieces
        let next_y = BOARD_Y + 22.0 + MINI_BOX_H + 24.0;
        draw_text("NEXT", panel_x + sx, next_y + sy, 18.0, label_color);
        for (i, &kind) in self.next_pieces.iter().enumerate() {
            let fade = if i == 0 { 1.0 } else { 0.7 - i as f32 * 0.08 };
            draw_mini_piece(
                panel_x + sx,
                next_y + 12.0 + i as f32 * MINI_BOX_H + sy,
                kind,
                fade.max(0.35),
            );
        }

        // Score / Level / Lines
        let info_y = next_y + 20.0 + 5.0 * MINI_BOX_H;
        draw_text("SCORE", panel_x + sx, info_y + sy, 16.0, label_color);
        draw_text(&self.score.to_string(), panel_x + sx, info_y + 20.0 + sy, 24.0, value_color);

        draw_text("LEVEL", panel_x + sx, info_y + 52.0 + sy, 16.0, label_color);
        draw_text(&self.level.to_string(), panel_x + sx, info_y + 72.0 + sy, 24.0, value_color);

        draw_text("LINES", panel_x + sx, info_y + 104.0 + sy, 16.0, label_color);
        draw_text(&self.lines_cleared.to_string(), panel_x + sx, info_y + 124.0 + sy, 24.0, value_color);

        // Controls hint
        draw_text(
            "ARROWS/AD  SPACE=Drop  Z/W=Rotate  C=Hold  P=Pause",
            12.0 + sx,
            WIN_H as f32 - 8.0 + sy,
            13.0,
            Color::new(0.30, 0.30, 0.45, 0.5),
        );
    }
}

fn draw_cell(x: f32, y: f32, color: Color) {
    let m = 1.5; // margin between cells

    // outer glow
    draw_rectangle(
        x + m - 1.0, y + m - 1.0,
        CELL - m * 2.0 + 2.0, CELL - m * 2.0 + 2.0,
        Color::new(color.r * 0.5, color.g * 0.5, color.b * 0.5, 0.3),
    );

    // main body
    draw_rectangle(x + m, y + m, CELL - m * 2.0, CELL - m * 2.0, color);

    // inner highlight (top-left soft glow)
    let hi = Color::new(1.0, 1.0, 1.0, 0.30);
    draw_rectangle(x + m, y + m, CELL - m * 2.0, (CELL - m * 2.0) * 0.35, hi);
    let hi2 = Color::new(1.0, 1.0, 1.0, 0.15);
    draw_rectangle(x + m, y + m, (CELL - m * 2.0) * 0.35, CELL - m * 2.0, hi2);

    // inner shine dot (top-left corner)
    draw_rectangle(
        x + m + 3.0, y + m + 3.0, 5.0, 5.0,
        Color::new(1.0, 1.0, 1.0, 0.35),
    );

    // subtle bottom-right darken
    let lo = Color::new(0.0, 0.0, 0.0, 0.15);
    draw_rectangle(x + m, y + CELL - m - 3.0, CELL - m * 2.0, 3.0, lo);
    draw_rectangle(x + CELL - m - 3.0, y + m, 3.0, CELL - m * 2.0, lo);
}

const MINI_S: f32 = CELL * 0.6;
const MINI_BOX_W: f32 = MINI_S * 5.0; // wide enough for I-piece (4 cells)
const MINI_BOX_H: f32 = MINI_S * 3.0; // tall enough for all pieces (2 cells + padding)

fn draw_mini_piece(x: f32, y: f32, kind: usize, alpha: f32) {
    let cells = ALL_PIECES[kind][0];
    let color = piece_color(kind);
    let c = Color::new(color.r, color.g, color.b, alpha);
    let mut min_c = 4;
    let mut max_c = 0;
    let mut min_r = 4;
    let mut max_r = 0;
    for (dr, dc) in cells {
        min_c = min_c.min(dc);
        max_c = max_c.max(dc);
        min_r = min_r.min(dr);
        max_r = max_r.max(dr);
    }
    let pw = (max_c - min_c + 1) as f32 * MINI_S;
    let ph = (max_r - min_r + 1) as f32 * MINI_S;
    let ox = x + (MINI_BOX_W - pw) * 0.5;
    let oy = y + (MINI_BOX_H - ph) * 0.5;
    for (dr, dc) in cells {
        let cx = ox + (dc - min_c) as f32 * MINI_S;
        let cy = oy + (dr - min_r) as f32 * MINI_S;
        let m = 1.0;
        draw_rectangle(cx + m, cy + m, MINI_S - m * 2.0, MINI_S - m * 2.0, c);
        // small highlight
        draw_rectangle(cx + m, cy + m, MINI_S - m * 2.0, (MINI_S - m * 2.0) * 0.3, Color::new(1.0, 1.0, 1.0, 0.2 * alpha));
    }
}

fn draw_menu() {
    clear_background(Color::new(0.06, 0.055, 0.12, 1.0));
    let t = get_time() as f32;

    for i in 0..60 {
        let x = ((i * 137 + 50) % WIN_W as usize) as f32;
        let y = ((i * 251 + 30) % WIN_H as usize) as f32;
        let b = 0.2 + 0.2 * ((t * 1.2 + i as f32 * 0.7).sin());
        draw_circle(x, y, 1.0, Color::new(0.7, 0.75, 1.0, b));
    }

    let title = "TETRIS";
    let title_size = 72.0;
    let m = measure_text(title, None, title_size as u16, 1.0);
    let tx = (WIN_W as f32 - m.width) * 0.5;
    let ty = 200.0;

    for (i, ch) in title.chars().enumerate() {
        let hue = (t * 0.3 + i as f32 * 0.15) % 1.0;
        let color = hsl_to_color(hue, 0.9, 0.65);
        let oy = (t * 2.5 + i as f32 * 0.8).sin() * 6.0;
        let cx = tx + i as f32 * (m.width / title.len() as f32);
        draw_text(&ch.to_string(), cx, ty + oy, title_size, color);
    }

    let sub = "Press ENTER to start";
    let sm = measure_text(sub, None, 22, 1.0);
    let alpha = 0.5 + 0.5 * (t * 2.0).sin();
    draw_text(
        sub,
        (WIN_W as f32 - sm.width) * 0.5,
        300.0,
        22.0,
        Color::new(0.8, 0.8, 1.0, alpha),
    );

    let controls = [
        "LEFT / RIGHT / A / D  -  Move",
        "UP / W  -  Rotate CW",
        "Z  -  Rotate CCW",
        "DOWN / S  -  Hard Drop",
        "SPACE  -  Hard Drop",
        "C / SHIFT  -  Hold",
        "P / ESC  -  Pause",
    ];
    for (i, &line) in controls.iter().enumerate() {
        draw_text(
            line,
            120.0,
            400.0 + i as f32 * 28.0,
            18.0,
            Color::new(0.5, 0.5, 0.7, 0.8),
        );
    }
}

fn draw_game_over(game: &Game) {
    game.draw();

    draw_rectangle(0.0, 0.0, WIN_W as f32, WIN_H as f32, Color::new(0.03, 0.02, 0.08, 0.75));

    let title = "GAME OVER";
    let tm = measure_text(title, None, 56, 1.0);
    draw_text(title, (WIN_W as f32 - tm.width) * 0.5, 260.0, 56.0, Color::new(0.92, 0.38, 0.42, 1.0));

    let score_text = format!("Score: {}", game.score);
    let sm = measure_text(&score_text, None, 30, 1.0);
    draw_text(&score_text, (WIN_W as f32 - sm.width) * 0.5, 320.0, 30.0, WHITE);

    let level_text = format!("Level: {}  Lines: {}", game.level, game.lines_cleared);
    let lm = measure_text(&level_text, None, 22, 1.0);
    draw_text(&level_text, (WIN_W as f32 - lm.width) * 0.5, 360.0, 22.0, Color::new(0.7, 0.7, 0.9, 1.0));

    let restart = "Press ENTER to restart";
    let rm = measure_text(restart, None, 20, 1.0);
    let alpha = 0.5 + 0.5 * (get_time() as f32 * 2.0).sin();
    draw_text(restart, (WIN_W as f32 - rm.width) * 0.5, 420.0, 20.0, Color::new(0.8, 0.8, 1.0, alpha));
}

fn hsl_to_color(h: f32, s: f32, l: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = match (h * 6.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    Color::new(r + m, g + m, b + m, 1.0)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_string(),
        window_width: WIN_W,
        window_height: WIN_H,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = GameState::Menu;
    let mut game = Game::new();

    loop {
        match state {
            GameState::Menu => {
                draw_menu();
                if is_key_pressed(KeyCode::Enter) {
                    game = Game::new();
                    state = GameState::Playing;
                }
            }
            GameState::Playing => {
                if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
                    state = GameState::Paused;
                } else {
                    game.update();
                    game.draw();
                    if game.state == GameState::GameOver {
                        state = GameState::GameOver;
                    }
                }
            }
            GameState::Paused => {
                game.draw();
                draw_rectangle(0.0, 0.0, WIN_W as f32, WIN_H as f32, Color::new(0.0, 0.0, 0.0, 0.6));
                let title = "PAUSED";
                let tm = measure_text(title, None, 56, 1.0);
                draw_text(title, (WIN_W as f32 - tm.width) * 0.5, 320.0, 56.0, WHITE);
                let hint = "Press P or ESC to resume";
                let hm = measure_text(hint, None, 20, 1.0);
                let alpha = 0.5 + 0.5 * (get_time() as f32 * 2.0).sin();
                draw_text(hint, (WIN_W as f32 - hm.width) * 0.5, 370.0, 20.0, Color::new(0.8, 0.8, 1.0, alpha));
                if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
                    state = GameState::Playing;
                }
            }
            GameState::GameOver => {
                draw_game_over(&game);
                if is_key_pressed(KeyCode::Enter) {
                    game = Game::new();
                    state = GameState::Playing;
                }
            }
        }
        next_frame().await;
    }
}
