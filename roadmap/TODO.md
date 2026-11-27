# Sanpei — backlog unico

## Working contract

Questo è l'unico elenco operativo. Ordine editoriale = priorità fra attività pronte; le dipendenze definiscono la fattibilità. Le fasi T0–T5 in `docs/implementation-plan.md` sono requisiti di riferimento, non un secondo backlog. Prima consegna: Sanpei recupera le condizioni per le zone documentate, confronta i giorni disponibili e propone quando, dove e come pescare, spiegando limiti e alternative.

Non confondere codice esistente con attività completata: le attività iniziali includono completamento e verifica del prototipo. Ogni attività deve stare in una sessione; se non ci sta, dividerla in nuovi ID prima di iniziarla. Non copiare nomi, dati o contenuti personali dall'esempio esterno. Owner identifica il ruolo dell'agente, senza dati personali.

```markdown
- [ ] **Not started** **AREA-001** Titolo concreto.
  Depends on: none.
  Responsible: Agent
  Owner: unassigned
  Scope: Confini, requisiti di riferimento ed esclusioni.
  Completion: Risultato osservabile e verificabile.

- [ ] **In progress** **AREA-002** Titolo concreto.
  Depends on: `AREA-001`.
  Responsible: Agent
  Owner: Codex
  Scope: Un'attività verificabile in una sessione.
  Completion: Risultato dimostrabile.
  Note: (owner: Codex, attività corrente)

- [x] **Done** **AREA-003** Titolo concreto.
  Depends on: `AREA-002`.
  Responsible: Agent
  Owner: Codex
  Scope: Confini dell'attività conclusa.
  Completion: Criteri interamente soddisfatti.
  Verification: 2026-11-27 UTC comandi realmente eseguiti, prove e limiti.

- **OLD-001** Retired. Reason: Attività sostituita. Replacement: `AREA-001`.
```

Campi su righe singole; riga vuota = fine attività. Stati ammessi solo Not started / In progress / Done; Done richiede checkbox x, Owner assegnato e Verification con data UTC valida, prove e limiti. In progress richiede Owner e Note corrispondente. Attività avviate o concluse richiedono dipendenze Done. Responsible è Agent oppure Owner. Gli ID sono permanenti e non si riutilizzano. Il grafo è generato, mai modificato a mano. I controlli strutturali non certificano la veridicità delle prove: resta necessaria la revisione.

## Fondamenta e contratti per il percorso principale

- [x] **Done** **GOV-001** Attivare backlog unico, validatore, grafo e workflow.
  Depends on: none.
  Responsible: Agent
  Owner: Codex
  Scope: Implementare questo sistema secondo l'esempio adattato; collegare istruzioni, hook esistenti e CI preservando i controlli privacy; distinguere requisiti, fatti e lavoro.
  Completion: Validatore e test stdlib passano; grafo riproducibile; ready e layers corretti; tutti i requisiti T0–T5 coperti; nessun secondo backlog operativo.
  Verification: 2026-11-27 UTC python3 scripts/check_roadmap.py --write-graph; python3 scripts/check_roadmap.py; python3 scripts/check_roadmap.py --ready; python3 scripts/check_roadmap.py --layers; python3 -m unittest discover -s tests -v: 14 test passati, inclusi CLI/idempotenza/mutazioni; sh scripts/hooks/pre-commit e pnpm privacy passati; revisione manuale della copertura T0–T5. Solo workflow locale verificato, non integrazioni live o app desktop; CI configurata, esecuzione remota non attestata.

