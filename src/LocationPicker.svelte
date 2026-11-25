<script lang="ts">
 import {onMount} from 'svelte';
 import maplibregl from 'maplibre-gl';
 import 'maplibre-gl/dist/maplibre-gl.css';
 export let onPick:(latitude:number,longitude:number)=>void;
 let element:HTMLDivElement;
 onMount(()=>{
  const map=new maplibregl.Map({container:element,attributionControl:false,style:{version:8,sources:{regions:{type:'geojson',data:'./data/regions.geojson'}},layers:[{id:'sea',type:'background',paint:{'background-color':'#dcebe6'}},{id:'land',type:'fill',source:'regions',paint:{'fill-color':'#f4f4df','fill-outline-color':'#9fb8aa'}}]},center:[12.2,41.9],zoom:7});
  map.addControl(new maplibregl.NavigationControl({showCompass:false}),'top-right');
  let marker:maplibregl.Marker|null=null;
  map.on('click',e=>{marker?.remove();marker=new maplibregl.Marker({color:'#285e57'}).setLngLat(e.lngLat).addTo(map);onPick(e.lngLat.lat,e.lngLat.lng)});
  return ()=>map.remove();
 });
</script>
<div class="location-map"><div bind:this={element} class="map-canvas" aria-label="Mappa regionale per una posizione personale"></div><span>Confini: ISTAT / Openpolis, CC BY 4.0</span></div>
