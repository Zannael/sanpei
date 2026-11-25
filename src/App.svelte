<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import ItalyMap from './ItalyMap.svelte';
  import TripDetails from './TripDetails.svelte';
  import Weather from './Weather.svelte';
  import rigContent from '../content/lazio-rigs.v1.json';
  type Trip = { id:number; title:string; starts_at:string; location:string; notes:string; outcome:string };
  type Section = 'esplora' | 'pianifica' | 'diario' | 'dati';
  let section: Section = 'esplora';
  let trips: Trip[] = [];
  let title = '', date = '', location = '', notes = '', error = '', notice = '';
  let selectedRig = 'A';
  let selectedTrip: number | null = null;
  let backupMessage = '';
  async function makeBackup() { try { const done = await invoke<boolean>('backup_database'); backupMessage = done ? 'Copia del database creata e verificata.' : 'Salvataggio annullato.'; } catch (e) { backupMessage = typeof e === 'object' && e !== null && 'message' in e ? String(e.message) : 'Impossibile creare la copia.'; } }
  const rigs = rigContent.rigs;
  $: rig = rigs.find(r => r.id === selectedRig)!;
  async function loadTrips() { try { trips = await invoke<Trip[]>('list_trips'); error=''; } catch { error='Diario non disponibile. Avvia Sanpei come applicazione desktop.'; } }
  async function saveTrip() {
    if (!title.trim() || !date) { error='Inserisci un titolo e una data.'; return; }
    try { const starts_at = new Date(`${date}T12:00:00`).toISOString(); await invoke('create_trip', { trip: {title, starts_at, location, notes} }); title=''; date=''; location=''; notes=''; notice='Uscita salvata sul dispositivo.'; await loadTrips(); } catch { error='Impossibile salvare l’uscita.'; }
  }
  function displayDate(value:string) { const d = new Date(value); return Number.isNaN(d.getTime()) ? value : new Intl.DateTimeFormat('it-IT',{dateStyle:'long',timeZone:'Europe/Rome'}).format(d); }
  onMount(() => { if ('__TAURI_INTERNALS__' in window) loadTrips(); });
</script>