- [ ] **Not started** **CORE-001** Riprodurre avvio e persistenza nella WebView Linux.
  Depends on: `GOV-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T0; provare checkout attuale con dati sintetici, migrazioni, mappa regionale, riavvio e dato riletto; registrare ambiente generico e limiti senza percorsi personali.
  Completion: Prove dell'app Tauri reale e della persistenza disponibili; eventuali difetti bloccanti risolti o suddivisi in attività prima di procedere; prova browser separata.

- [ ] **Not started** **CORE-002** Separare valutazione pura da comandi e persistenza.
  Depends on: `CORE-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Architettura §§1,3; estrarre il nucleo esistente in modulo o crate senza Tauri, SQLx e rete, mantenendo API e snapshot compatibili.
  Completion: Test di regole e determinismo eseguibili senza desktop o database; integrazione esistente non regressa; confini documentati.

- [ ] **Not started** **CORE-003** Generare i contratti frontend dai tipi Rust.
  Depends on: `CORE-002`.
  Responsible: Agent
  Owner: unassigned
  Scope: Architettura §1 e specifica T0; contratti versionati di geometrie, condizioni, valutazioni ed errori; comandi applicativi e capabilities ristrette, senza filesystem o SQL generico.
  Completion: Generazione riproducibile e controllo CI dei tipi obsoleti; build frontend e Rust passano; vecchi piani leggibili; frontend senza tipi duplicati dei contratti migrati.

- [ ] **Not started** **SRC-001** Verificare fonti e accesso ai servizi ambientali e cartografici.
  Depends on: `CORE-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T1/T3; consultare fonti primarie Open-Meteo meteo/mare, base cartografica e immagini satellitari; registrare URL, termini, licenze, attribuzioni, limiti di accesso/cache e data; nessun abbonamento o segreto pubblico.
  Completion: Matrice dei servizi con decisione motivata, configurazione necessaria e fallback; eventuale impedimento al satellite dichiarato; nessun orizzonte o diritto assunto senza fonte.

## Geografia e mappa utilizzabile

- [ ] **Not started** **GEO-001** Versionare il contratto geografico dei pacchetti.
  Depends on: `CORE-003`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T1 e architettura §2; regione, settore, località, punto/tratto, geometria e punto di richiesta ambientale distinti; fonti, revisione, accuratezza e stato verificato/candidato/centro rappresentativo/sospeso.
  Completion: Schema validato con fixture valide e invalide; importazione atomica; versioni precedenti ancora leggibili; coordinate A002 sospese; riferimenti stabili e nessun punto inventato.

- [ ] **Not started** **GEO-002** Verificare cartograficamente i settori settentrionali della ricerca.
  Depends on: `GEO-001`, `SRC-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T1; Santa Marinella, Santa Severa, Ladispoli e Palo/Marina di San Nicola; verificare nomi, estensioni e geometrie con fonti e diritti, distinguendo centri rappresentativi e postazioni.
  Completion: Geometrie revisionate e riproducibili importabili; ogni elemento ha provenienza e precisione dichiarata; nessuna promessa di accessibilità o postazione ottimale.

- [ ] **Not started** **GEO-003** Verificare cartograficamente i settori meridionali della ricerca.
  Depends on: `GEO-001`, `SRC-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T1; settori del catalogo fra litorale romano meridionale e Anzio; verificare geometrie e ambito della ricerca; anomalia Pomezia A002 esclusa finché irrisolta, esclusioni editoriali distinte dai divieti.
  Completion: Catalogo geografico iniziale completo rispetto ai settori esistenti, fonti e limiti visibili; eventuali elementi irrisolti marcati sospesi e non usati nelle proposte operative.

- [ ] **Not started** **MAP-001** Mostrare una base costiera dettagliata con fallback offline.
  Depends on: `SRC-001`, `CORE-003`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T0/T1; integrare base autorizzata MapLibre, stile e attribuzioni; archivio PMTiles locale attraverso adattatore limitato ai file autorizzati; risorse locali dove necessarie.
  Completion: Costa e riferimenti locali leggibili nella WebView Linux; richieste di file non autorizzati respinte; senza rete nessuna schermata vuota e base offline attribuita; dimensioni asset documentate.

- [ ] **Not started** **MAP-002** Collegare zone, schede e selezione sulla mappa.
  Depends on: `MAP-001`, `GEO-002`, `GEO-003`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T1; overlay di settori/tratti e marker con etichette di precisione; selezione bidirezionale scheda-mappa, zoom e breadcrumb Italia/regione/settore/località; tastiera e movimento ridotto.
  Completion: Ogni settore pubblicato si trova dalla mappa e dalla scheda, con evidenziazione coerente; Lazio iniziale, altre regioni indisponibili; nessun marker scambiabile per postazione verificata.

- [ ] **Not started** **MAP-003** Aggiungere il livello satellitare o ortofoto autorizzato.
  Depends on: `MAP-002`, `SRC-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T1 e richiesta utente; integrare la fonte scelta, commutazione base/immagini e overlay leggibili; fonte, data disponibile e attribuzioni; credenziali eventualmente locali.
  Completion: Immagini visibili nella WebView con attribuzione e comportamento offline esplicito; nessun download/cache oltre i diritti; indisponibilità del livello non blocca zone o pianificatore.

