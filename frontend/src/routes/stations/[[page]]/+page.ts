import { z } from "zod";
import { error } from '@sveltejs/kit';
import { PUBLIC_API_ADDRESS } from '$env/static/public'

import type { PageLoad } from './$types';

const MAX_STATIONS_PER_PAGE = 50;

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

export const load = (async ({ fetch, params }) => {

    let page = 0;

    if (params.page) {
        let page_parse_result = z.coerce.number().safeParse(params.page);
        page = (page_parse_result.success) ? page_parse_result.data : 0;
    }

    const res = await fetch(`${PUBLIC_API_ADDRESS}/stations?page=${page}`);
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

    return {
        page: page, 
        max_per_page: MAX_STATIONS_PER_PAGE, 
        stations: parsed_station_list_view.data
    };

}) satisfies PageLoad;
