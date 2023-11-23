use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use math_tutor_core::{create_math_lessons, Lesson};
use ratatui::{prelude::*, widgets::*};
struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

/// This struct holds the current state of the app. In particular, it has the `items` field which is
/// a wrapper around `ListState`. Keeping track of the items state let us render the associated
/// widget with its state and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
struct App<'a> {
    items: StatefulList<Lesson<'a>>,
    show_lesson_details: Option<i8>, // New field to track if lesson details should be shown
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            // into_iter.. is needed to convert from heapless vec we use for embedded devices.
            items: StatefulList::with_items(create_math_lessons().into_iter().collect()),
            show_lesson_details: Option::None,
        }
    }

    // /// Rotate through the event list.
    // /// This only exists to simulate some kind of "progress"
    // fn on_tick(&mut self) {
    //     let event = self.events.remove(0);
    //     self.events.push(event);
    // }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left => app.items.unselect(),
                        KeyCode::Down => app.items.next(),
                        KeyCode::Up => app.items.previous(),
                        KeyCode::Enter => {
                            // app.show_lesson_details = match app.show_lesson_details {
                            //     None => {}
                            //     Some(_) => {}
                            // }; // Toggle lesson details view
                        }
                        _ => {}
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            // app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.size());

    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let lines = vec![Line::from(i.title)];
            // for _ in 0..i.1 {
            //     lines.push(
            //         "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
            //             .italic()
            //             .into(),
            //     );
            // }
            ListItem::new(lines).style(Style::default().fg(Color::Green).bg(Color::Black))
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Lessons"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.items.state);

    if app.show_lesson_details.is_some() {
        if let Some(selected) = app.items.state.selected() {
            let lesson = &app.items.items[selected];
            let lesson_text = lesson
                .lessons
                .iter()
                .fold(String::new(), |mut acc, lesson| {
                    acc.push_str(lesson.as_str());
                    acc.push('\n');
                    acc
                });

            let paragraph = Paragraph::new(lesson_text.as_str()).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Lesson Details"),
            );
            f.render_widget(paragraph, chunks[1]);
        }
    }
}