## Condizioni recuperate dal software

- [ ] **Not started** **ENV-001** Definire e testare il contratto ambientale normalizzato.
  Depends on: `CORE-003`, `SRC-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T3; variabili con unità e convenzioni direzionali, coordinate richieste/restituite, provider/modello disponibile, validità UTC, recupero, emissione se disponibile e orizzonti per variabile; distinguere previsione/osservazione/simulazione.
  Completion: Test di conversioni e direzioni, null e zero distinti, campi mancanti conservati; vecchia cache e snapshot trattati senza reinterpretazioni silenziose; nessun uso decisionale di JSON grezzo.

- [ ] **Not started** **ENV-002** Completare l'adattatore automatico Open-Meteo meteo.
  Depends on: `ENV-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T3; isolare HTTP dal dominio, richieste per coordinate catalogate e intervallo disponibile, timeout, stato HTTP, risposta parziale e normalizzazione; usare client sostituibile nei test.
  Completion: Test con risposte simulate per successo, timeout, errore HTTP, null e timestamp non validi; nessun inserimento manuale di vento o pioggia necessario per dati disponibili.

- [ ] **Not started** **ENV-003** Completare l'adattatore automatico Open-Meteo mare.
  Depends on: `ENV-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T3; onde, periodo, direzione e altre variabili realmente disponibili; verificare celle marine rispetto alla costa e conservare coordinate restituite e limiti; correnti non dedotte dal solo vento.
  Completion: Test per risposta completa, mare assente, copertura temporale diversa dal meteo e cella non utilizzabile; mancanze non trasformate in mare calmo o condizioni lavorabili.

- [ ] **Not started** **ENV-004** Coordinare recupero per più zone, cache e aggiornamento.
  Depends on: `ENV-002`, `ENV-003`, `GEO-002`, `GEO-003`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T3; un'azione di pianificazione recupera meteo/mare per le zone ammesse senza coordinate digitate; richieste limitate e cancellabili, riuso cache, date visibili e consenso persistente alle connessioni configurabile.
  Completion: Test su più zone con un provider indisponibile; offline usa cache datata senza presentarla come attuale; nessuna perdita delle serie valide in aggiornamenti parziali; nessuna posizione personale trasmessa implicitamente.

- [ ] **Not started** **ENV-005** Calcolare sole e fase lunare localmente.
  Depends on: `ENV-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T3; selezionare libreria e verificarne fonte/licenza, alba/tramonto e fase lunare con validità e precisione dichiarate; UTC e presentazione Europe/Rome.
  Completion: Casi di riferimento indipendenti e cambio dell'ora verificati; risultati offline; fase lunare non trasformata in probabilità o beneficio di cattura.

## Contenuti e regole con provenienza

