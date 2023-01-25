<script lang="ts">
    import type { PageData } from './$types';

    export let data: PageData;
</script>

<h1>Stations</h1>

<table>
    <thead>
        <tr>
            <td>Station ID</td>
            <td>Name</td>
            <td>City</td>
            <td>Operator</td>
            <td>Bicycle capacity</td>
            <td>View</td>
        </tr>
    </thead>

    {#each data.stations as station}
    <tr>
        <td>{station.station_id}</td>
        <td>{station.name.finnish} | {station.name.swedish} | {station.name.english} </td>
        <td>{station.city_name.finnish} | {station.city_name.swedish}</td>
        <td>{station.operator_name}</td>
        <td>{station.capacity}</td>
        <td><a href="/station/{station.station_id}">View</a></td>
    </tr>
    {/each}
</table>