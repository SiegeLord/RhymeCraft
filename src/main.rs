#[macro_use]
extern crate allegro;
extern crate allegro_audio;
extern crate allegro_dialog;
extern crate allegro_primitives;
extern crate allegro_font;
extern crate allegro_image;
extern crate fern;
#[macro_use]
extern crate log;
extern crate time;
extern crate rand;
#[macro_use]
extern crate slr_config;

#[macro_use]
mod game_state;
mod debug_draw;
mod engine;
mod parent;
mod map;
mod animation;
mod util;
mod unit;
mod cursor;
mod path;
mod effect;
mod game_ui;
mod turn;
mod craft;
mod announce;
mod main_menu;

use debug_draw::*;
use engine::world::*;
use game_state::*;
use parent::*;
use map::*;
use unit::*;
use cursor::*;
use path::*;
use effect::*;
use game_ui::*;
use turn::*;
use announce::*;
use main_menu::*;

use std::fs::OpenOptions;

use allegro::*;
use allegro_dialog::*;
use allegro_primitives::*;
use allegro_font::*;
use allegro_image::*;

fn game()
{
	let mut logfile_options = OpenOptions::new();
	logfile_options.write(true).create(true).truncate(true);
	let logger_config = fern::DispatchConfig
	{
		format: Box::new(|msg: &str, level: &log::LogLevel, loc: &log::LogLocation| {
			format!("{} {} {: <24}   {}", time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap(),
				level, format!("{}:{}", loc.file(), loc.line()), msg)
		}),
		output: vec![fern::OutputConfig::stderr(), fern::OutputConfig::file_with_options("game.log", &logfile_options),],
		level: log::LogLevelFilter::Trace,
	};
	fern::init_global_logger(logger_config, log::LogLevelFilter::Trace).unwrap();

	info!("It's time to play!");
	
	let mut core = Core::init().unwrap();
	core.install_keyboard().unwrap();
	core.install_mouse().unwrap();
	
	let prim = PrimitivesAddon::init(&core).unwrap();
	let _image = ImageAddon::init(&core).unwrap();
	let font = FontAddon::init(&core).unwrap();
	//~ let ttf = TtfAddon::init(&font).unwrap();
	//~ core.set_new_display_flags(RESIZABLE);
	core.set_new_display_flags(FULLSCREEN_WINDOW);
	let disp = Display::new(&core, 1280, 960).unwrap();
	disp.set_window_title("RhymeCraft");
	let buffer = Bitmap::new(&core, disp.get_width() / SCALE as i32, disp.get_height() / SCALE as i32).unwrap();

	let timer = Timer::new(&core, DT as f64).unwrap();
	let mut q = EventQueue::new(&core).unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source());
	q.register_event_source(core.get_mouse_event_source());
	q.register_event_source(timer.get_event_source());

	let state = GameState::new(core, prim, disp, buffer, font);
	let mut world = World::<Object, GameState>::new(state);
	
	world.add_input_behavior(Box::new(UIInput));
	world.add_input_behavior(Box::new(CraftInput));
	world.add_input_behavior(Box::new(CursorInput));
	world.add_input_behavior(Box::new(SelectableInput));
	world.add_input_behavior(Box::new(PathableInput));
	world.add_input_behavior(Box::new(CameraInput));
	world.add_input_behavior(Box::new(MainMenuInput));
	
	world.add_logic_behavior(Box::new(UnitLogic));
	world.add_logic_behavior(Box::new(SolidLogic));
	world.add_logic_behavior(Box::new(OrdersLogic));
	world.add_logic_behavior(Box::new(EffectLogic));
	world.add_logic_behavior(Box::new(HealthDeathLogic));
	world.add_logic_behavior(Box::new(CameraLogic));
	world.add_logic_behavior(Box::new(TurnLogic));
	world.add_logic_behavior(Box::new(AnnounceLogic));
	world.add_logic_behavior(Box::new(InventoryLogic));
	world.add_logic_behavior(Box::new(VictoryLogic));
	// Must be last.
	world.add_logic_behavior(Box::new(ParentLogic));

	world.add_draw_behavior(Box::new(CameraDraw));
	world.add_draw_behavior(Box::new(MapDraw));
	world.add_draw_behavior(Box::new(DebugDraw));
	world.add_draw_behavior(Box::new(MagicCircleDraw));
	world.add_draw_behavior(Box::new(UnitDraw));
	world.add_draw_behavior(Box::new(EffectDraw));
	world.add_draw_behavior(Box::new(SelectableDraw));
	world.add_draw_behavior(Box::new(UnitDrawPathable));
	world.add_draw_behavior(Box::new(PathableDraw));
	world.add_draw_behavior(Box::new(CursorDraw));
	world.add_draw_behavior(Box::new(IdentityTransformDraw));
	world.add_draw_behavior(Box::new(SelectedDraw));
	world.add_draw_behavior(Box::new(AnnounceDraw));
	world.add_draw_behavior(Box::new(UIDraw));
	world.add_draw_behavior(Box::new(MainMenuDraw));
	
	//~ let map = create_map("data/map0.cfg", &mut world.state);
	//~ world.state.add_object(map);
	let menu = create_main_menu(&mut world.state);
	world.state.add_object(menu);
	
	timer.start();
	let offset = world.state.core.get_time();
	'exit: loop
	{
		for event in &mut q
		{
			world.state.key_down = None;
			world.state.key_up = None;
			world.state.mouse_button_down = None;
			world.state.mouse_x = None;
			world.state.mouse_y = None;
			match event
			{
				DisplayClose{..} =>
				{
					break 'exit;
				},
				DisplayResize{..} =>
				{
					world.state.disp.acknowledge_resize().ok();
					world.state.buffer = Bitmap::new(&world.state.core, world.state.disp.get_width() / SCALE as i32, world.state.disp.get_height() / SCALE as i32).unwrap();
					info!("New buffer size: {} {}", world.state.buffer.get_width(), world.state.buffer.get_height());
				},
				KeyDown{keycode: k, ..} =>
				{
					world.state.key_down = Some(k);
					world.input();
				},
				KeyUp{keycode: k, ..} =>
				{
					world.state.key_up = Some(k);
					world.input();
				},
				MouseAxes{x, y, ..} =>
				{
					world.state.mouse_x = Some(x);
					world.state.mouse_y = Some(y);
					world.input();
				},
				MouseLeaveDisplay{..} =>
				{
					world.input();
				},
				MouseButtonDown{button, x, y, ..} =>
				{
					world.state.mouse_button_down = Some(button);
					world.state.mouse_x = Some(x);
					world.state.mouse_y = Some(y);
					world.input();
				},
				TimerTick{count, ..} =>
				{
					if !world.state.paused
					{
						world.state.time = count as f64 * DT as f64;
						world.logic();
					}
					if world.state.quit
					{
						break 'exit;
					}
				},
				_ => ()
			}
		}

		let cur_time = world.state.core.get_time();
		world.state.draw_interp = (cur_time - offset - world.state.time) as f32 / DT;
		world.state.core.set_target_bitmap(&world.state.buffer);
		world.state.core.clear_to_color(Color::from_rgb(0, 0, 0));
		world.draw();
		world.state.core.set_target_bitmap(world.state.disp.get_backbuffer());
		world.state.core.clear_to_color(Color::from_rgb(0, 0, 0));
		world.state.core.draw_bitmap(&world.state.buffer, 0.0, 0.0, BitmapDrawingFlags::zero());
		world.state.core.draw_scaled_bitmap(&world.state.buffer,
			0.0, 0.0, world.state.buffer.get_width() as f32, world.state.buffer.get_height() as f32,
			0.0, 0.0, world.state.disp.get_width() as f32, world.state.disp.get_height() as f32,
			BitmapDrawingFlags::zero());
		world.state.core.flip_display();
	}

	info!("All's well that ends well.");
}

allegro_main!
{
	use std::panic::catch_unwind;

	match catch_unwind(game)
	{
		Err(e) =>
		{
			let err: String = e.downcast_ref::<&'static str>().map(|&e| { e.to_owned()}).or_else(||
			{
				e.downcast_ref::<String>().map(|e| e.clone())
			}).unwrap_or("Unknown error!".to_owned());

			show_native_message_box(None, "Error!", "An error has occurred! See game.log for more info.", &err, Some("You make me sad."), MESSAGEBOX_ERROR);
		}
		Ok(_) => ()
	}
}