- [ ] **Not started** **SCI-001** Revisionare catalogo e misure delle sei montature.
  Depends on: `CORE-002`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T1 e tabella del catalogo; verificare sintesi, varianti, diametri, ami, quote A–F e schemi originali; associare localizzazione della fonte, etichetta di evidenza e diritti della rielaborazione.
  Completion: Checklist A–F revisionata, origini delle quote e somme E/F corrette; nessuna configurazione dichiarata ottima o sicurezza certificata dal solo diametro.

- [ ] **Not started** **SCI-002** Collegare parametri ambientali a regole documentate.
  Depends on: `SCI-001`, `ENV-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T1/T4; trascrivere relazioni documentate fra habitat, stagione/fascia oraria, specie quando supportata, tecnica e condizioni; separare evidenze, suggerimenti, limiti e politiche editoriali; nessuna soglia numerica inventata.
  Completion: Ogni regola operativa ha fonte e dominio validato; parametri mancanti o relazioni non dimostrabili producono verifica necessaria; la mappatura automatica non fabbrica categorie calmo/lavorabile.

- [ ] **Not started** **SCI-003** Completare anteprima e compatibilità degli aggiornamenti di ricerca.
  Depends on: `SCI-002`, `GEO-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T1, architettura §§2–5; validazione transazionale e anteprima differenze, tipi e unità, operatori supportati e capacità; pacchetti regionali espliciti senza sblocco automatico.
  Completion: Test di pacchetto invalido senza importazione parziale, nuovo parametro senza colonna, capacità sconosciuta respinta, nuova ricerca attiva senza modificare gli storici.

- [ ] **Not started** **SCI-004** Presentare riferimenti ambientali e normativi con validità verificata.
  Depends on: `SCI-001`, `GEO-003`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica attendibilità/T1; verificare fonti primarie dei riferimenti pubblicati, distinguere monitoraggi balneari, stato ambientale, accesso, liceità e idoneità alimentare; automatizzare soltanto fonti con servizio e diritti effettivi documentati.
  Completion: Ogni avviso pubblicato ha fonte, data e ambito; validità sconosciuta esplicita; nessuna promessa di monitoraggio in tempo reale o certificazione del pescato.

## Quando, dove e come: prima consegna utile

- [ ] **Not started** **PLAN-001** Salvare inventario e preferenze riutilizzabili.
  Depends on: `CORE-003`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T4; configurazione locale di canne, componenti/esche disponibili, limiti dichiarati, zone/raggio, disponibilità di giorni e orari e obiettivi; nessuna stima stradale o account obbligatorio.
  Completion: Dopo riavvio il pianificatore riusa i dati senza reinserimento; preferenze modificabili e scelte bloccabili; assenza di inventario non autorizza compatibilità positiva.

