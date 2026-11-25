<script lang="ts">
  import { onMount } from 'svelte';
  import maplibregl from 'maplibre-gl';
  import 'maplibre-gl/dist/maplibre-gl.css';
  let element: HTMLDivElement;
  export let onSelect: (region: string) => void = () => {};
  let selected = 'Lazio';
  let map: maplibregl.Map;
  let failed = false;
  const regions = ['Abruzzo','Basilicata','Calabria','Campania','Emilia-Romagna','Friuli-Venezia Giulia','Lazio','Liguria','Lombardia','Marche','Molise','Piemonte','Puglia','Sardegna','Sicilia','Toscana','Trentino-Alto Adige/Südtirol','Umbria','Valle d’Aosta/Vallée d’Aoste','Veneto'];
  function choose(name: string) {
    selected = name;
    onSelect(name);
    if (!map?.getLayer('selected-region')) return;
    map.setFilter('selected-region', ['==', ['get','reg_name'], name]);
    if (name === 'Lazio') map.fitBounds([[11.44,41.15],[14.1,42.88]], {padding:35,duration:matchMedia('(prefers-reduced-motion: reduce)').matches?0:300});
  }
  function allItaly() { map?.fitBounds([[6.5,35.4],[18.7,47.2]], {padding:25,duration:matchMedia('(prefers-reduced-motion: reduce)').matches?0:300}); }
  onMount(() => {
    map = new maplibregl.Map({container:element, attributionControl:false, style:{version:8,sources:{regions:{type:'geojson',data:'./data/regions.geojson'}},layers:[{id:'sea',type:'background',paint:{'background-color':'#dcebe6'}},{id:'regions',type:'fill',source:'regions',paint:{'fill-color':'#f4f4df','fill-outline-color':'#a0b9a8'}},{id:'selected-region',type:'fill',source:'regions',filter:['==',['get','reg_name'],'Lazio'],paint:{'fill-color':'#7daf9b','fill-opacity':0.9}},{id:'borders',type:'line',source:'regions',paint:{'line-color':'#9fb8aa','line-width':0.7}}]},center:[12.6,42.6],zoom:4.2});
    map.addControl(new maplibregl.NavigationControl({showCompass:false}), 'top-right');
    map.on('click','regions', e => { const name=e.features?.[0]?.properties?.reg_name; if (typeof name === 'string') choose(name); });
    map.on('error', () => failed=true);
    return () => map.remove();
  });
</script>
<div class="map-shell">
 <div bind:this={element} class="map-canvas" aria-label="Mappa delle regioni italiane"></div>
 <div class="map-actions"><label>Regione <select bind:value={selected} on:change={() => choose(selected)}>{#each regions as region}<option>{region}</option>{/each}</select></label><button on:click={allItaly}>Mostra Italia</button></div>
 <div class="map-notice"><strong>{selected}</strong><span>{selected === 'Lazio' ? 'Ricerca disponibile per settori limitati; punti e accessi in verifica.' : 'Ricerca non ancora disponibile.'}</span></div>
 {#if failed}<p class="map-error">Impossibile caricare i confini offline.</p>{/if}
 <div class="map-credit">Confini: ISTAT / Openpolis, CC BY 4.0 · nessuna indicazione di accessibilità</div>
</div>
