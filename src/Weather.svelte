<script lang="ts">
 import {invoke} from '@tauri-apps/api/core';
 import ForecastView from './ForecastView.svelte';
 type Dataset={latitude:number,longitude:number,timezone:string,hourly:Record<string,(number|string|null)[]>,hourly_units:Record<string,string>};
 type Forecast={weather:Dataset|null,marine:Dataset|null,retrieved_at:string};
 export let onForecast:(value:Record<string,unknown>|null)=>void=()=>{};
 let latitude='',longitude='',loading=false,error='',fromCache=false; let result:Forecast|null=null; let shownCoordinates:{latitude:number,longitude:number}|null=null;
 async function fetchForecast(){
  const lat=Number(latitude),lon=Number(longitude);
  if(!latitude.trim()||!longitude.trim()||!Number.isFinite(lat)||!Number.isFinite(lon)||lat < -90||lat > 90||lon < -180||lon > 180){error='Inserisci latitudine e longitudine valide.';return}
  error='';loading=true;
  try{result=await invoke('fetch_forecast',{latitude:lat,longitude:lon});fromCache=false;shownCoordinates={latitude:lat,longitude:lon};onForecast({source:'Open-Meteo',requested_latitude:lat,requested_longitude:lon,...result})}catch(e){error=typeof e==='object'&&e!==null&&'message' in e?String(e.message):'Impossibile ottenere le previsioni.'}finally{loading=false}
 }
 async function loadCached(){ if(!latitude.trim()||!longitude.trim()){error='Inserisci le coordinate.';return} try{result=await invoke('cached_forecast',{latitude:Number(latitude),longitude:Number(longitude)});fromCache=true;shownCoordinates=result ? {latitude:Number(latitude),longitude:Number(longitude)} : null;onForecast(result ? {source:'Open-Meteo',requested_latitude:Number(latitude),requested_longitude:Number(longitude),...result} : null);error=result?'':'Nessuna previsione salvata per queste coordinate.'}catch{error='Impossibile leggere la cache.'} }

</script>
<div class="weather"><h2>Meteo e mare</h2><p>Le coordinate vengono inviate a Open-Meteo quando richiedi le previsioni. Il dato descrive una cella del modello, non le condizioni esatte sulla riva.</p><form on:submit|preventDefault={fetchForecast}><label>Latitudine<input type="number" step="any" min="-90" max="90" bind:value={latitude} required></label><label>Longitudine<input type="number" step="any" min="-180" max="180" bind:value={longitude} required></label><button class="primary" disabled={loading}>{loading?'Caricamento…':'Recupera previsioni'}</button><button type="button" on:click={loadCached}>Leggi ultima previsione salvata</button></form>{#if error}<p class="error" role="alert">{error}</p>{/if}
{#if result}<p class="source">{fromCache ? 'Previsione salvata in precedenza: può essere superata.' : 'La risposta è salvata localmente.'}</p><ForecastView snapshot={{source:'Open-Meteo',requested_latitude:shownCoordinates?.latitude,requested_longitude:shownCoordinates?.longitude,...result}} />{/if}</div>