- [ ] **Not started** **PLAN-002** Generare finestre candidate per giorni e zone disponibili.
  Depends on: `ENV-004`, `ENV-005`, `SCI-002`, `PLAN-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T4; unire disponibilità personali, aree ammesse, condizioni normalizzate e finestre documentate; dati automatici per ogni candidato, orizzonti reali per variabile, informazioni assenti esplicite.
  Completion: Test deterministici multi-giorno/multi-zona con copertura parziale, cambio ora e data oltre previsione; nessuna previsione inventata; simulazione e osservazione sono percorsi facoltativi distinti.

- [ ] **Not started** **PLAN-003** Valutare e ordinare proposte spiegabili con vincoli reali.
  Depends on: `PLAN-002`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T4 e architettura §3; valutazione Rust di luogo/finestra/tecnica/montatura/esca e distanza solo se documentata; peso totale lanciato e componenti; ordinamento per vincoli, preferenze e regole versionate.
  Completion: Test di scelte bloccate, incompatibilità, dati mancanti e parità stabili; niente percentuali di cattura; assenza di candidati spiegata; sole previsioni insufficienti non diventano raccomandazione certa.

- [ ] **Not started** **PLAN-004** Costruire il flusso guidato di proposta automatica sulla mappa.
  Depends on: `PLAN-003`, `MAP-002`, `MAP-003`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T4 e richiesta utente; azione confronta prossime disponibilità, recupero servizi senza trascrizione manuale, selezione coerente mappa/risultati; giorno, fascia, zona, tecnica, montatura, esca, motivi e alternative.
  Completion: Nella WebView un utente con configurazione salvata ottiene proposte senza digitare coordinate o condizioni; errori e cancellazione comprensibili; risultati mostrano copertura, freschezza e controlli all'arrivo.

- [ ] **Not started** **PLAN-005** Salvare e riaprire snapshot completi delle proposte automatiche.
  Depends on: `PLAN-004`, `SCI-003`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T4; conservare candidati valutati, risultato, geometrie, parametri, provenienza/provider, condizioni e versioni; compatibilità con piani esistenti e creazione uscita collegata.
  Completion: Nuovo meteo o nuova ricerca non cambiano il piano salvato; vecchio snapshot leggibile offline; percorso proposta → piano → uscita mantiene zona e motivazioni originarie.

- [ ] **Not started** **ACCEPT-001** Dimostrare la prima versione utile di pianificazione su Linux.
  Depends on: `PLAN-005`.
  Responsible: Agent
  Owner: unassigned
  Scope: Verifica verticale T0/T1/T3/T4; avvio reale, preferenze salvate, zone e satellite, recupero servizi, confronto giorni, proposta e salvataggio/riavvio; fixture sintetiche per test e nessun dato personale in prove pubbliche.
  Completion: Checklist riproducibile nella WebView e prova live dei servizi documentate separatamente dai provider simulati; senza rete cache datata o indisponibilità esplicita; nessuna dichiarazione di release completa.

## Diario, conservazione e completamento della v1

- [ ] **Not started** **DIARY-001** Completare modifiche e cambio atomico di assetto.
  Depends on: `ACCEPT-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T2; modifica di canne/intervalli/catture/osservazioni, cambio assetto chiude e apre intervalli in transazione, pause e contemporaneità conservate.
  Completion: Test SQL di sovrapposizione rifiutata e due canne ammesse, rollback del cambio invalido e persistenza dopo riavvio; nessuna cancellazione necessaria per correggere un record.

- [ ] **Not started** **DIARY-002** Completare misure, partecipanti e parametri aggiuntivi del diario.
  Depends on: `DIARY-001`, `SCI-003`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T2; numero ami, identificazione incerta, lunghezza/peso stimati o misurati, rilascio/trattenimento, partecipanti facoltativi e valori aggiuntivi con definizioni versionate.
  Completion: Test su assenza/zero/stima; un nuovo parametro non richiede una colonna né rende incompleti vecchi record; definizioni storiche leggibili e dati locali.

