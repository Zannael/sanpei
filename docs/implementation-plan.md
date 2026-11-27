# Sanpei — specifica e criteri di accettazione

## Scelte confermate

App locale in italiano, Linux prima, Tauri 2 + Svelte/TypeScript/Vite + Rust + SQLite/SQLx + MapLibre. UTC in persistenza, Europe/Rome in presentazione, unità metriche. Quattro aree: Esplora, Pianifica, Diario, Dati/impostazioni. Nessun server obbligatorio, modello predittivo o sincronizzazione nella v1.

Applicare `architecture.md` a tutte le tappe. I pacchetti cambiano parametri e regole supportate; nuove capacità richiedono codice esplicito. La ricerca è stata ricevuta e letta: vedere il registro seguente. Non è ancora un pacchetto operativo validato.

## Registro della ricerca ricevuta

- Documento: `pesca-lazio-scientifica.pdf`, titolo "Pesca da riva nel Lazio: vademecum critico".
- Versione dichiarata: 1.0, 24 novembre 2026; 23 pagine fisiche, numerazione interna fino a 22.
- SHA-256: `2b85773bde88a278ff9df8cd7bb64fb4fdbeef5322fc7ccfae71838653c5ecc4`.
- Tipo: revisione narrativa critica, non sperimentazione locale o revisione sistematica preregistrata. Bibliografia di 38 voci; distingue testi integrali, abstract e testimonianze.
- Lettura del testo completata; ispezione visiva di campione delle montature C ed E effettuata. Fonti esterne citate dal PDF non ancora riverificate autonomamente.
- Originale e testo integrale rimangono esterni al repository; nessun percorso personale va registrato nei file pubblici.

### Requisiti ricavati dalla ricerca

- Dare priorità a habitat coerente e presentazione controllabile. Nessuna equivalenza tra mare più forte e più catture o tra luna e successo.
- Preservare le etichette: solida, trasferibile con cautela, operativa, non stabilita, con contesto e limiti.
- Distinguere monitoraggi balneari, stato chimico/ecologico, accesso e idoneità alimentare. Nessuna certificazione alimentare dello spot.
- La selezione iniziale della ricerca esclude Fiumicino/sistema di foce del Tevere e riduce la priorità di altri settori secondo il proprio profilo prudenziale. Registrare queste scelte come ambito/politica del pacchetto, non come preferenze personali permanenti dedotte dal PDF.
- Coordinate anomale Pomezia A002: impedire l'uso operativo della geometria fino a risoluzione cartografica documentata. Il PDF non fornisce postazioni precise verificate.
- Calendario, fasce orarie, tempi di recupero esca e scaduta sono suggerimenti contestuali. Non inventare soglie numeriche universali di onde, corrente o distanza dalle foci.
- Il diario registra numero di ami, tempo con esca efficace, problemi e rilasci; separa catture e trattenimenti. Indicatori descrittivi: catture/ora-canna e massa trattenuta/ora-canna; la conformità del trattenimento resta distinta quando la normativa non è verificata.
- Normative e dati ambientali riportati sono riferiti alla ricerca: verifica delle fonti primarie prima dell'uso come informazioni attuali.

### Catalogo da trascrivere e validare

I valori seguenti sono riferimenti iniziali operativi del documento, non limiti strutturali dell'app. Conservare nel pacchetto anche componenti, diametri, ami, uso, correzioni e cautele presenti nelle sezioni 10–14.

| ID | Montatura | Dati di controllo della trascrizione |
| --- | --- | --- |
| A | Bracciolo basso singolo | Trave 100 cm, snodo 15 cm dal piombo, bracciolo 120 cm, piombo iniziale 60 g; variante 80 cm per grovigli |
| B | Piombo scorrevole | Terminale 150 cm, piombo 80–120 g, variante 100 cm; passaggio libero da verificare, senza fermo che imprigioni il piombo |
| C | Due braccioli corti | Trave 180 cm; snodi a 40/120 cm, braccioli 30/35 cm; piombo iniziale 125 g, margine geometrico 15 cm |
| D | Bracciolo alto singolo | Trave 150 cm, snodo 120 cm, bracciolo 100 cm; variante 70–80 cm, clip da riregolare |
| E | Bolognese, spallinata aperta | Terminale 80 cm, galleggiante 1,5 g; gruppi 0,50/0,30/0,20/0,15/0,10/0,05 g a 140/110/80/55/30/10 cm dalla giunzione; somma 1,30 g |
| F | Bolognese, peso concentrato | Terminale 60 cm, galleggiante 3 g; torpille 2,20 g a 80 cm, gruppi 0,25/0,15 g a 45/15 cm; somma 2,60 g |

