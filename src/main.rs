use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

struct App<'a> {
    items: Vec<&'a str>,
    selected: usize,
    result: &'a str,
    selected_text: &'a str,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: vec![
                "Item 1",
                "Item 2",
                "Item 3",
                "Item 4",
                "Item 5",
                "Quit",
            ],
            selected: 0,
            result: "",
            selected_text: "",
        }
    }

    fn next(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
        self.update_selected_text();
    }

    fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.items.len() - 1;
        }
        self.update_selected_text();
    }

    fn selected_item(&self) -> &str {
        self.items[self.selected]
    }

    fn execute_selected_item(&mut self) {
        self.result = match self.selected_item() {
            "Item 1" => item_1_function(),
            "Item 2" => item_2_function(),
            "Item 3" => item_3_function(),
            "Item 4" => item_4_function(),
            "Item 5" => item_5_function(),
            "Quit" => "Quitting the application...",
            _ => "Unknown Item",
        };
    }

    fn update_selected_text(&mut self) {
        self.selected_text = match self.selected_item() {
            "Item 1" => " You have selected Item 1 You have selected Item 1",
            "Item 2" => " You have selected Item 2 You have selected Item 2",
            "Item 3" => " You have selected Item 3 You have selected Item 3",
            "Item 4" => " You have selected Item 4 You have selected Item 4",
            "Item 5" => " You have selected Item 5 You have selected Item 5",
            "Quit" => "You have selected Quit",
            _ => "Unknown selection",
        };
    }
}

fn item_1_function() -> &'static str {
    "Result of Item 1: This is a detailed description of what happens when Item 1 is selected."
}

fn item_2_function() -> &'static str {
    "Result of Item 2: This is a detailed description of what happens when Item 2 is selected."
}

fn item_3_function() -> &'static str {
    "Result of Item 3: This is a detailed description of what happens when Item 3 is selected."
}

fn item_4_function() -> &'static str {
    "Result of Item 4: This is a detailed description of what happens when Item 4 is selected."
}

fn item_5_function() -> &'static str {
    "Result of Item 5: This is a detailed description of what happens when Item 5 is selected."
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()), // Quit on 'q'
                KeyCode::Down => {
                    app.next();
                }
                KeyCode::Up => {
                    app.previous();
                }
                KeyCode::Enter => {
                    if app.selected_item() == "Quit" {
                        return Ok(()); // Quit on Enter if "Quit" is selected
                    }
                    app.execute_selected_item();
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut tui::Frame<B>, app: &App) {
    let size = f.size();
    
    // Define the title block
    let title_block = Block::default()
        .title(" CLI Application ")
        .borders(Borders::ALL)
        .style(Style::default().add_modifier(Modifier::BOLD));

    let inner_area = title_block.inner(size);
    
    // Add padding layout
    let layout_with_padding = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(1), // Left padding
                Constraint::Percentage(18), // Menu content
                Constraint::Percentage(1), // Middle padding
                Constraint::Percentage(70), // Result content
            ]
            .as_ref(),
        )
        .split(inner_area);

    let menu_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(65), // Menu
                Constraint::Percentage(25), // Selected item description
                Constraint::Percentage(10), // Footer
            ]
            .as_ref(),
        )
        .split(layout_with_padding[1]); // Menu content area

    let items: Vec<ListItem> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let content = if i == app.selected {
                Spans::from(vec![
                    Span::raw(" "), // Add two spaces for left padding
                    Span::styled(
                        *item,
                        Style::default().add_modifier(Modifier::BOLD | Modifier::REVERSED),
                    )
                ])
            } else {
                Spans::from(vec![
                    Span::raw(" "), // Add two spaces for left padding
                    Span::styled(
                        *item,
                        Style::default(),
                    )
                ])
            };
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Menu "))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    let selected_item = Paragraph::new(app.selected_text)
        .block(Block::default().borders(Borders::ALL).title(" Description "))
        .wrap(Wrap { trim: false });

    let result = Paragraph::new(app.result)
        .block(Block::default().borders(Borders::ALL).title(" Result "))
        .wrap(Wrap { trim: false });

    let footer = Paragraph::new(Spans::from(vec![
            Span::raw(" "), // Add a single space for left padding
            Span::styled(
                "Version 0.1.0 Jan Rock",
                Style::default().add_modifier(Modifier::ITALIC),
            )
        ]))
        .block(Block::default().borders(Borders::ALL).title(" About "))
        .wrap(Wrap { trim: false });

    f.render_widget(title_block, size);
    f.render_widget(list, menu_chunks[0]);
    f.render_widget(selected_item, menu_chunks[1]);
    f.render_widget(footer, menu_chunks[2]);
    f.render_widget(result, layout_with_padding[3]);
}