- [ ] **Not started** **DIARY-003** Gestire fotografie e ciclo di vita degli allegati locali.
  Depends on: `DIARY-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T2; importazione controllata nella directory applicativa, metadata e riferimenti stabili, limiti e anteprime; mai percorsi arbitrari dal frontend o file reali nel repository.
  Completion: Test di importazione/rimozione, file mancante o non valido, eliminazione uscita e coerenza database/file anche su errore; riavvio conserva una fotografia sintetica.

- [ ] **Not started** **DIARY-004** Completare sforzo, esiti e riepiloghi descrittivi.
  Depends on: `DIARY-002`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T2 e verifiche trasversali; ore-canna nominali/efficaci, pause, catture/cappotto/registrazione incompleta distinti; indicatori descrittivi solo con denominatori e misure sufficienti.
  Completion: Test doppia canna, pausa, cambio ora e dettagli mancanti; nessuna equivalenza tra assenza catture registrate e cappotto, né inferenza causale dalle statistiche.

- [ ] **Not started** **PLAN-006** Rivalutare sul posto con diagnosi motivate e confronto storico.
  Depends on: `ACCEPT-001`, `DIARY-002`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T4; osservazioni su esca, grovigli, deriva, sottotaglia e passata; trasformazione esplicita in input supportati, nuova valutazione collegata e consiglio seguito annotabile.
  Completion: Test di correzioni e dati ignoti; confronto motivato con originale immutato; osservazioni non trasformate in previsioni né cause certe di cattura.

- [ ] **Not started** **DATA-001** Versionare esportazioni complete con definizioni e allegati.
  Depends on: `DIARY-003`, `DIARY-004`, `PLAN-006`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T5; completare JSON, CSV e GeoJSON con nuovi campi, riferimenti agli allegati, definizioni/versioni e storici; documentare contenuto di ogni formato e protezione delle formule.
  Completion: Test su dati sintetici completi, null distinti da zero, coordinate e definizioni ricostruibili, testo CSV sicuro e destinazione esistente non sovrascritta.

- [ ] **Not started** **DATA-002** Creare backup coerente con allegati e checksum.
  Depends on: `DATA-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T5; estendere snapshot VACUUM INTO a contenitore versionato con manifest, checksum, allegati e pacchetti; bloccare mutazioni durante preparazione, pubblicazione atomica e non sovrascrivente.
  Completion: Test con scrittura concorrente, file mancante e backup interrotto; archivio verifica checksum e riferimenti; database corrente e vecchi backup invariati su errore.

- [ ] **Not started** **DATA-003** Ripristinare il backup completo senza perdita dei dati correnti.
  Depends on: `DATA-002`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T5; area temporanea, limiti di estrazione, controllo percorsi, integrità/riferimenti/schema, sostituzione controllata e copia precedente; mantenere lettura dei backup SQLite supportati.
  Completion: Test di file corrotto, checksum errato, versione futura, allegato assente e percorso malevolo; errori lasciano dati correnti intatti; successo ricostruisce piani e foto.

- [ ] **Not started** **DATA-004** Verificare aggiornamenti, vecchi piani e installazione pulita.
  Depends on: `DATA-003`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T5 e architettura §6; migrare database precedente, aggiornare ricerca con nuovo parametro, consultare storico e ripristinare tutto in directory applicativa pulita.
  Completion: Prove riproducibili con diario/foto/pacchetti/piani conservati e versioni incompatibili respinte; nessuna dipendenza dai cataloghi correnti per leggere uno storico.

- [ ] **Not started** **REL-001** Verificare accessibilità e percorso completo nella WebView Linux.
  Depends on: `DATA-004`, `SCI-004`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T0–T5; tastiera, movimento ridotto, errore rete/provider parziale, mappa offline, selezione posizione, piano → uscita a due canne → pausa/cambio → foto → backup/ripristino → riavvio.
  Completion: Checklist desktop reale eseguita con evidenze sintetiche; nessun passaggio simulato dichiarato completo; difetti aperti registrati e risolti prima della release.

- [ ] **Not started** **REL-002** Costruire e verificare il pacchetto Debian pubblicabile.
  Depends on: `REL-001`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T5; toolchain/lockfile e CI aggiornate, build deb, installazione pulita, avvio e migrazione; revisione privacy di repository e artefatti, licenze e istruzioni d'uso concrete.
  Completion: Build, test, Clippy, formattazione e privacy passano; pacchetto installato e provato con dati sintetici; checksum, limiti e attribuzioni disponibili senza segreti o dati personali.

- [ ] **Not started** **REL-003** Pubblicare la release Linux verificata.
  Depends on: `REL-002`.
  Responsible: Agent
  Owner: unassigned
  Scope: Specifica T5; pubblicare su repository autorizzato solo gli artefatti revisionati e note coerenti con prove e limiti, collegando il tag al commit verificato.
  Completion: Release e pacchetto scaricabili con checksum e istruzioni; prova del pacchetto pubblicato documentata; nessuna funzionalità incompleta presentata come disponibile.

## Retired IDs

Nessun ID ritirato.