Le quote hanno origini diverse: attacco piombo per fondo, giunzione terminale per pallini da bolognese. La taratura E/F si verifica con tutto il complesso in acqua: la somma nominale non deve coincidere forzatamente con la portata stampata. Verificare massa totale lanciata e resistenze reali dei componenti, senza certificare sicurezza dal solo diametro.

## Obiettivo operativo aggiornato

Decisione utente del 27 novembre 2026: il software recupera le condizioni dai servizi e propone quando, dove e come pescare confrontando giorni e zone disponibili. L'utente configura disponibilità, inventario, preferenze e area una volta e può aggiornarli; non deve trascrivere coordinate dei settori, vento, onde o altri dati già disponibili attraverso i servizi. Le osservazioni sul posto e la simulazione restano percorsi distinti. Avviare una pianificazione può avviare il recupero, con informazione sulle coordinate trasmesse e impostazioni persistenti delle connessioni; nessun account applicativo o server obbligatorio.

La proposta mostra giorno/finestra, zona sulla mappa, tecnica, montatura, esca, motivazioni, alternative, dati mancanti e controlli all'arrivo. Le disponibilità oltre l'orizzonte reale dei provider restano pianificabili con assenza esplicita di previsioni. L'ordinamento applica regole documentate e vincoli personali, senza percentuali di cattura o soglie inventate. Il satellite è un livello cartografico con fonte e diritti verificati, non una prova di habitat, accessibilità o pescosità.

## Requisiti e criteri per fase

Questa sezione definisce il risultato richiesto, non l'ordine operativo o lo stato del lavoro. L'unico backlog è [roadmap/TODO.md](../roadmap/TODO.md). Le attività iniziali includono verifica e completamento del prototipo esistente; non sono dichiarate Done per la sola presenza di codice. La priorità è il percorso completo di pianificazione, poi il completamento del diario e della conservazione. Nessuna fase completa viene dedotta dal numero di attività concluse.

### T0 — Repository e fondamenta

Repository pubblico revisionato, esclusioni, identità Git generica, licenza e attribuzioni, dati sempre esterni al checkout. Toolchain e lockfile riproducibili; nucleo Rust indipendente da desktop/rete/database e contratti TypeScript generati. Comandi applicativi e permessi Tauri limitati; nessun filesystem o SQL generico dal frontend. CI con build Linux, TypeScript, test, formattazione, Clippy, privacy e controllo roadmap.

Accettazione: finestra Tauri reale su Linux, migrazioni e dato sintetico riletto dopo riavvio; mappa MapLibre offline e PMTiles locale limitato alle risorse autorizzate. La verifica browser non sostituisce la WebView.

Copertura nel backlog: GOV-001, CORE-001–003, MAP-001, REL-001–002. Le verifiche privacy sono trasversali e richieste prima di pubblicare.

### T1 — Contenuti e geografia

Pacchetti versionati con affermazioni, fonti, evidenze, parametri, geometrie, cataloghi e regole dichiarative; validazione transazionale, anteprima dei cambiamenti e compatibilità delle capacità. Montature A–F complete di misure, varianti e cautele, con disegni originali guidati dai dati e revisione dei diritti. Geometrie di regioni/settori/località/punti o tratti verificate; centri rappresentativi e punti candidati distinti dalle postazioni verificate. A002 sospeso finché irrisolto.

Accettazione: navigazione Italia → regione → settore → località → punto/tratto con Lazio iniziale, altre regioni indisponibili, breadcrumb, animazioni interrompibili e movimento ridotto. Schede e mappa si selezionano reciprocamente e mostrano precisione, provenienza, limiti e revisione. Base e attribuzioni offline; satellite/ortofoto con fonte, data disponibile e diritti verificati. Raccomandazioni geografiche solo nell'ambito verificato.

Copertura nel backlog: SRC-001, GEO-001–003, MAP-001–003, SCI-001–004, ACCEPT-001.

### T2 — Diario offline

