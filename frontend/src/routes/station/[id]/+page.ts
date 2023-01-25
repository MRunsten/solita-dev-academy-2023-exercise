import { z } from "zod";

import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

import { PUBLIC_API_ADDRESS } from '$env/static/public'

const StationView = z.object({
    station_id: z.number().transform(v => v.toString().padStart(3, '0')),

    name: z.object({
        finnish: z.string(),
        swedish: z.string(),
        english: z.string(),
    }),

    city: z.object({
        finnish: z.string(),
        swedish: z.string(),
    }),

    address: z.object({
        finnish: z.string(),
        swedish: z.string(),
    }),

    total_ending_journeys: z.number(),
    total_starting_journeys: z.number(),
})

export const load = (async ({ fetch, params }) => {
    
    const res = await fetch(`${PUBLIC_API_ADDRESS}/station/${params.id}`);
    const station_json = await res.json();

    if(!res.ok) {
        throw error(res.status, {
            message: station_json,
        });
    }

    let parsed_station_view = StationView.safeParse(station_json);

    if (!parsed_station_view.success) {
        throw error(500, {
            message: parsed_station_view.error.message,
        });
    }

    return parsed_station_view.data;
}) satisfies PageLoad;
