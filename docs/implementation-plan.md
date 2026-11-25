# Piano operativo Sanpei

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

## Attività ordinate

### T0 — Repository e fondamenta

1. Predisporre esclusioni, licenza MIT del codice, attribuzioni, README, identità Git locale generica e controllo privacy su file e cronologia. Dati applicativi sempre fuori dal checkout.
2. Ripristinare l'accesso GitHub; verificare il repository `sanpei` nell'account autenticato, creare pubblico se libero, ispezionare se esistente senza sovrascrivere. Nessun push prima della revisione privacy.
3. Fissare versioni compatibili di toolchain e dipendenze, generare lockfile; introdurre nucleo Rust indipendente e comandi applicativi tipizzati.
4. Provare finestra Tauri, migrazione SQLite, persistenza dopo riavvio e lettura offline MapLibre/PMTiles limitata alle risorse autorizzate.
5. Configurare controlli TypeScript, formattazione, Clippy, test e build Linux in CI. Documentare i comandi effettivi.

Uscita: finestra reale e mappa offline funzionanti su Linux; dato sintetico persistente dopo riavvio. Non costruire tutte le schermate prima di risolvere questa prova.

### T1 — Pacchetto Lazio e geografia

Dipende da T0. Definire schemi e registro parametri prima di popolare dati.

1. Estrarre affermazioni e fonti dalla ricerca, assegnare identificativi e provenienza, distinguere evidenza e suggerimento operativo.
2. Trascrivere integralmente montature A–F e varianti, con controllo misure e unità. Verificare diritti dei materiali da distribuire.
3. Implementare validazione e attivazione transazionale, conservazione delle versioni, controllo capacità e anteprima dei cambiamenti.
4. Verificare geometrie di settori/località/punti con fonti cartografiche; tenere sospesa l'anomalia A002. Non sostituire postazioni mancanti con marker imprecisi non etichettati.
5. Realizzare Italia → regione → settore → località → punto/tratto, Lazio selezionato, altre regioni indisponibili, breadcrumb e animazioni interrompibili con movimento ridotto.
6. Includere base, stile, caratteri e attribuzioni offline con licenze verificate. Immagini aeree solo con provenienza e diritti disponibili.

Uscita: catalogo reale revisionato e navigazione con precisione geografica dichiarata. Raccomandazioni geografiche reali solo nei tratti verificati; elementi sospesi visibili come tali.

### T2 — Diario offline

Dipende da T0 e contratti del dominio; il completamento geografico di T1 non impedisce posizioni personali.

1. Uscite, canne, intervalli con assetto, pause, osservazioni, catture, esiti e allegati locali.
2. Posizioni da catalogo, mappa, coordinate e geolocalizzazione disponibile; origine, accuratezza e zona di lancio separate.
3. Vincoli sugli intervalli della stessa canna, cambi assetto atomici e due canne contemporanee.
4. Misure stimate/misurate, identificazioni incerte, rilasci, trattenimenti, numero ami e sforzo nominale/efficace distinguibili.
5. Campi aggiuntivi validati con definizioni versionate; una futura misura non rende incompleti i vecchi record.

Uscita: uscita a due canne con pausa e cambio esca, esito esplicito e allegato ricostruita dopo riavvio, senza rete.

### T3 — Ambiente e simulazione

Dipende dai contratti di T0/T1.

1. Adattatori Open-Meteo meteo e mare, richieste esplicite, informazione sulle coordinate trasmesse, timeout e cache persistente.
2. Unità e direzioni normalizzate; provenienza, coordinate restituite, validità, recupero e orizzonti separati per variabile.
3. Astronomia locale con libreria verificata; nessun punteggio dominante sulla luna.
4. Dati modellistici, osservati e simulati distinguibili; nessun meteo inventato oltre l'orizzonte disponibile.
5. Test su provider simulati per timeout, risposta parziale e assenza dati; attribuzioni e condizioni d'uso verificate.

Uscita: condizioni consultabili da cache offline con data visibile e pianificazione futura possibile anche senza previsione disponibile.

### T4 — Pianificatore e diagnosi

Dipende da T1 e T3; integrazione col diario dipende da T2.

1. Input con fascia oraria, zona/raggio, inventario, bersagli, preferenze e scelte bloccate. Nessuna stima di viaggio stradale.
2. Regole Rust deterministiche e spiegabili con compatibilità documentata, conoscenza insufficiente esplicita, alternative e controlli all'arrivo.
3. Applicare le correzioni della ricerca a esca assente, grovigli, deriva, sottotaglia e passata instabile; non trasformare ipotesi diagnostiche in cause certe.
4. Salvare snapshot immutabili con valori effettivi e versioni; rivalutare creando una revisione e mostrare le differenze.
5. Collegare piano e uscita e annotare se i consigli sono stati seguiti. Niente percentuali di cattura.

Uscita: scenari riproducibili, vincoli rispettati, vecchio piano invariato dopo nuova ricerca e adattamento sul posto.

### T5 — Conservazione e release Linux

Dipende da T1–T4.

1. Esportazioni JSON complete, CSV per analisi e GeoJSON; includere definizioni dei parametri e riferimenti storici necessari.
2. Backup database coerente con `VACUUM INTO`, allegati, pacchetti referenziati e manifest/checksum; sospendere modifiche durante la preparazione.
3. Ripristino in area temporanea, verifica integrità/schema/riferimenti, sostituzione controllata e copia di recupero dei dati precedenti.
4. Provare migrazione di database precedente, importazione di una nuova ricerca e consultazione dei vecchi piani.
5. Build `.deb`, prova su installazione pulita, revisione privacy degli artefatti e pubblicazione della release verificata.

Uscita: dati e piani ricostruiti su installazione pulita con allegati e contenuti storici; nessuna perdita in ripristini falliti.

## Verifiche trasversali

- Intervalli e ore-canna: sovrapposizioni, pause, doppia canna, cambio ora, zero e dato mancante.
- Ricerca: misure A–F, origini delle quote, somme E/F, geometrie sospese, diritti e fonti.
- Aggiornamenti: nuovo parametro senza nuova colonna, regola revisionata, capacità sconosciuta, unità errata, mancata attivazione senza danni.
- Storico: snapshot invariati, definizioni precedenti leggibili, esportazione e ripristino dopo aggiornamenti.
- Desktop reale: rete assente, mappa, riavvio e percorso piano → uscita → diario.
- Privacy: file, commit, allegati, screenshot e log prima della pubblicazione.
