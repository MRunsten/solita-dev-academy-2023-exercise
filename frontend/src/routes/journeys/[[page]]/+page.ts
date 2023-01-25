import { z } from "zod";

import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

import { PUBLIC_API_ADDRESS } from '$env/static/public'

const MAX_JOURNEYS_PER_PAGE = 100;

const JourneyListViewStation = z.object({
    station_id: z.number().transform(v => v.toString().padStart(3, '0')),
    name: z.object({
        finnish: z.string(),
        swedish: z.string(),
        english: z.string(),
    })
});

const JourneyListDate = z.string().datetime().transform(v => {
    let date = new Date(v);
    
    // Data has no timezone information available
    // => format date without taking timezone into account.
    return date.toLocaleString(undefined, { timeZone: 'UTC'});    
})

const JourneyListView = z.array(z.object({
    departure_date: JourneyListDate,
    return_date: JourneyListDate,

    departure_station: JourneyListViewStation,
    return_station: JourneyListViewStation,

    distance_kilometers: z.number().transform(v => v.toFixed(3)),
    duration_minutes: z.number().transform(v => Math.round(v)),
}));

export const load = (async ({ fetch, params }) => {
    const page_parse_result = z.coerce.number().safeParse(params.page);

    console.log(page_parse_result);

    const page = (page_parse_result.success) ? page_parse_result.data : 0;
    
    const res = await fetch(`${PUBLIC_API_ADDRESS}/journey/list?page=${page}`);
    const journey_json = await res.json();

    if(!res.ok) {
        throw error(res.status, {
            message: journey_json,
        });
    }

    let parsed_journey_list_view = JourneyListView.safeParse(journey_json);

    if (!parsed_journey_list_view.success) {
        throw error(500, {
            message: parsed_journey_list_view.error.message,
        });
    }

    if (parsed_journey_list_view.data.length === 0) {
        throw error(404, {
            message: "Page not found",
        });
    }

    return {
        page: page, 
        max_per_page: MAX_JOURNEYS_PER_PAGE, 
        journeys: parsed_journey_list_view.data
    };

}) satisfies PageLoad;