Uscite, canne, intervalli, assetti, pause, osservazioni, catture, esiti, fotografie e partecipanti facoltativi. Posizioni da catalogo, mappa, coordinate o geolocalizzazione, con origine e accuratezza; postazione distinta da zona di lancio. Identificazione incerta e misure stimate/misurate, rilascio e trattenimento, numero ami e parametri aggiuntivi con definizioni versionate. Nuove misure non rendono incompleti i vecchi dati.

Accettazione: una stessa canna non ha intervalli sovrapposti, due canne possono essere contemporanee e il cambio assetto è atomico. Catture, cappotto esplicito e registrazione incompleta restano distinti; ore-canna nominali ed effettive non si confondono, e mancanze non diventano zero. Un'uscita con pausa, cambio esca e foto si conserva dopo riavvio senza rete; gli indicatori descrittivi richiedono dati sufficienti.

Copertura nel backlog: DIARY-001–004, PLAN-005–006, DATA-001–004, REL-001.

### T3 — Ambiente e simulazione

Adattatori separati Open-Meteo meteo/mare con termini e attribuzioni verificati, HTTP sostituibile nei test, timeout, cancellazione, dati parziali e cache persistente. Contratto normalizzato con unità, direzioni, coordinate richieste/restituite, validità, recupero ed emissione se disponibile; orizzonti separati per variabile. Recupero coordinato per le zone selezionate, senza trascrizione manuale e senza invio implicito di posizioni personali. Sole e fase lunare locali con libreria verificata; nessun beneficio di cattura attribuito alla sola luna.

Accettazione: provider simulati provano timeout, errori HTTP, campi mancanti e copertura parziale; una prova live distinta conferma l'integrazione. Offline la cache mostra la data di recupero; oltre l'orizzonte non viene inventato meteo. Osservazione, previsione e simulazione restano distinte fino allo snapshot.

Copertura nel backlog: SRC-001, ENV-001–005, PLAN-002, ACCEPT-001.

### T4 — Pianificatore e adattamento

Disponibilità di giorni/fasce, zone/raggio, inventario, specie quando supportata dalla ricerca, preferenze e scelte bloccate sono persistenti. Il software genera candidati multi-giorno/multi-zona usando le condizioni recuperate, poi applica regole Rust deterministiche. Compatibilità non dimostrabile resta da verificare, inclusi peso totale lanciato e resistenze dei componenti. Non sono previsti tempi stradali. Risultati motivati e alternativi con dati mancanti; nessuna classifica numerica priva di base documentata.

Accettazione: con configurazione salvata, una pianificazione propone giorno, fascia, zona, tecnica, montatura ed esca senza digitare condizioni o coordinate. Se non ci sono candidati, spiega vincoli e mancanze. Salva snapshot immutabili di input, condizioni, geometrie, cataloghi, motore e risultati; crea uscita collegata e registra consiglio seguito. Correzioni sul posto per esca, grovigli, deriva, sottotaglia e passata producono nuova valutazione con differenze motivate, senza trasformare ipotesi in cause certe.

Copertura nel backlog: CORE-002–003, SCI-002–003, PLAN-001–006, ACCEPT-001.

### T5 — Conservazione e release Linux

Esportazioni JSON complete, CSV per analisi e GeoJSON con definizioni e riferimenti storici. Backup versionato con database coerente via VACUUM INTO, allegati, manifest e checksum; mutazioni sospese durante la preparazione. Ripristino in area temporanea con controlli di integrità, schema, riferimenti e percorsi; sostituzione controllata e copia di recupero precedente.

Accettazione: file corrotti, versioni incompatibili e riferimenti invalidi non danneggiano dati correnti. Installazione pulita ricostruisce diario, fotografie e piani anche dopo aggiornamento dei contenuti. Pacchetto Debian installato e provato, verifica privacy di repository e artefatti, release con checksum, istruzioni e limiti documentati.

Copertura nel backlog: DATA-001–004, REL-001–003.

## Verifica trasversale

I criteri comprendono doppia canna e cambio dell'ora, assenza/zero/stima, misure A–F e origini delle quote, geometrie sospese, fonti e diritti, aggiornamento con nuovo parametro senza colonna, unità incompatibile e capacità sconosciuta, storico immutato dopo aggiornamento e ripristino, rete assente e flusso desktop piano → uscita → diario. Ogni Done nel backlog registra prove reali e limiti; la validazione strutturale del TODO non prova il funzionamento dell'app.
