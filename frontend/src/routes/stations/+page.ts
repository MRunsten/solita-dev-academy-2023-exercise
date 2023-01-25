import { z } from "zod";

import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

import { PUBLIC_API_ADDRESS } from '$env/static/public'

const StationListView = z.array(z.object({
    station_id: z.number().transform(v => v.toString().padStart(3, '0')),

    name: z.object({
        finnish: z.string(),
        swedish: z.string(),
        english: z.string(),
    }),

    city_name: z.object({
        finnish: z.string(),
        swedish: z.string(),
    }),

    operator_name: z.string(),

    capacity: z.number(),
}));

export const load = (async ({ fetch }) => {
    
    const res = await fetch(`${PUBLIC_API_ADDRESS}/station/list`);
    const station_json = await res.json();

    if(!res.ok) {
        throw error(res.status, {
            message: station_json,
        });
    }

    let parsed_station_list_view = StationListView.safeParse(station_json);

    if (!parsed_station_list_view.success) {
        throw error(500, {
            message: parsed_station_list_view.error.message,
        });
    }

    return {stations: parsed_station_list_view.data};
}) satisfies PageLoad;
