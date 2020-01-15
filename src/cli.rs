use crate::error::*;
use crate::hero::{BaseValue, Hero};

use termion::event::{Event,Key};
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::input::TermRead;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Gauge, Widget, Paragraph, Text};
use tui::Terminal;
use rand::distributions::Uniform;
use clap::{Arg, App, AppSettings, SubCommand};

pub fn run_cli(hero: Hero) -> Result<()>
{
	let mut current_health = hero.health;
	let mut current_astral = hero.astral;
	let d20: Uniform<_> = (1..21).into();
	let mut rng = rand::thread_rng();
	let sleep_duration = std::time::Duration::from_millis(10);
	let mut cli_text = String::new();
	let mut cli_history = String::new();

	// Terminal initialization
	let stdout = std::io::stdout().into_raw_mode()?;
	let stdin = termion::async_stdin();
	let mut stdin = stdin.events();
	let stdout = MouseTerminal::from(stdout);
	let stdout = AlternateScreen::from(stdout);
	let backend = TermionBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;
	terminal.hide_cursor()?;

	'outer: loop
	{
		while let Some(event) = stdin.next()
		{
			match event?
			{
				Event::Key(Key::Char('\n')) =>
				{
					let split = cli_text.split_whitespace();

					let roll_skill = |skill: &str,bonus: &str| {};
					let change_stat = |stat: &mut isize, amount: &str| -> Result<bool>
					{
						*stat = stat.saturating_sub(amount.parse::<isize>()?);
						Ok(false)
					};

					let result: Result<bool> = (||
					{
						let mut args = vec![""];
						args.extend(split);
						let matches = crate::app()
							.subcommand
								( SubCommand::with_name("clear")
								.about("clear the screen")
								)
							.subcommand
								( SubCommand::with_name("damage")
								.about("do damage")
								.arg
									( Arg::with_name("foo")
									)
								)
							.subcommand
								( SubCommand::with_name("exit")
								.about("interactive mode")
								)
							.setting(AppSettings::ColorNever)
							.get_matches_from_safe(args.iter())?;
						
						if let Some(_matches) = matches.subcommand_matches("clear")
						{
							cli_history.clear();
							return Ok(false);
						}
						if let Some(matches) = matches.subcommand_matches("damage")
						{
							let damage = matches.value_of("foo").unwrap();
							change_stat(&mut current_health,&damage);
							return Ok(false);
						}
						if let Some(_matches) = matches.subcommand_matches("exit")
						{
							return Ok(true);
						}
						if let Some(_matches) = matches.subcommand_matches("dump")
						{
							cli_history.push_str(format!("{:?}",hero).as_str());
							return Ok(false);
						}

						Ok(false)
					})();

					match result
					{
						Ok(true) => break 'outer,
						Ok(false) => {},
						Err(error) =>
						{
							cli_history.push_str(error.description());
							cli_history.push('\n');
						},
					}

					cli_text.clear();
				},
				Event::Key(Key::Char(ch)) => { cli_text.push(ch); },
				Event::Key(Key::Ctrl('l')) => { cli_history.clear(); },
				Event::Key(Key::Backspace) => { cli_text.pop(); },
				_ => {},
			}
		}

		if cli_history.len() > 1024
		{
			let len = cli_history.len();
			let truncated: String = cli_history.chars()
				.skip(len - 1024)
				.skip_while(|ch| *ch != '\n')
				.skip(1)
				.collect();
			cli_history.clear();
			cli_history.push_str(&truncated);
		}

		terminal.draw(|mut f|
		{
			let chunks = Layout::default()
				.direction(Direction::Vertical)
				.margin(0)
				.constraints(
					[
						Constraint::Length(1),
						Constraint::Length(1),
						Constraint::Percentage(80),
						Constraint::Length(1),
					]
					.as_ref(),
				)
				.split(f.size());

			Gauge::default()
				.style(Style::default().fg(Color::Red))
				.ratio((current_health as f64 / hero.health as f64).min(1.0).max(0.0))
				.label(&format!("Health: {}/{}", current_health, hero.health))
				.render(&mut f, chunks[0]);

			Gauge::default()
				.style(Style::default().fg(Color::Blue))
				.ratio((current_astral as f64 / hero.astral as f64).min(1.0).max(0.0))
				.label(&format!("Astral: {}/{}", current_astral, hero.astral))
				.render(&mut f, chunks[1]);

			Paragraph::new([Text::raw(&cli_history)].into_iter())
				.block(Block::default().borders(Borders::NONE))
				.wrap(true)
				.render(&mut f, chunks[2]);

			Paragraph::new([Text::raw(&cli_text)].into_iter())
				.block(Block::default().borders(Borders::ALL))
				.wrap(true)
				.render(&mut f, chunks[3]);
		})?;

		std::thread::sleep(sleep_duration);
	}

	Ok(())
}

