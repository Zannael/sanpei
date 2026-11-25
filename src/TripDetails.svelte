<script lang="ts">
 import {invoke} from '@tauri-apps/api/core';
 export let tripId:number;
 export let initialOutcome:string;
 export let onChanged:()=>void;
 type Rod={id:number,name:string}; type Interval={id:number,rod_id:number,starts_at:string,ends_at:string,rig:string,bait:string}; type Catch={id:number,occurred_at:string,species:string,uncertain:boolean,length_cm:number|null,weight_g:number|null,weight_estimated:boolean,disposition:string}; type Details={rods:Rod[],intervals:Interval[],catches:Catch[]};
 let data:Details={rods:[],intervals:[],catches:[]}; let error=''; let rodName=''; let rodId=0; let start='',end='',rig='',bait=''; let species='',caughtAt='',length='',weight='',uncertain=false,weightEstimated=false,disposition='unknown'; let outcome='incomplete';
 let loadedId=0;
 $: if(tripId!==loadedId){loadedId=tripId;outcome=initialOutcome;load()}
 async function load(){try{data=await invoke('trip_details',{tripId}); error=''; if(!data.rods.some(x=>x.id===rodId))rodId=data.rods[0]?.id || 0;}catch{error='Impossibile caricare i dettagli.'}}
 async function action(command:string,args:Record<string,unknown>){try{await invoke(command,args); await load();onChanged();}catch(e){error=typeof e==='object'&&e!==null&&'message' in e?String(e.message):'Impossibile salvare i dati.';}}
 function utc(s:string){return new Date(s).toISOString().replace('.000Z','Z')}
 function display(s:string){return new Intl.DateTimeFormat('it-IT',{dateStyle:'short',timeStyle:'short',timeZone:'Europe/Rome'}).format(new Date(s))}
 async function saveRod(){await action('add_rod',{tripId,name:rodName});rodName=''}
 async function saveInterval(){if(!start||!end||!rodId){error='Seleziona canna, inizio e fine.';return} await action('add_interval',{interval:{rod_id:Number(rodId),starts_at:utc(start),ends_at:utc(end),rig,bait}})}
 async function saveCatch(){if(!caughtAt||!species.trim()){error='Inserisci specie e ora.';return}await action('add_catch',{catch:{trip_id:tripId,occurred_at:utc(caughtAt),species,uncertain,length_cm:length?Number(length):null,weight_g:weight?Number(weight):null,weight_estimated:weightEstimated,disposition}})}
</script>
<div class="trip-details">
<h3>Dettagli dell’uscita</h3>
{#if error}<p class="error" role="alert">{error}</p>{/if}
<div class="detail-columns"><section><h4>Canne e intervalli</h4><form on:submit|preventDefault={saveRod} class="compact-form"><label>Nome della canna<input bind:value={rodName} required placeholder="Canna 1"></label><button type="submit">Aggiungi canna</button></form>
{#if data.rods.length}<form on:submit|preventDefault={saveInterval} class="compact-form"><label>Canna<select bind:value={rodId}>{#each data.rods as rod}<option value={rod.id}>{rod.name}</option>{/each}</select></label><label>Inizio<input type="datetime-local" bind:value={start} required></label><label>Fine<input type="datetime-local" bind:value={end} required></label><label>Montatura<input bind:value={rig}></label><label>Esca<input bind:value={bait}></label><button type="submit">Aggiungi intervallo</button></form>{/if}
{#each data.intervals as interval}<div class="detail-row"><strong>{data.rods.find(r=>r.id===interval.rod_id)?.name}</strong><br>{display(interval.starts_at)} – {display(interval.ends_at)}<br>{interval.rig || 'Montatura non indicata'} · {interval.bait || 'Esca non indicata'}</div>{/each}</section>
<section><h4>Catture</h4><form on:submit|preventDefault={saveCatch} class="compact-form"><label>Specie<input bind:value={species} required></label><label><input type="checkbox" bind:checked={uncertain}> Identificazione incerta</label><label>Ora<input type="datetime-local" bind:value={caughtAt} required></label><label>Lunghezza in cm <span>facoltativo</span><input type="number" min="0.1" step="0.1" bind:value={length}></label><label>Peso in g <span>facoltativo</span><input type="number" min="0.1" step="0.1" bind:value={weight}></label><label><input type="checkbox" bind:checked={weightEstimated}> Peso stimato</label><label>Esito<select bind:value={disposition}><option value="unknown">Non indicato</option><option value="released">Rilasciato</option><option value="kept">Trattenuto</option></select></label><button type="submit">Aggiungi cattura</button></form>{#each data.catches as item}<div class="detail-row"><strong>{item.species}{item.uncertain ? ' (incerta)' : ''}</strong><br>{display(item.occurred_at)} · {item.disposition==='released'?'rilasciato':item.disposition==='kept'?'trattenuto':'esito non indicato'}{#if item.length_cm} · {item.length_cm} cm{/if}{#if item.weight_g} · {item.weight_g} g{item.weight_estimated?' stimati':''}{/if}</div>{/each}</section></div>
<form class="outcome-form" on:submit|preventDefault={() => action('set_outcome',{tripId,outcome})}><label>Stato dell’uscita <select bind:value={outcome}><option value="incomplete">Registrazione incompleta</option><option value="zero">Nessuna cattura</option><option value="catches">Catture registrate</option></select></label><button type="submit">Aggiorna esito</button></form>
</div>