<svelte:head><title>Sanpei — diario di pesca</title><meta name="description" content="Pianifica e registra le tue uscite di pesca."></svelte:head>
<div class="app-shell">
  <aside class="sidebar">
    <div class="brand"><span class="brand-mark" aria-hidden="true">〰</span><span>sanpei<small>Diario di pesca</small></span></div>
    <nav aria-label="Navigazione principale">
      <button class:active={section==='esplora'} on:click={() => section='esplora'}><span aria-hidden="true">⌖</span> Esplora</button>
      <button class:active={section==='pianifica'} on:click={() => section='pianifica'}><span aria-hidden="true">◷</span> Pianifica</button>
      <button class:active={section==='diario'} on:click={() => section='diario'}><span aria-hidden="true">▤</span> Diario</button>
      <button class:active={section==='dati'} on:click={() => section='dati'}><span aria-hidden="true">⚙</span> Dati e impostazioni</button>
    </nav>
    <div class="side-note"><span class="dot"></span><div><strong>La tua base, sul dispositivo</strong><br><span>Il diario viene salvato in locale.</span></div></div>
  </aside>
  <main>
    <header class="topbar"><span>Sanpei <span class="slash">/</span> {section === 'dati' ? 'Dati e impostazioni' : section.charAt(0).toUpperCase()+section.slice(1)}</span><span class="top-status"><span class="dot"></span> Progetto in sviluppo</span></header>
    {#if section === 'esplora'}
      <section class="page explore">
        <div class="page-heading"><div><p class="overline">La costa, con criterio</p><h1>Esplora il Lazio</h1><p class="lead">Luoghi, tecniche e osservazioni per preparare la prossima uscita.</p></div><span class="region-label">Italia <span>›</span> Lazio</span></div>
        <div class="explore-grid">
          <ItalyMap />
          <div class="research-panel"><div class="section-kicker">La ricerca</div><h2>Prima l’habitat,<br>poi la distanza.</h2><p>Una buona presentazione dell’esca e una postazione accessibile contano più di una regola universale sulla luna o sulla distanza di lancio.</p><div class="research-rule"></div><h3>Da verificare sul posto</h3><ul><li>Accesso e ordinanze del tratto</li><li>Onde, corrente e tenuta del piombo</li><li>Avvisi ambientali aggiornati</li></ul><p class="source">Sintesi operativa della ricerca Lazio, versione 1.0. Le condizioni locali e la sicurezza alimentare richiedono verifiche specifiche.</p></div>
        </div>
        <div class="bottom-callout"><div><span class="small-label">Il tuo quaderno</span><h2>Ogni uscita aggiunge contesto.</h2><p>Anche quando non si cattura nulla, un appunto può essere utile la prossima volta.</p></div><button class="primary" on:click={() => section='diario'}>Apri il diario <span>→</span></button></div>
      </section>
    {:else if section === 'pianifica'}
      <section class="page"><div class="page-heading"><div><p class="overline">La preparazione</p><h1>Montature e scelte</h1><p class="lead">Sei configurazioni operative da adattare alle condizioni reali.</p></div></div><div class="info-banner">Il pianificatore di scenari è in preparazione. Queste montature sono proposte pratiche, non risultati sperimentali.</div><Weather /><div class="rig-layout"><div class="rig-list" aria-label="Montature">{#each rigs as item}<button class:selected={selectedRig===item.id} on:click={() => selectedRig=item.id}><span class="rig-id">{item.id}</span><span><strong>{item.name}</strong><small>{item.use}</small></span><span class="rig-arrow">›</span></button>{/each}</div><article class="rig-detail"><div class="rig-badge">Montatura {rig.id}</div><h2>{rig.name}</h2><p class="rig-use">{rig.use}</p><h3>Configurazione iniziale</h3><p>{rig.detail}</p><h3>Da controllare</h3><p>{rig.caution}</p><p class="source">Ricerca Lazio, revisione 24 novembre 2026. Consulta la scheda completa prima di costruire la montatura.</p></article></div></section>
    {:else if section === 'diario'}
      <section class="page"><div class="page-heading"><div><p class="overline">Il tuo quaderno</p><h1>Diario delle uscite</h1><p class="lead">Conserva luogo, data e appunti sul tuo dispositivo.</p></div></div><div class="journal-layout"><form class="journal-form" on:submit|preventDefault={saveTrip}><h2>Nuova uscita</h2><p>Inizia con poche informazioni, poi aggiungi canne, intervalli e catture.</p><label>Titolo<input bind:value={title} required placeholder="Per esempio: uscita serale"></label><label>Data<input type="date" bind:value={date} required></label><label>Luogo <span>facoltativo</span><input bind:value={location} placeholder="Località o tratto"></label><label>Appunti <span>facoltativo</span><textarea bind:value={notes} rows="3" placeholder="Condizioni, idee o cose da ricordare"></textarea></label>{#if error}<p class="error" role="alert">{error}</p>{/if}{#if notice}<p class="notice" role="status">{notice}</p>{/if}<button class="primary" type="submit">Salva uscita</button></form><div class="entries"><div class="entries-heading"><h2>Le tue uscite</h2><span>{trips.length} {trips.length===1?'uscita':'uscite'}</span></div>{#if trips.length===0}<div class="empty"><div class="empty-icon">⌁</div><h3>Il diario è ancora vuoto</h3><p>La prima uscita inizia con un luogo e una data. Puoi registrarla anche al ritorno.</p></div>{:else}{#each trips as trip}<article class="entry"><div><span>{displayDate(trip.starts_at)}</span><h3>{trip.title}</h3><p>{trip.location || 'Luogo non indicato'}</p>{#if trip.notes}<p>{trip.notes}</p>{/if}</div><button class="outcome" on:click={() => selectedTrip = selectedTrip === trip.id ? null : trip.id}>{trip.outcome === 'zero' ? 'Nessuna cattura' : trip.outcome === 'catches' ? 'Catture' : 'Incompleta'} · Dettagli</button></article>{#if selectedTrip === trip.id}<TripDetails tripId={trip.id} initialOutcome={trip.outcome} onChanged={loadTrips} />{/if}{/each}{/if}</div></div></section>
    {:else}
      <section class="page narrow"><div class="page-heading"><div><p class="overline">Il tuo dispositivo</p><h1>Dati e impostazioni</h1><p class="lead">Le informazioni del diario sono conservate in locale.</p></div></div><div class="settings-block"><h2>Conservazione</h2><p>Sanpei usa la cartella dati dell’applicazione sul computer. Puoi creare una copia del database. Il ripristino guidato, gli allegati e le esportazioni non sono ancora disponibili.</p><button class="primary" on:click={makeBackup}>Salva copia del database</button>{#if backupMessage}<p role="status">{backupMessage}</p>{/if}</div><div class="settings-block"><h2>Fonti e contenuti</h2><p>La ricerca iniziale riguarda la pesca da riva nel Lazio. Informazioni ambientali, accessi e norme devono essere verificati prima dell’uscita.</p><p>Le schede riportano sintesi della ricerca, ancora da completare e verificare rispetto alle fonti originali.</p></div></section>
    {/if}
  </main>
</div>
