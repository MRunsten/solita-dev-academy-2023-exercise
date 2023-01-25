<script lang="ts">
    import type { PageData } from './$types';

    export let data: PageData;
</script>

<h1>Bicycle journeys: page {data.page} </h1>

{#if data.page > 0}
    <a href="/journeys/{data.page - 1}">Previous page</a>
{/if}

{#if (data.journeys.length === data.max_per_page)}
    <a href="/journeys/{data.page + 1}">Next page</a>
{/if}

<table>
    <thead>
        <tr>
            <td>Departure date</td>
            <td>Return date</td>
            <td>Departure station</td>
            <td>Return station</td>
            <td>Distance</td>
            <td>Duration</td>
        </tr>
    </thead>

    {#each data.journeys as journey}
    <tr>
        <td>{journey.departure_date}</td>
        <td>{journey.return_date.toLocaleString()}</td>
        <td>{journey.departure_station.station_id} | {journey.departure_station.name.finnish} | {journey.departure_station.name.swedish} | {journey.departure_station.name.english}</td>
        <td>{journey.return_station.station_id} | {journey.return_station.name.finnish} | {journey.return_station.name.swedish} | {journey.return_station.name.english}</td>
        <td>{journey.distance_kilometers} km</td>
        <td>{journey.duration_minutes} minutes</td>
    </tr>
    {/each}
</table>

{#if data.page > 0}
    <a href="/journeys/{data.page - 1}">Previous page</a>
{/if}

{#if (data.journeys.length === data.max_per_page)}
    <a href="/journeys/{data.page + 1}">Next page</a>
{/if}