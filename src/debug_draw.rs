// Copyright 2015 SiegeLord
//
// See LICENSE for terms.

use allegro::*;

simple_behavior!
{
	DebugDraw[obj.debug_draw && obj.has_pos] |obj, state|
	{
		state.prim.draw_circle(obj.x, obj.y, 10.0, Color::from_rgb(64, 255, 255), 4.0);
		//~ state.core.draw_bitmap(&state.dollar, obj.x, obj.y, BitmapDrawingFlags::zero());
	}
}

